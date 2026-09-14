# Maginet

Maginet is a small, chess-like tactics game. Mages share a movement pattern, but each kind has a different attack pattern. Moving triggers attacks; mana is health, and a mage with no mana sleeps. Powerups modify movement, attacks, or defense.

This README is the starting point for contributors, including coding agents. Keep the project simple: plain Rust types, explicit state transitions, and small modules. Add machinery only when the game needs it.

## Run it

Use Rust/Cargo, the `wasm32-unknown-unknown` target, and `wasm-pack`. Run commands from the repository root.

```sh
rustup target add wasm32-unknown-unknown
wasm-pack build --target web --debug --out-name maginet --out-dir static/js/pkg
cargo run -p server
```

The server listens on `127.0.0.1:8000` and serves the page, assets, and API. Open `http://127.0.0.1:8000`.

**API routing:** `src/net.rs` currently points non-deploy clients at `https://tunnel.evrim.zone`, and deploy clients at `https://maginet.evrim.zone`. The existing local workflow expects a tunnel to the local server. For a fully local setup, change the non-deploy `API_URL` to `http://127.0.0.1:8000` while developing; do not accidentally commit a personal endpoint change.

Optional rebuild watchers (`watchexec`):

```sh
watchexec -w server/src -w shared -r -e rs -- cargo run -p server
watchexec -w src -w shared -r -e rs -- wasm-pack build --target web --debug --out-name maginet --out-dir static/js/pkg
```

Build against the production API by adding `-- --features deploy` to the `wasm-pack` command. `deploy` also disables development hotkeys. `demo` gates demo content. Building does not publish the game. Generated Wasm packages and `target/` are ignored; edit the Rust sources rather than generated JavaScript.

## Where things live

| Path | Responsibility |
| --- | --- |
| `shared/src/logic/` | Deterministic rules: boards, levels, mages, spells, powerups, turns, results, and AI search. No rendering or animation timing. |
| `shared/src/lobby.rs`, `shared/src/net.rs` | Lobby lifecycle and serialized messages shared by client and server. |
| `src/lib.rs` | Browser setup, assets, input listeners, and animation loop. |
| `src/app/app.rs` | App state dispatch, logical canvas layout, input, and browser storage. |
| `src/app/render.rs` | Native-resolution game/UI buffers and integer nearest-neighbour display scaling. |
| `src/app/state/` | Game, tutorial, editor, and menus; each implements `State::tick` and `State::draw`. |
| `src/app/presentation.rs` | Client-only turn playback, sampled sprite positions, and one-shot UI signals. |
| `src/app/particle.rs`, `src/app/audio.rs`, `src/app/ui.rs` | Particle effects, sound, and small UI components. |
| `src/draw.rs` | Canvas drawing and sprite-atlas helpers. |
| `src/net.rs` | Browser requests and incoming message pool. |
| `server/src/main.rs` | Axum server, sessions, lobbies, and turn validation. |
| `generate/` | Offline level generation and simulation tool. |
| `html/`, `static/` | Page shell, loader, styles, sprite atlas, and audio. |
| `TODO.md` | Followup work, completed items, and deliberately skipped ideas. |

Start with `shared/src/logic/game.rs` for mechanics and `src/app/state/game.rs` for the playable screen. A `Turn(from, to)` is the command: `take_move` validates it, applies movement/pickup/attacks, and returns hit tiles. The same rules support local play, AI, and server replication.

## Pixel grid

The board occupies a native 256×256 play area inside a 400×272 frame that preserves the surrounding menus and controls. Game and interface layers render into offscreen canvases at that native resolution; the atlas retains its original resolution. `Renderer` composites those layers onto one display canvas with nearest-neighbour scaling. Logical drawing never scales with device-pixel ratio. All sprite blits snap their final transformed origin to whole native pixels (including mage bodies and scaled tiles), and normalize quarter-turn rounding errors. Animation state stays continuous; only raster placement is snapped.

Display scale is a whole number of device pixels per native pixel, fitted to the window with room for navigation. Resizing changes only presentation size, and mouse/touch coordinates map from the displayed bounds back to the native grid. Portrait orientation rotates at native resolution before enlargement. Keep new graphics on this grid; do not draw effects directly on the enlarged display canvas or let CSS rescale it independently.

## Animation and UI flow

The lobby's game is ground truth and advances immediately. The presentation queue holds each accepted turn's before/after snapshots and actual hit tiles. Local moves, AI moves, and batches received online all enter this same queue. Playback does not modify the rules or wire format.

