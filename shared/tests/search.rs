use std::collections::BTreeMap;

use shared::*;
fn fixture(team: Team, power: Option<PowerUp>) -> Game {
    let mut red = Mage::new(0, Team::Red, MageSort::Diamond, Position(0, 0));
    let mut blue = Mage::new(1, Team::Blue, MageSort::Cross, Position(2, 2));
    red.mana = Mana(2, 4);
    blue.mana = Mana(1, 4);
    blue.powerup = power;
    let mut pickups = BTreeMap::new();
    if let Some(p) = power {
        pickups.insert(Position(1, 0), p);
    }
    Game::new(
        &Level::new(Board::new(3, 3).unwrap(), vec![red, blue], pickups, team),
        true,
    )
    .unwrap()
}
fn minimax(game: &Game, depth: usize) -> isize {
    if game.result().is_some() || depth == 0 {
        return game.evaluate();
    }
    let values = game.legal_turns().iter().map(|t| {
        let mut g = game.clone();
        g.take_move(t.0, t.1).unwrap();
        minimax(&g, depth - 1)
    });
    if game.turn_for() == Team::Red {
        values.max().unwrap()
    } else {
        values.min().unwrap()
    }
}
#[test]
fn exact_scores_match_exhaustive_rules_and_uncached_search() {
    for team in [Team::Red, Team::Blue] {
        for power in [
            None,
            Some(PowerUp::Shield),
            Some(PowerUp::Beam),
            Some(PowerUp::Diagonal),
        ] {
            let game = fixture(team, power);
            for depth in 1..=4 {
                let result = game.search(SearchLimits::depth(depth), 123, || false);
                let mut limits = SearchLimits::depth(depth);
                limits.table_capacity = 0;
                let uncached = game.search(limits, 123, || false);
                assert_eq!(result.moves, uncached.moves);
                assert_eq!(result.completed_depth, depth);
                assert_eq!(result.moves.len(), game.legal_turns().len());
                for m in &result.moves {
                    let mut child = game.clone();
                    child.take_move(m.turn.0, m.turn.1).unwrap();
                    assert_eq!(m.score, minimax(&child, depth - 1));
                }
                assert_eq!(result.moves[0].score, minimax(&game, depth));
            }
        }
    }
}
#[test]
fn interruption_keeps_only_completed_iterations_and_seeded_fallback() {
    let game = fixture(Team::Blue, Some(PowerUp::Shield));
    let first = game.search(SearchLimits::depth(1), 44, || false);
    let mut limits = SearchLimits::depth(5);
    limits.max_nodes = Some(first.visited_nodes + 1);
    let interrupted = game.search(limits, 44, || false);
    assert_eq!(interrupted.completed_depth, 1);
    assert_eq!(interrupted.moves, first.moves);
    assert_eq!(interrupted.stop_reason, StopReason::Nodes);
    assert_eq!(interrupted, game.search(limits, 44, || false));
    limits.max_nodes = Some(0);
    let fallback = game.search(limits, 44, || false);
    assert!(fallback.moves.is_empty());
    assert!(game.legal_turns().contains(&fallback.best_move().unwrap()));
    let deadline = game.search(SearchLimits::depth(5), 44, || true);
    assert_eq!(deadline.stop_reason, StopReason::Deadline);
    assert_eq!(deadline.fallback, fallback.fallback);
}
#[test]
fn terminal_root_and_early_terminal_children() {
    let mut sleeper = Mage::new(0, Team::Red, MageSort::Cross, Position(0, 0));
    sleeper.mana = Mana(0, 4);
    let game = Game::new(&Level::default_with_mages(vec![sleeper]), true).unwrap();
    let result = game.search(SearchLimits::depth(4), 0, || false);
    assert_eq!(result.stop_reason, StopReason::Terminal);
    assert_eq!(result.best_move(), None);
    assert!(game.best_turn(4, 0).is_none());
    let red = Mage::new(0, Team::Red, MageSort::Diamond, Position(0, 0));
    let mut blue = Mage::new(1, Team::Blue, MageSort::Cross, Position(1, 1));
    blue.mana = Mana(1, 4);
    let live = Game::new(
        &Level::new(
            Board::new(3, 3).unwrap(),
            vec![red, blue],
            BTreeMap::from([(Position(1, 0), PowerUp::Beam)]),
            Team::Red,
        ),
        true,
    )
    .unwrap();
    let terminal_turn = live
        .legal_turns()
        .iter()
        .find(|t| {
            let mut child = live.clone();
            child.take_move(t.0, t.1);
            child.result().is_some()
        })
        .copied()
        .expect("fixture must exercise an early terminal child");
    let result = live.search(SearchLimits::depth(4), 42, || false);
    let mut terminal = live.clone();
    terminal.take_move(terminal_turn.0, terminal_turn.1);
    assert_eq!(
        result
            .moves
            .iter()
            .find(|m| m.turn == terminal_turn)
            .unwrap()
            .score,
        terminal.evaluate()
    );
}
#[test]
fn stalemate_counters_and_scores_are_rule_sensitive() {
    let game = fixture(Team::Red, None);
    let mut value = serde_json::to_value(&game).unwrap();
    value["turns"] = serde_json::to_value(vec![Turn(Position(0, 0), Position(1, 0)); 16]).unwrap();
    value["last_nominal"] = serde_json::json!(0);
    let ended: Game = serde_json::from_value(value.clone()).unwrap();
    assert!(ended.result().is_some());
    assert_eq!(
        ended
            .search(SearchLimits::depth(3), 0, || false)
            .best_move(),
        None
    );
    value["turns"] = serde_json::to_value(vec![Turn::sentinel(); 14]).unwrap();
    let near: Game = serde_json::from_value(value.clone()).unwrap();
    assert!(near.result().is_none());
    for m in near.search(SearchLimits::depth(3), 5, || false).moves {
        let mut child = near.clone();
        child.take_move(m.turn.0, m.turn.1).unwrap();
        assert_eq!(m.score, minimax(&child, 2));
    }
    value["turns"] = serde_json::to_value(vec![Turn::sentinel(); 16]).unwrap();
    value["level"]["mages"][0]["mana"][0] = serde_json::json!(1);
    let drawn: Game = serde_json::from_value(value.clone()).unwrap();
    assert!(drawn.result() == Some(GameResult::Stalemate));
    assert_eq!(drawn.evaluate(), 0);
    value["can_stalemate"] = serde_json::json!(false);
    let ongoing: Game = serde_json::from_value(value).unwrap();
    assert!(ongoing.result().is_none());
}
#[test]
fn ranked_sampling_renormalizes_and_allows_weaker_forced_outcomes() {
    let base = fixture(Team::Red, None).search(SearchLimits::depth(1), 0, || false);
    for difficulty in [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard] {
        assert_eq!(Difficulty::from_preference(difficulty.label()), difficulty);
        for count in 1..=3 {
            let mut result = base.clone();
            result.moves = (0..count)
                .map(|i| ScoredMove {
                    turn: Turn(Position(i, 0), Position(i, 1)),
                    score: if i == 0 { 99999 } else { -99999 },
                })
                .collect();
            let mut counts = vec![0usize; count as usize];
            for seed in 0..10000 {
                let turn = result.select(difficulty, seed).unwrap();
                assert_eq!(Some(turn), result.select(difficulty, seed));
                counts[result.moves.iter().position(|m| m.turn == turn).unwrap()] += 1;
            }
            let weights = difficulty.weights();
            let sum: u64 = weights[..count as usize].iter().sum();
            for (i, actual) in counts.iter().enumerate() {
                assert!((*actual as f64 / 10000.0 - weights[i] as f64 / sum as f64).abs() < 0.025);
            }
        }
    }
    for invalid in ["", "invalid", "normal"] {
        assert_eq!(Difficulty::from_preference(invalid), Difficulty::Normal);
    }
}

#[test]
fn seeds_include_ordered_history() {
    let game = fixture(Team::Red, None);
    let a = Turn(Position(0, 0), Position(1, 0));
    let b = Turn(Position(1, 0), Position(0, 0));
    let mut value = serde_json::to_value(&game).unwrap();
    value["turns"] = serde_json::to_value(vec![a, b]).unwrap();
    let first: Game = serde_json::from_value(value.clone()).unwrap();
    value["turns"] = serde_json::to_value(vec![b, a]).unwrap();
    let second: Game = serde_json::from_value(value).unwrap();
    assert_ne!(first.history_seed(42), second.history_seed(42));
    assert_eq!(first.history_seed(42), first.clone().history_seed(42));
}
