//! Reproducible release-mode search workload; timings are informational only.
use std::{collections::BTreeMap, time::Instant};

use shared::*;
fn main() {
    for (name, size, count, depth) in [("small", 3, 2, 5), ("large", 8, 6, 4)] {
        let mages = (0..count)
            .map(|i| {
                Mage::new(
                    i,
                    Team::from_index(i),
                    MageSort::from(i % 5),
                    Position((i % 2 * (size - 1)) as i8, (i / 2 * 2) as i8),
                )
            })
            .collect();
        let game = Game::new(
            &Level::new(
                Board::new(size, size).unwrap(),
                mages,
                BTreeMap::new(),
                Team::Red,
            ),
            true,
        )
        .unwrap();
        if std::env::args().any(|a| a == "--snapshot") {
            println!("{}", serde_json::to_string(&game).unwrap());
            continue;
        }
        for capacity in [0, 32768] {
            let mut limits = SearchLimits::depth(depth);
            limits.table_capacity = capacity;
            let start = Instant::now();
            let result = game.search(limits, 42, || false);
            println!(
                "{name}: depth={} cache={capacity} nodes={} elapsed={:?}",
                result.completed_depth,
                result.visited_nodes,
                start.elapsed()
            );
        }
    }
}