Each transition has two phases, measured using the app's elapsed-time clock (60 logical frames per second):

1. **Glide:** 18 frames (300 ms), with smoothstep interpolation between board coordinates. Sprites, mana bars, and attached particles use the sampled fractional position. A separate 12-pixel body offset rises gently and drops sharply at landing; shadows stay on the ground. The pickup becomes visible on arrival.
2. **Attack:** when there are hits, missiles travel for 12 frames (200 ms), starting with forward speed and accelerating into impact. Missiles launch during the final three movement frames. The horizontal cast flip starts 50 ms after launch, at landing, and lasts six frames (100 ms). Each frame fills the missile’s travelled segment through to impact with particles spaced two board pixels apart, including after skipped frames. Each missile emits a short-lived trail made only from randomly cropped 4×4 quadrants of the existing 8×8 missile sprites. Shield retaliation originates at a defending mage. At impact, the after-snapshot becomes visible, including damage, sleeping mages, the next turn, and the result.

`Presentation::mages` and `missile_trail` sample the timeline; they do not integrate sprite motion or change entities. `advance` consumes crossed boundaries and returns ordered `Move`, `Pickup`, and `Impact` signals. The game screen handles those signals with sound, fireworks, hit flashes, and shake. Crossed signals fire once, including when frames are skipped. Multiple hits in a turn arrive together.

Gameplay input and AI wait for playback to finish; menus and undo remain available. Tutorial progression reads the presented game. Undo rewinds the rules immediately and queues the inverse snapshots after any playback already in progress. Each undone move glides back in reverse turn order, restoring health and pickups on landing without replaying attacks. Replacement lobby snapshots discard playback and particles. Readiness-only lobby updates preserve playback by comparing deterministic game history. Full game replacements snap to their authoritative state; ordinary turn messages animate.

To add an effect, derive its data from the accepted turn and snapshots, sample continuous visuals from time, and use a signal for one-shot effects. Keep timing out of `shared/`; do not add sleeps, browser callbacks per piece, or a second rules engine. Timing constants live together in `presentation.rs`.

## Campaign progression

The campaign map starts at the guided Tutorial portal beside Basics I. Complete the tutorial (from the map or main menu) to unlock campaign battles; leaving it unfinished keeps them locked. Saved wins remain completed even after a later loss.

Winning a level unlocks its cardinal neighbours. Available levels keep their names, and locked names are revealed when adjacent to an available level; more distant names read `???`. The full campaign progresses through Grass, Desert, Flesh, Crust, and Eldritch tilesets by map column. Each portal uses the same style as its battle. Styles do not change level codes or existing progress keys.

A fixed top-left star counter shows completed portals over the total, including the tutorial, with the total adapted to the demo build.

The completion twinkle plays once when returning to the map from a newly won level. Selecting completed levels, opening the map, and replaying an existing win do not retrigger it. The guided tutorial returns to its portal when entered from the campaign, including after a rematch.

## Way of working

- Read the relevant code and `TODO.md` before editing. Check `git status`; preserve work already in progress.
- Work on one requested task at a time. Animation combines a board-plane glide with a fake vertical lift and landing. Other followups remain separate tasks.
- Keep rules deterministic and rendering client-only. Use mage IDs for identity; array order is not identity. Sort visual copies for drawing.
- Prefer existing modules and helpers. Avoid new dependencies, frameworks, or broad refactors for a small feature.
- Test behavior at the boundary you changed. Timing tests should cover skipped frames, ordering, impact-state consistency, and resets rather than duplicate implementation details.
- Format changed Rust files using the repository's `rustfmt.toml`. Avoid unrelated formatting churn. Update this README when the workflow or architecture changes.
- Summarize the change, checks, and remaining limitations. Do not mark TODO items complete until their requested behavior has been checked.

## Check a change

```sh
cargo test --workspace
cargo check --workspace
cargo check -p maginet --target wasm32-unknown-unknown --all-features
```

The presentation tests run natively without a browser. Build the browser client with the command above after changing rendering or browser integration.

For animation changes, also play through a no-hit move, a multi-hit attack, a lethal hit, diagonal/beam pickups, shield retaliation, and undo during playback. Check that mana bars travel with their sprites, missiles accelerate, damage and victory appear at impact, and tutorial text does not get ahead of playback. In online play, check a batch of turns and a rematch/snapshot replacement. A browser pass is needed to judge visual feel and audio; passing Rust tests alone does not establish that.

## Solver and opponent difficulty

