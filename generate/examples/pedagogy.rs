//! Opening and replay decision diagnostics. Existing candidate arrays remain accepted.
//! Usage: cargo run --release -p generate --example pedagogy -- candidates.json [id-prefix]
//! Optional entry.replay names a simulator matchup JSON (relative to the working directory).
use std::{fs, path::PathBuf};

use generate::analysis::{GameResultRecord, Outcome};
use serde_json::{json, Value};
use shared::*;

fn search(game: &Game, depth: usize, nodes: u64) -> SearchResult {
    game.search(
        SearchLimits {
            max_depth: depth,
            max_nodes: Some(nodes),
            table_capacity: 32768,
        },
        1,
        || false,
    )
}
fn diagnose(game: &Game, nodes: u64) -> Value {
    let depths: Vec<_> = [2, 4, 8, 10]
        .into_iter()
        .map(|depth| {
            let result = search(game, depth, nodes);
            let gap = result
                .moves
                .get(1)
                .map(|next| (result.moves[0].score - next.score).abs());
            json!({"requested_depth": depth, "best_next_gap": gap, "search": result})
        })
        .collect();
    let result: SearchResult =
        serde_json::from_value(depths.last().unwrap()["search"].clone()).unwrap();
    let moves: Vec<_> = result.moves.iter().map(|m| {
        let mut next = game.clone();
        let pickup = next.powerups().get(&m.turn.1).copied();
        let hits = next.take_move(m.turn.0, m.turn.1).unwrap();
        let replies: Vec<_> = next.legal_turns().iter().map(|t| {
            let mut child = next.clone();
            let pickup = child.powerups().get(&t.1).copied();
            let hits = child.take_move(t.0, t.1).unwrap();
            json!({"turn": t, "hits": hits, "pickup": pickup, "result": child.result().map(|r| match r { GameResult::Win(Team::Red) => "RedWin", GameResult::Win(Team::Blue) => "BlueWin", GameResult::Stalemate => "Draw" }), "mana_difference": child.mana_difference()})
        }).collect();
        let damaging: Vec<_> = replies.iter().filter(|r| !r["hits"].as_array().unwrap().is_empty()).collect();
        json!({"turn": m.turn, "score": m.score, "hits": hits, "pickup": pickup, "replies": replies, "damaging_replies": damaging})
    }).collect();
    // A bounded best-play rollout is evidence, not a proof of a forced win.
    let mut continuation = game.clone();
    let mut line = Vec::new();
    for _ in 0..24 {
        if continuation.result().is_some() {
            break;
        }
        let r = search(&continuation, 8, 20_000);
        let Some(turn) = r.best_move() else {
            break;
        };
        line.push(
            json!({"turn": turn, "completed_depth": r.completed_depth, "stop": r.stop_reason}),
        );
        continuation.take_move(turn.0, turn.1).unwrap();
    }
    json!({"ply": game.turns(), "team": game.turn_for(), "legal_alternatives": game.legal_turns(),
        "completed_depth": result.completed_depth, "nodes": result.visited_nodes, "stop": result.stop_reason, "budget_exhausted": result.stop_reason == StopReason::Nodes,
        "moves": moves, "depths": depths, "continuation": line, "continuation_result": continuation.result().map(|r| match r { GameResult::Win(Team::Red) => "RedWin", GameResult::Win(Team::Blue) => "BlueWin", GameResult::Stalemate => "Draw" }),
        "continuation_capped": continuation.result().is_none()})
}
fn main() {
    let path = PathBuf::from(std::env::args().nth(1).expect("candidate JSON"));
    let entries: Vec<Value> = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    let filter = std::env::args().nth(2);
    let mut results = Vec::new();
    for entry in entries {
        let id = entry["id"].as_str().unwrap();
        if filter.as_ref().is_some_and(|f| !id.starts_with(f)) {
            continue;
        }
        let level = Level::parse_code(entry["code"].as_str().unwrap()).unwrap();
        let nodes = entry["diagnostic_nodes"].as_u64().unwrap_or(500_000);
        let mut opening = diagnose(&Game::new(&level, true).unwrap(), nodes);
        opening["id"] = json!(id);
        opening["code"] = entry["code"].clone();
        opening["config"] = json!({"version": 2, "engine": format!("fnv1a-{:016x}", generate::analysis::trial_seed(0, &[
            include_str!("../../shared/src/logic/game.rs"), include_str!("../../shared/src/logic/search.rs"),
            include_str!("../../shared/src/logic/deadlock.rs"), include_str!("pedagogy.rs")].join("\n"), 0)), "seed": 1, "depths": [2,4,8,10], "nodes": nodes,
            "table_capacity": 32768, "continuation_depth": 8, "continuation_nodes": 20000, "continuation_max_plies": 24,
            "selection": "first win and first loss; first rune, following player decision, midpoint, last player decision",
            "scores": "Red perspective; only completed iterations", "stalemates": true});
        let mut decisions = Vec::new();
        if let Some(path) = entry["replay"].as_str() {
            let matchup: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
            let games: Vec<GameResultRecord> =
                serde_json::from_value(matchup["games"].clone()).unwrap();
            for outcome in [Outcome::Win, Outcome::Loss] {
                let Some(record) = games.iter().find(|g| g.outcome == outcome) else {
                    continue;
                };
                let rune = record
                    .replay
                    .iter()
                    .position(|m| m.pickup.is_some())
                    .unwrap_or(2);
                let mut points = vec![
                    rune / 2 * 2,
                    (rune / 2 + 1) * 2,
                    record.plies / 4 * 2,
                    record.plies.saturating_sub(2) / 2 * 2,
                ];
                points.sort();
                points.dedup();
                let mut game = Game::new(&level, true).unwrap();
                for (ply, m) in record.replay.iter().enumerate() {
                    if points.contains(&ply) {
                        let mut point = diagnose(&game, nodes);
                        point["trial"] = json!(record.trial);
                        point["outcome"] = json!(outcome);
                        point["played"] = json!(m.turn);
                        if let Some(reply) = record.replay.get(ply + 1) {
                            let mut next = game.clone();
                            next.take_move(m.turn.0, m.turn.1).unwrap();
                            let response = search(&next, 8, 20_000);
                            let regret = response
                                .moves
                                .iter()
                                .find(|ranked| ranked.turn == reply.turn)
                                .map(|ranked| (response.moves[0].score - ranked.score).abs());
                            point["opponent_response"] =
                                json!({"played": reply.turn, "regret": regret, "search": response});
                        }
                        decisions.push(point);
                    }
                    assert_eq!(
                        game.take_move(m.turn.0, m.turn.1).unwrap(),
                        m.damage,
                        "replay damage mismatch"
                    );
                }
                assert_eq!(game.turns(), record.plies);
            }
        }
        opening["decisions"] = json!(decisions);
        results.push(opening);
    }
    println!("{}", serde_json::to_string_pretty(&results).unwrap());
}
