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
| `src/app/app.rs` | App state dispatch, canvas layout, input, and browser storage. |
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

## Animation and UI flow

The lobby's game is ground truth and advances immediately. The presentation queue holds each accepted turn's before/after snapshots and actual hit tiles. Local moves, AI moves, and batches received online all enter this same queue. Playback does not modify the rules or wire format.

Each transition has two phases, measured using the app's elapsed-time clock (60 logical frames per second):

1. **Glide:** 18 frames (300 ms), with smoothstep interpolation between board coordinates. Sprites, mana bars, and attached particles use the sampled fractional position. A separate 12-pixel body offset rises gently and drops sharply at landing; shadows stay on the ground. The pickup becomes visible on arrival.
2. **Attack:** when there are hits, missiles travel for 12 frames (200 ms), starting with forward speed and accelerating into impact. Missiles launch during the final three movement frames. The horizontal cast flip starts 50 ms after launch, at landing, and lasts six frames (100 ms). Each frame fills the missile’s travelled segment through to impact with particles spaced two board pixels apart, including after skipped frames. Each missile emits a short-lived trail made only from randomly cropped 4×4 quadrants of the existing 8×8 missile sprites. Shield retaliation originates at a defending mage. At impact, the after-snapshot becomes visible, including damage, sleeping mages, the next turn, and the result.

`Presentation::mages` and `missile_trail` sample the timeline; they do not integrate sprite motion or change entities. `advance` consumes crossed boundaries and returns ordered `Move`, `Pickup`, and `Impact` signals. The game screen handles those signals with sound, fireworks, hit flashes, and shake. Crossed signals fire once, including when frames are skipped. Multiple hits in a turn arrive together.

Gameplay input and AI wait for playback to finish; menus and undo remain available. Tutorial progression reads the presented game. Undo rewinds the rules immediately and queues the inverse snapshots after any playback already in progress. Each undone move glides back in reverse turn order, restoring health and pickups on landing without replaying attacks. Replacement lobby snapshots discard playback and particles. Readiness-only lobby updates preserve playback by comparing deterministic game history. Full game replacements snap to their authoritative state; ordinary turn messages animate.

To add an effect, derive its data from the accepted turn and snapshots, sample continuous visuals from time, and use a signal for one-shot effects. Keep timing out of `shared/`; do not add sleeps, browser callbacks per piece, or a second rules engine. Timing constants live together in `presentation.rs`.

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
