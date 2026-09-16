use std::collections::{BTreeMap, HashSet, VecDeque};

use shared::*;

fn duel() -> Level {
    Level::new(
        Board::new(3, 3).unwrap(),
        vec![
            Mage::new(0, Team::Red, MageSort::Diamond, Position(0, 0)),
            Mage::new(1, Team::Blue, MageSort::Knight, Position(2, 2)),
        ],
        BTreeMap::new(),
        Team::Red,
    )
}

fn corridors() -> Level {
    let squares = [
        Position(0, 0),
        Position(1, 0),
        Position(2, 1),
        Position(3, 1),
        Position(0, 4),
        Position(1, 4),
        Position(2, 5),
        Position(3, 5),
    ];
    let mut rocks = BTreeMap::new();
    for y in 0..6 {
        for x in 0..4 {
            if !squares.contains(&Position(x, y)) {
                rocks.insert(Position(x, y), PowerUp::Boulder(BoulderStyle::Rock));
            }
        }
    }
    Level::new(
        Board::new(4, 6).unwrap(),
        vec![
            Mage::new(0, Team::Red, MageSort::Plus, Position(0, 0)),
            Mage::new(1, Team::Blue, MageSort::Plus, Position(3, 1)),
            Mage::new(2, Team::Red, MageSort::Plus, Position(0, 4)),
            Mage::new(3, Team::Blue, MageSort::Plus, Position(3, 5)),
        ],
        rocks,
        Team::Red,
    )
}

// Exact independent joint-state oracle: authoritative movement, no inactivity or
// overcharge, stop at first damage. Before damage only locations/turn can change.
fn exact_contact(level: &Level) -> bool {
    let root = Game::new_with_overcharge(level, false, false).unwrap();
    let key = |g: &Game| {
        (
            g.turn_for(),
            g.iter_mages()
                .map(|m| (m.position.0, m.position.1))
                .collect::<Vec<_>>(),
        )
    };
    let mut seen = HashSet::from([key(&root)]);
    let mut queue = VecDeque::from([root]);
    while let Some(game) = queue.pop_front() {
        for turn in game.legal_turns() {
            let mut next = game.clone();
            if !next.take_move(turn.0, turn.1).unwrap().is_empty() {
                return true;
            }
            if seen.insert(key(&next)) {
                queue.push_back(next);
            }
        }
    }
    false
}

#[test]
fn parity_and_multi_mage_geometry_are_resolved() {
    for level in [duel(), corridors()] {
        assert_eq!(
            contact_reachability(&level, level.starting_team, false),
            ContactReachability::Impossible
        );
        assert!(!exact_contact(&level));
        assert_eq!(
            contact_reachability(&level, level.starting_team, true),
            ContactReachability::Possible
        );
        let game = Game::new(&level, true).unwrap();
        assert_eq!(game.overcharge_at(), Some(0));
        assert!(game.iter_mages().all(Mage::has_diagonals));
        let mut charged = level.clone();
        for mage in &mut charged.mages {
            mage.powerup = Some(PowerUp::Diagonal);
        }
        assert!(exact_contact(&charged));
        assert_eq!(
            Game::new_with_overcharge(&level, true, false)
                .unwrap()
                .overcharge_at(),
            None
        );
        assert_eq!(Game::new(&level, false).unwrap().overcharge_at(), None);
    }
}

#[test]
fn all_small_duel_proofs_agree_with_exact_joint_reachability() {
    let mut proofs = 0;
    for first in 0..9 {
        for second in 0..9 {
            if first == second {
                continue;
            }
            for red in 0..5 {
                for blue in 0..5 {
                    for team in [Team::Red, Team::Blue] {
                        let mut level = duel();
                        level.starting_team = team;
                        level.mages = vec![
                            Mage::new(0, Team::Red, red.into(), Position(first % 3, first / 3)),
                            Mage::new(1, Team::Blue, blue.into(), Position(second % 3, second / 3)),
                        ];
                        if contact_reachability(&level, team, false)
                            == ContactReachability::Impossible
                        {
                            proofs += 1;
                            assert!(!exact_contact(&level), "false proof: {}", level.as_code());
                        }
                    }
                }
            }
        }
    }
    assert!(proofs > 100);
}

#[test]
fn geometry_proofs_with_multiple_mages_and_sleepers_match_oracle() {
    let mut proofs = 0;
    for kind in 0..5 {
        for team in [Team::Red, Team::Blue] {
            for sleepers in [false, true] {
                let mut level = corridors();
                level.starting_team = team;
                for mage in &mut level.mages {
                    mage.sort = kind.into();
                    mage.spell = Spell::select(mage.sort);
                }
                if sleepers {
                    let rocks: Vec<_> = level.powerups.keys().copied().collect();
                    level.powerups.clear();
                    for position in rocks {
                        let mut sleeper =
                            Mage::new(level.mages.len(), Team::Red, MageSort::Plus, position);
                        sleeper.mana.0 = 0;
                        level.mages.push(sleeper);
                    }
                }
                if contact_reachability(&level, team, false) == ContactReachability::Impossible {
                    proofs += 1;
                    assert!(!exact_contact(&level));
                }
            }
        }
    }
    assert!(proofs >= 4);
}

