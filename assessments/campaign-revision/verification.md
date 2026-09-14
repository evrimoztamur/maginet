# Verification record

The implementation retains the authoritative AI profiles and game rules. The original `assessments/campaign-seed-1` tree is unchanged. Revision artifacts are committed separately from implementation and documentation.

## Automated checks

- `cargo test --workspace`: passed, including all catalogue, simulation, CLI, search and campaign UI state tests.
- `cargo test -p maginet --features demo`: passed (20 tests).
- `cargo check --workspace`: passed.
- `cargo check -p maginet --target wasm32-unknown-unknown --all-features`: passed.
- `wasm-pack build --target web --out-name maginet --out-dir static/js/pkg`: passed.
- `NODE_PATH=/tmp/maginet-browser-check/node_modules node scripts/check-campaign.cjs`: passed fresh tutorial entry and completed-tutorial battle entry; tutorial AI/stalemate overrides preserved.
- `NODE_PATH=/tmp/maginet-browser-check/node_modules node scripts/check-campaign-graph.cjs`: passed actual canvas arrow drawing, cardinal shafts, junction entry, both sides of the Diagonals loop, both Rite IV exits, blocked reverse unlocking, nearby unconnected portals, and the 2/36 star counter. Screenshots at Junction I and Rite IV were inspected.
- `python3 scripts/audit-campaign.py DIR`: passed for all 42 revision datasets, including the two archived Patterns I candidate datasets. The unchanged original full survey also passes its audit.
- `python3 scripts/audit-campaign-replays.py`: independently verified move order, acting team, damage and mana accounting for all 12,090 executed game records, including the archived candidate and overlapping screen/followup trials.
- `python3 scripts/report-campaign-revision.py`: verified paired trial seeds, exact original first-30 record reproduction, and exact screen/final first-30 replay equality. It generated 318 combined matchup summaries representing 13,860 trials, with explicit source attribution and each cell's actual sample count.
- `git diff --check`: passed.

Existing compiler warnings about unused items/imports and an elided lifetime remain; checks have no errors.

## Graph invariants

All 36 portals have unique IDs, positions, scenario codes and progress keys. The eight junctions each contain exactly one Red and one Blue mage. All 40 connections join cells with Manhattan distance exactly one, so there are no diagonal links or skipped cells. Duplicate shared route segments are emitted only once.

Every portal is reachable from Tutorial. Removing any required Basics, Patterns, dedicated powerup introduction, Rite IV or Ascension I blocks the ending. A Challenge-free route exists, as does a route that bypasses Rites I–III. Removing junction IDs from each expanded optional route reproduces the originally requested battle sequence. Junction I is a shared split after Shields I; all teaching prerequisites remain mandatory.

Each final branch exit works forwards and remains blocked backwards, in both shared-graph and actual campaign UI state tests. Nearness without an explicit link does not unlock a portal. Demo filtering returns exactly five portals and four connections. Tutorial gating, replay availability, map/battle styles and unique star progress remain covered by native tests; the browser check verifies the new star denominator.

## Reproduction and provenance

Full serial, parallel and resumed test runs produce byte-identical replay/checkpoint/report output with an explicit baseline namespace. The test uses the unchanged 200-ply cap and completes full games. Resume rejects altered seed namespaces, graph directions, main-route order, catalogue data, configuration and corrupt checkpoints. A separate replay test reapplies recorded moves through the authoritative game engine and checks pickups, damage and mana totals. Scenario changes preserve seeds under an explicit namespace; default scenario-derived seeds still change normally.

The eight earlier target-scenario runs predate the cardinal junction layout. Their original layout metadata is preserved, and their scenario results are explicitly reused by the combined report under the current exported graph. This is not a resume migration: changed metadata is rejected, and new experiments use new output directories. The eight junction scenarios have no original baseline counterparts.

## Limits

AI-versus-AI trials do not validate human difficulty or tutorial comprehension. Inactivity endings and draws remain in Patterns I, Rite II, Rite IV and some junctions. The interpretation records these concerns explicitly. No accepted final scenario produced a damage-free game or reached the 200-ply cap in the final Normal/Normal samples; that observation does not guarantee resolution for every seed/profile.
