use std::collections::BTreeMap;

use shared::*;

fn fixture(starting_team: Team, pickup: Option<PowerUp>, enabled: bool) -> Game {
    let at = if starting_team == Team::Red {
        Position(1, 1)
    } else {
        Position(6, 6)
    };
    Game::new(
        &Level::new(
            Board::new(8, 8).unwrap(),
            vec![
                Mage::new(0, Team::Red, MageSort::Plus, Position(0, 0)),
                Mage::new(1, Team::Blue, MageSort::Plus, Position(7, 7)),
            ],
            pickup
                .map(|p| BTreeMap::from([(at, p)]))
                .unwrap_or_default(),
            starting_team,
        ),
        enabled,
    )
    .unwrap()
}

fn quiet_move(game: &mut Game) {
    let mage = game
        .iter_mages()
        .find(|m| m.team == game.turn_for())
        .unwrap();
    let from = mage.position;
    let left = if mage.team == Team::Red {
        if from.1 == 0 {
            0
        } else {
            1
        }
    } else if from.1 == 7 {
        6
    } else {
        5
    };
    let to = Position(if from.0 == left { left + 1 } else { left }, from.1);
    assert!(game.take_move(from, to).unwrap().is_empty());
}

#[test]
fn pickups_at_expiry_boundary_grant_exactly_eight_full_turns() {
    for team in [Team::Red, Team::Blue] {
        for power in [PowerUp::Diagonal, PowerUp::Shield, PowerUp::Beam] {
            let mut game = fixture(team, Some(power), true);
            for _ in 0..22 {
                quiet_move(&mut game);
            }
            assert_eq!(game.quiet_turns(), 7);
            assert!(game.result().is_none());
            let from = game.iter_mages().find(|m| m.team == team).unwrap().position;
            let to = if team == Team::Red {
                Position(1, 1)
            } else {
                Position(6, 6)
            };
            assert!(game.take_move(from, to).unwrap().is_empty());
            assert!(game.powerups().is_empty());
            assert_eq!(game.quiet_turns(), 0);
            assert!(game.result().is_none());
            assert_eq!(game.rewind(1).quiet_turns(), 7);
            let mut game: Game =
                serde_json::from_str(&serde_json::to_string(&game).unwrap()).unwrap();
            for quiet in 1..=16 {
                quiet_move(&mut game);
                assert_eq!(
                    game.quiet_turns(),
                    quiet / 2,
                    "team {team:?}, power {power:?}, quiet {quiet}"
                );
                assert_eq!(game.result().is_some(), quiet == 16);
            }
            assert!(game.result() == Some(GameResult::Stalemate));
            let rewound = game.rewind(1);
            assert_eq!(rewound.quiet_turns(), 7);
            assert!(rewound.result().is_none());
        }
    }
}

#[test]
fn opening_grace_and_disabled_inactivity_are_preserved() {
    for team in [Team::Red, Team::Blue] {
        let mut game = fixture(team, None, true);
        for ply in 1..=23usize {
            quiet_move(&mut game);
            assert_eq!(game.quiet_turns(), ply.saturating_sub(7) / 2);
            assert_eq!(game.result().is_some(), ply == 23);
        }
        let mut tutorial = fixture(team, None, false);
        for _ in 0..100 {
            quiet_move(&mut tutorial);
        }
        assert_eq!(tutorial.quiet_turns(), 0);
        assert!(tutorial.result().is_none());
    }
}