Settings includes a saved AI difficulty selector below the audio controls. Missing or invalid `difficulty` storage values use Normal. Campaign and local AI battles use this preference; the tutorial and main-menu demonstration always use Easy. Stars, wins, and campaign unlocks keep their existing storage keys and behavior.

| Difficulty | Maximum plies | Search budget | Best / second / third move |
| --- | ---: | ---: | --- |
| Easy | 2 | 50 ms | 65% / 25% / 10% |
| Normal | 4 | 150 ms | 80% / 15% / 5% |
| Hard | 8 | 500 ms | 90% / 8% / 2% |

These are initial tuning values, not measured strength guarantees. Sampling renormalizes for fewer than three moves, and even Hard can choose a weaker forced outcome. Seeded tie ordering and selection use the game seed mixed with its ordered turn history. Identical seeds and completed depths reproduce choices; wall-clock budgets can complete different depths on different runs. Use node limits for deterministic analysis.

`shared/src/logic/search.rs` implements the single iterative-deepening alpha-beta engine used by `best_turn`, `best_turn_auto`, and native simulations. Terminal results are checked before the depth horizon, and evaluations contain no random noise. Every root move receives a full-window search so published scores are exact and comparable at one completed depth. Previous iterations order root moves and cached internal best moves. A per-search table holds at most 32,768 entries, distinguishes exact/lower/upper bounds, and keys the current level, side to move, turn count, last attack counter, and stalemate setting. Successors use `Game::take_move` and clones of authoritative state.

`Game::search(SearchLimits, seed, deadline)` returns ranked root scores (positive for Red), completed depth, visited successor nodes, and a stop reason. An interrupted iteration never replaces completed results. Before the first iteration completes, selection returns a seeded legal fallback; a terminal root returns no move. `SearchResult::best_move` gives the actual best move; `select` applies difficulty probabilities separately. Set `table_capacity` to zero to compare uncached search.

The game screen owns one `Pending` search. `static/js/ai-client.js` starts a module worker with a JSON game snapshot, difficulty, seed string (preserving all 64 bits), request ID, and game revision. `ai-worker.js` loads the existing Wasm package and calls `search_ai`; Wasm startup skips DOM and audio initialization outside a window. Search time excludes worker/Wasm startup. Replies include the selected turn and search statistics. The screen validates IDs, revision, and legality, then queues `Message::Turn` at the existing presentation eligibility point. Undo and replacement cancel pending work; dropping a screen on navigation/rematch releases callbacks and terminates its worker. Worker errors release pending state and use a seeded legal fallback with zero search depth on the main thread.

Reproducible checks:

```sh
cargo test --workspace
cargo run --release -p shared --example search_bench
wasm-pack build --target web --out-name maginet --out-dir static/js/pkg
```

The benchmark also accepts `-- --snapshot` to emit the fixed positions as JSON. On one local release run, the 3×3/two-mage position at depth 5 visited 86 nodes with caching versus 101 without (about 0.15 ms each). The 8×8/six-mage position at depth 4 visited 957 versus 1,098 nodes (about 2.5 versus 2.3 ms). Instrumenting the previous engine on those fixtures gave alpha-beta 61/584 nodes (0.13/1.8 ms) and gameplay PVS 55/1,280 nodes (0.09/4.0 ms; PVS uses depth 4 on both). The new engine improves the larger gameplay workload, but exact scores for all roots and iterative deepening add work relative to the old best-only alpha-beta. Cache overhead can exceed savings on small searches. Timings are informational and have no CI assertions.

For automated browser checks, run the server at `http://127.0.0.1:8000`, install `playwright-core` outside the repository, and run:

```sh
npm install --prefix /tmp/maginet-browser-check playwright-core
NODE_PATH=/tmp/maginet-browser-check/node_modules node scripts/check-ai-worker.cjs
NODE_PATH=/tmp/maginet-browser-check/node_modules node scripts/check-ai-ui.cjs
```

Set `CHROME_PATH` if Chrome is not at the default macOS location. These checks exercise real worker startup and main-thread responsiveness, reply filtering, error reporting, callback cleanup, saved difficulty selection, tutorial/menu overrides, and cancellation on navigation, undo, and rematch, including a late reply after undo. They do not require visual inspection. The native scenario analyser and initial campaign survey are documented below.

## Scenario analysis

The native `generate` executable has explicit `analyse`, `campaign`, and `generate` subcommands. No arguments prints help; `generate` preserves the original level-generation workflow.

```sh
cargo run --release -p generate -- analyse --code hg12g014cm0j800 --output /tmp/basics-survey
cargo run --release -p generate -- campaign --output assessments/campaign-seed-1
cargo run --release -p generate -- campaign --demo --games 100 --output /tmp/demo-survey
cargo run --release -p generate -- generate
```

