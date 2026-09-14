use generate::analysis::{simulate, Outcome, RunConfig};
use shared::{campaign_catalogue, Team};

#[test]
fn starting_team_controls_the_first_search() {
    let config = RunConfig {
        max_plies: 1,
        ..Default::default()
    };
    for team in [Team::Red, Team::Blue] {
        let mut level = campaign_catalogue(false)[0].level();
        level.starting_team = team;
        let result = simulate(&level, true, &config, 0, 2, 0);
        assert_eq!(result.plies, 1);
        assert_eq!(result.red.searches, usize::from(team == Team::Red));
        assert_eq!(result.blue.searches, usize::from(team == Team::Blue));
        assert_eq!(result.outcome, Outcome::SafetyLimit);
    }
}

#[test]
fn legal_fallback_collects_beam_and_finishes_a_forced_game() {
    use shared::{Board, BoulderStyle, Level, Mage, MageSort, Mana, Position, PowerUp};
    let mut config = RunConfig {
        max_plies: 1,
        ..Default::default()
    };
    for profile in &mut config.profiles {
        profile.nodes = 0;
    }
    for team in [Team::Red, Team::Blue] {
        let attacker = Mage::new(0, team, MageSort::Diamond, Position(0, 0));
        let mut victim = Mage::new(1, team.enemy(), MageSort::Cross, Position(1, 1));
        victim.mana = Mana(1, 4);
        let level = Level::new(
            Board::new(3, 3).unwrap(),
            vec![attacker, victim],
            [
                (Position(1, 0), PowerUp::Beam),
                (Position(0, 1), PowerUp::Boulder(BoulderStyle::Rock)),
            ]
            .into(),
            team,
        );
        let result = simulate(&level, true, &config, 0, 2, 0);
        assert_eq!(
            result.outcome,
            if team == Team::Red {
                Outcome::Win
            } else {
                Outcome::Loss
            }
        );
        assert_eq!(result.plies, 1);
        assert_eq!(result.red.fallbacks + result.blue.fallbacks, 1);
    }
}

#[test]
fn paired_namespace_survives_scenario_changes_and_replays_authoritative_moves() {
    use shared::{Game, PowerUp};
    let config = RunConfig {
        seed_namespace: Some("original-scenario".into()),
        replays: true,
        ..Default::default()
    };
    let entries = campaign_catalogue(false);
    let level = entries
        .iter()
        .find(|e| e.id == "diagonals-i")
        .unwrap()
        .level();
    let record = simulate(&level, true, &config, 1, 1, 0);
    let mut changed = level.clone();
    changed.mages[0].mana.0 -= 1;
    assert_eq!(config.trial_seed(&level, 0), config.trial_seed(&changed, 0));
    assert_ne!(
        RunConfig::default().trial_seed(&level, 0),
        RunConfig::default().trial_seed(&changed, 0)
    );
    let mut game = Game::new(&level, true).unwrap();
    for step in &record.replay {
        assert_eq!(game.turn_for(), step.team);
        assert_eq!(game.powerups().get(&step.turn.1).copied(), step.pickup);
        assert_eq!(
            game.take_move(step.turn.0, step.turn.1).unwrap(),
            step.damage
        );
        let mut mana = [0, 0];
        for mage in game.iter_mages() {
            mana[usize::from(mage.team == Team::Blue)] += mage.mana.0 as u32;
        }
        assert_eq!(mana, step.mana);
    }
    assert_eq!(game.turns(), record.plies);
    assert!(game.result().is_some());
    assert_ne!(record.termination, "SafetyLimit");
    assert!(record
        .replay
        .iter()
        .any(|s| s.pickup == Some(PowerUp::Diagonal)));
}
