//! Release-mode contact detector timings. No wall-clock assertions.
use std::{collections::BTreeMap, hint::black_box, time::Instant};

use shared::*;

fn main() {
    let mut fixtures: Vec<_> = campaign_catalogue(false)
        .into_iter()
        .filter(|e| !e.chaos)
        .map(|e| (e.id.clone(), e.level()))
        .collect();
    for size in [3, 8] {
        fixtures.push((
            format!("parity-{size}"),
            Level::new(
                Board::new(size, size).unwrap(),
                vec![
                    Mage::new(0, Team::Red, MageSort::Diamond, Position(0, 0)),
                    Mage::new(
                        1,
                        Team::Blue,
                        MageSort::Knight,
                        Position(size as i8 - 1, size as i8 - 1),
                    ),
                ],
                BTreeMap::new(),
                Team::Red,
            ),
        ));
    }
    let mut results = Vec::new();
    for (name, level) in fixtures {
        let mut micros = Vec::new();
        for _ in 0..9 {
            let start = Instant::now();
            for _ in 0..2000 {
                black_box(contact_reachability(
                    black_box(&level),
                    level.starting_team,
                    false,
                ));
            }
            micros.push(start.elapsed().as_secs_f64() * 1e6 / 2000.0);
        }
        micros.sort_by(f64::total_cmp);
        results.push(serde_json::json!({"fixture": name, "status": format!("{:?}", contact_reachability(&level, level.starting_team, false)), "median_us": micros[4], "min_us": micros[0], "max_us": micros[8], "batch_calls": 2000, "batches": 9}));
    }
    println!("{}", serde_json::to_string_pretty(&results).unwrap());
}
