//! Assess independent seeded Chaos loadouts separately from fixed campaign puzzles.
use generate::analysis::*;
use rayon::prelude::*;
use serde_json::json;
use shared::*;
fn main() {
    let games = std::env::args()
        .nth(1)
        .map(|s| s.parse().unwrap())
        .unwrap_or(30);
    let entry = campaign_catalogue(false)
        .into_iter()
        .find(|e| e.chaos)
        .unwrap();
    let config = RunConfig {
        games,
        replays: true,
        seed_namespace: Some("campaign-challenge-chaos".into()),
        ..Default::default()
    };
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .unwrap();
    let mut reports = Vec::new();
    for (red, blue) in [(2, 1), (1, 1), (2, 2)] {
        let records: Vec<_> = pool.install(|| {
            (0..games)
                .into_par_iter()
                .map(|trial| {
                    let loadout_seed = trial_seed(1, "chaos-loadout", trial);
                    let lobby = Lobby::new(
                        LobbySettings {
                            seed: loadout_seed,
                            loadout_method: LoadoutMethod::ArenaChaos(
                                entry.level(),
                                entry.position,
                            ),
                            ..Default::default()
                        },
                        std::time::Duration::ZERO,
                    );
                    let code = lobby.game.prototype_code();
                    let game = simulate(
                        &Level::parse_code(&code).unwrap(),
                        true,
                        &config,
                        red,
                        blue,
                        trial,
                    );
                    (code, loadout_seed, game)
                })
                .collect()
        });
        let stats = aggregate(
            &records
                .iter()
                .map(|(_, _, g)| g.clone())
                .collect::<Vec<_>>(),
        );
        reports.push(json!({"red":red,"blue":blue,"stats":stats,"trials":records}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({"config":config,"template":entry,"matchups":reports}))
            .unwrap()
    );
}