#[test]
fn powerups_and_teammate_waits_prevent_false_claims() {
    for power in [PowerUp::Beam, PowerUp::Diagonal, PowerUp::Shield] {
        let mut level = duel();
        level.powerups.insert(Position(1, 0), power);
        assert_eq!(
            contact_reachability(&level, Team::Red, false),
            ContactReachability::Unsupported
        );
        assert_eq!(Game::new(&level, true).unwrap().overcharge_at(), None);
        level.powerups.clear();
        level.mages[0].powerup = Some(power);
        assert_eq!(
            contact_reachability(&level, Team::Red, false),
            ContactReachability::Unsupported
        );
    }
    let mut level = duel();
    level
        .mages
        .push(Mage::new(2, Team::Red, MageSort::Diamond, Position(0, 2)));
    assert_eq!(
        contact_reachability(&level, Team::Red, false),
        ContactReachability::Possible
    );
    assert_eq!(Game::new(&level, true).unwrap().overcharge_at(), None);
    let mut level = duel();
    level.starting_team = Team::Blue;
    assert_eq!(
        contact_reachability(&level, Team::Blue, false),
        ContactReachability::Possible
    );
}

#[test]
fn elimination_can_trigger_once_and_undo_reconstructs_it() {
    let mut level = duel();
    let mut victim = Mage::new(2, Team::Blue, MageSort::Plus, Position(1, 2));
    victim.mana.0 = 1;
    level.mages.push(victim);
    let mut game = Game::new(&level, true).unwrap();
    assert_eq!(game.overcharge_at(), None);
    assert_eq!(
        game.take_move(Position(0, 0), Position(1, 0)).unwrap(),
        vec![Position(1, 2)]
    );
    assert_eq!(game.overcharge_at(), Some(1));
    assert_eq!(game.stalemate().1, 0);
    assert!(game.get_mage(2).unwrap().powerup.is_none());
    let restored: Game = serde_json::from_str(&serde_json::to_string(&game).unwrap()).unwrap();
    assert_eq!(restored.overcharge_at(), Some(1));
    assert_eq!(restored.legal_turns(), game.legal_turns());
    let mut rewind = restored.rewind(1);
    assert_eq!(rewind.overcharge_at(), None);
    rewind.take_move(Position(0, 0), Position(1, 0)).unwrap();
    assert_eq!(rewind.overcharge_at(), Some(1));
    let turn = game.legal_turns()[0];
    game.take_move(turn.0, turn.1).unwrap();
    assert_eq!(game.overcharge_at(), Some(1));
}

#[test]
fn activation_uses_the_same_inactivity_clock_as_damage() {
    let mut level = duel();
    let mut victim = Mage::new(2, Team::Blue, MageSort::Plus, Position(1, 2));
    victim.mana.0 = 1;
    level.mages.push(victim);
    let game = Game::new(&level, true).unwrap();
    let mut snapshot = serde_json::to_value(game).unwrap();
    // Put the same red-to-move position beyond the opening grace period.
    snapshot["turns"] = serde_json::to_value(vec![Turn::sentinel(); 10]).unwrap();
    snapshot["last_nominal"] = serde_json::json!(9);
    let mut game: Game = serde_json::from_value(snapshot).unwrap();
    game.take_move(Position(0, 0), Position(1, 0)).unwrap();
    assert_eq!(game.overcharge_at(), Some(11));
    assert_eq!(game.stalemate(), (false, 1));
    let mut snapshot = serde_json::to_value(game).unwrap();
    snapshot["turns"] = serde_json::to_value(vec![Turn::sentinel(); 25]).unwrap();
    assert_eq!(
        serde_json::from_value::<Game>(snapshot.clone())
            .unwrap()
            .stalemate(),
        (false, 15)
    );
    snapshot["turns"] = serde_json::to_value(vec![Turn::sentinel(); 26]).unwrap();
    assert_eq!(
        serde_json::from_value::<Game>(snapshot.clone())
            .unwrap()
            .stalemate(),
        (false, 16)
    );
    snapshot["turns"] = serde_json::to_value(vec![Turn::sentinel(); 27]).unwrap();
    assert_eq!(
        serde_json::from_value::<Game>(snapshot)
            .unwrap()
            .stalemate(),
        (true, 17)
    );
}

#[test]
fn separated_regions_and_terminal_positions_do_not_get_futile_overcharge() {
    let mut level = corridors();
    level.mages.retain(|m| m.index == 0 || m.index == 3);
    assert_eq!(
        contact_reachability(&level, Team::Red, true),
        ContactReachability::Impossible
    );
    assert_eq!(Game::new(&level, true).unwrap().overcharge_at(), None);
    level.mages.retain(|m| m.team == Team::Red);
    assert_eq!(Game::new(&level, true).unwrap().overcharge_at(), None);
}