Analysis defaults to 30 games per matchup, seed 1, a 200-ply safety limit, and at most four Rayon workers. All nine Easy/Normal/Hard player/opponent combinations run for battles. The tutorial runs three player profiles against Easy, with stalemates disabled. Red is the player, Blue the opponent; every scenario retains its starting team. Campaign defaults to all 27 battles plus the tutorial, independently of browser demo features; `--demo` selects four battles plus the tutorial.

| Profile | Maximum depth | Nodes per move | Ranked probabilities |
| --- | ---: | ---: | --- |
| Easy | 2 | 1,000 | 65/25/10 |
| Normal | 4 | 5,000 | 80/15/5 |
| Hard | 8 | 20,000 | 90/8/2 |

These deterministic node caps are analysis defaults, not equivalents of browser time budgets or guarantees of relative strength. Override `--games`, `--seed`, `--max-plies`, `--easy-nodes`, `--normal-nodes`, `--hard-nodes`, `--workers`, and `--output`. Zero node caps exercise legal fallback; zero maximum plies records nonterminal roots as unresolved. Games and workers must be positive. Malformed level codes fail with an error; valid legacy record ordering is normalized for seed derivation.

`shared/src/campaign.rs` is the UI/analyser catalogue of names, original codes, positions, styles, demo membership, and tutorial identity. Existing saved-progress keys still use `Level::as_code()`. Rite III and Rite IV intentionally remain separate portals with identical scenarios and shared progress. `generate::analysis` exposes agent settings, run configuration, game simulation, telemetry, aggregation, and Wilson intervals. Simulations use the shared authoritative moves, bounded search table, completed-iteration fallback, history seeding, and ranked selection.

Each output directory contains `metadata.json`, one `matchup-LEVEL-RED-BLUE.json` checkpoint per completed matchup, `aggregate.json`, and `report.md`. Indices follow the metadata catalogue/profile order, and trial indices start at zero. Checkpoints are replaced atomically and reports update after each matchup. Repeating the same command resumes completed matchups; partial matchups are rerun. Configuration, catalogue, and engine-source fingerprints must match. Worker count and output path may change because they cannot affect results. Keep metadata with checkpoints, and use a separate output directory for another configuration. Do not run two writers into one directory.

Trial seeds use FNV-1a over the base seed's eight little-endian bytes, the canonical `level.as_code()` bytes, and the trial index's eight little-endian bytes. The same trial seeds are reused across matchups. Each move mixes that seed with ordered game history via `Game::history_seed`. To replay one trial programmatically, call `generate::analysis::simulate` with the recorded configuration, level, stalemate setting, profile indices, and trial index; its returned seed must match the checkpoint. CLI reruns reproduce the full trial range.

Reports distinguish actual rule draws from safety-limit terminations. Red's resolved win rate is wins divided by wins + losses + draws, with a 95% Wilson interval. The possible overall range treats every unresolved game first as a non-win and then as a win. More than 10% unresolved prevents definitive difficulty ranking. Progression flags compare Normal/Normal results across cardinal neighbours moving outward by shortest tutorial distance. A drop of at least 20 percentage points is supported only with disjoint confidence intervals and acceptable unresolved rates; these exploratory comparisons have no multiple-comparison correction.

The [initial survey](assessments/campaign-seed-1/report.md) and [interpretation and recommendations](assessments/campaign-seed-1/interpretation.md) cover 246 matchups and 7,380 games. These describe simulated agents, not validated human difficulty. No levels, placements, or progression were rebalanced.

Verification includes terminal wins/losses/draws, safety limits, both starting teams, powerups, fallback, malformed inputs, statistics, serial/parallel/resumed equality, campaign saves, full/demo membership, and browser campaign navigation. Run:

```sh
cargo test --workspace
cargo test -p maginet --features demo
cargo check --workspace
cargo check -p maginet --target wasm32-unknown-unknown --all-features
wasm-pack build --target web --out-name maginet --out-dir static/js/pkg
NODE_PATH=/tmp/maginet-browser-check/node_modules node scripts/check-campaign.cjs
python3 scripts/audit-campaign.py assessments/campaign-seed-1
```

The browser check uses the server at `http://127.0.0.1:8000`, `playwright-core`, and Chrome as described above. The audit independently checks matchup coverage, trial seeds, outcome accounting, game lengths, node caps, and aggregate telemetry. Editor analysis UI, human playtesting, and campaign rebalancing remain followups.
