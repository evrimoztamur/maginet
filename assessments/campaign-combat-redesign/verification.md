# Verification

- Workspace tests, demo tests (20), native workspace check, all-features Wasm check, and `wasm-pack build --target web --out-name maginet --out-dir static/js/pkg` passed. Existing compiler warnings remain.
- `scripts/check-combat-redesign.cjs` passed in Chrome: all three redesigned battles load from the campaign, use 4×4 boards with the expected mage counts, and reproduce the first recorded move, resulting team mana, and Normal AI request. The capstone screenshot was visually inspected.
- `scripts/check-campaign-graph.cjs` passed with the live shared catalogue: cardinal arrows, loops, junctions, one-way exits, blocked reverse unlocks, nearby unconnected portals, and the 36-star total remain correct.
- `scripts/audit-campaign.py` passed for all 11 datasets: eight 30-game full-matchup screens and three 300-game Normal/Normal followups.
- `scripts/audit-campaign-replays.py assessments/campaign-combat-redesign` verified move order, acting team, hit accounting and mana for all 3,060 executed trial records.
- `scripts/report-combat-redesign.py` verified identical engine fingerprints, full simulation configurations and paired trial seeds against the preceding 300-trial datasets. Final first-30 replay records exactly match the corresponding screens. It produced the current 318-cell campaign matrix with actual sample counts and explicit reuse attribution.
- The exported graph matches the preceding graph exactly in all connections, route orders, portal IDs, positions, styles and membership. Exactly three scenario codes differ: Patterns I, Rite II and Rite IV. All 36 codes remain distinct and pass parsing/round-trip tests.
- `git diff --check` passed for the task changes. The original and preceding assessment trees are unchanged.

The native simulation/search tests cover deterministic serial/parallel/resumed runs and authoritative replay application. Neither simulation nor search code changed in this redesign. This work changes only the three battle definitions, assessment/report tooling, browser checks and documentation.

Concurrent edits to `deploy-itch.sh` and `src/draw.rs` were left untouched and excluded from the redesign commits.

The reported reductions are exploratory AI-versus-AI evidence, not human difficulty validation. Remaining inactivity and immobilization endings are documented in the interpretation. Candidate selection and reuse of the screen’s first 30 seeds limit independence; there is no claim that the draw problem is eliminated.
