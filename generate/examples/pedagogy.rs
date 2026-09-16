//! Opening decisions for teaching candidates, using completed exact root scores.
use std::{fs, path::PathBuf};

use serde_json::json;
use shared::*;

fn main() {
    let path = PathBuf::from(std::env::args().nth(1).expect("candidate JSON"));
    let entries: Vec<serde_json::Value> = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    let filter = std::env::args().nth(2);
    let mut results = Vec::new();
    for entry in entries {
        let id = entry["id"].as_str().unwrap();
        if filter.as_ref().is_some_and(|f| !id.starts_with(f)) {
            continue;
        }
        let level = Level::parse_code(entry["code"].as_str().unwrap()).unwrap();
        let game = Game::new(&level, true).unwrap();
        let result = game.search(
            SearchLimits {
                max_depth: 10,
                max_nodes: Some(500_000),
                table_capacity: 32768,
            },
            1,
            || false,
        );
        let moves=result.moves.iter().map(|m| {
            let mut next=game.clone();
            let hits=next.take_move(m.turn.0,m.turn.1).unwrap();
            let replies=next.legal_turns().iter().filter_map(|t| {
                let mut child=next.clone();
                let hits=child.take_move(t.0,t.1)?;
                (!hits.is_empty()).then(|| json!({"turn":t,"hits":hits,"mana_difference":child.mana_difference()}))
            }).collect::<Vec<_>>();
            json!({"turn":m.turn,"score":m.score,"hits":hits,"damaging_replies":replies})
        }).collect::<Vec<_>>();
        results.push(json!({"id":id,"code":entry["code"],"completed_depth":result.completed_depth,"nodes":result.visited_nodes,"stop":result.stop_reason,"moves":moves}));
    }
    println!("{}", serde_json::to_string_pretty(&results).unwrap());
}
