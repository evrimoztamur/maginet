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
