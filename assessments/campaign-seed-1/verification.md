# Verification record

Verified on 14 September 2026 using `rustc 1.96.0-nightly (69370dc4a 2026-03-05)` on Darwin arm64. Analysis used the release executable, four workers, seed 1, 30 trials per matchup, the default 200-ply limit and 1,000/5,000/20,000 node caps. Engine-source fingerprint: `fnv1a-b68d3374820e3a28` (also in metadata).

| Check | Result |
|---|---|
| `cargo test --workspace` | Passed, including parser, simulation, search, aggregation, CLI, resumption, progression, and UI state tests. |
| `cargo test -p maginet --features demo` | Passed, including demo reachability, tutorial gating, styles, and saved progress. |
| `cargo check --workspace` | Passed. Existing warnings remain. |
| `cargo check -p maginet --target wasm32-unknown-unknown --all-features` | Passed. |
| `wasm-pack build --target web --out-name maginet --out-dir static/js/pkg` | Passed. |
| `scripts/check-campaign.cjs` with Playwright/Chrome and the local server | Passed: fresh campaign enters the tutorial with Easy and no stalemates; a tutorial win unlocks Basics I, using the saved Hard preference and campaign stalemate rules. No page errors. |
| Catalogue compared to the pre-extraction source | All 27 battle names, original codes, and positions match exactly; full/demo counts are 28/5 including the tutorial. |
| Full Basics I matrix, one versus four workers | All 270 complete game records and every output file are byte-identical. |
| Interrupted-matchup/resume tests | Removing a completed checkpoint and resuming reproduces the serial output; mismatched configuration/catalogue, invalid checkpoints, and missing metadata are rejected. |
| Full survey resumed with one and four workers | All JSON data remain unchanged; regenerated reports are byte-identical across repeated resumes. |
| `python3 scripts/audit-campaign.py assessments/campaign-seed-1` | Passed: 246 matchups, 7,380 games, all trial seeds independently reconstructed, all outcomes/lengths/telemetry accounted for, per-profile node caps respected. |

The final audit counts are 3,399 Red wins, 2,965 Blue wins, 1,016 draws, and 0 unresolved games. All 27 battles have all nine skill matchups, and the tutorial has all three player profiles against Easy. Each matchup has exactly 30 ordered trials. Rite III and Rite IV have identical records in all nine matchup pairs, as expected from the preserved duplicate scenario.

Tests include terminal wins/losses/draws, zero-ply safety limits, an authoritative forced beam-pickup win/loss with zero-node legal fallback, each starting team's first search, and completed-iteration fallback. Confidence-interval tests cover a known Wilson value, empty resolved samples, draw denominators, and unresolved bounds. Progression tests cover overlapping intervals, excessive unresolved games, the 20-point threshold, and shortest tutorial distance. No timing assertions were added to CI.

The browser check is a navigation/state smoke test, not human playtesting or a validation of difficulty. The initial survey is exploratory and does not adjust its many interval comparisons for multiplicity. No campaign content or progression was changed.
