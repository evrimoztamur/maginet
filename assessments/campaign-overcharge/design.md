# Conservative deadlock detector

The detector proves absence of a first damage event. It does not solve for a forced
win, distinguish good defense from evasion, or predict what the sampled AI will do.

Each living mage has four 64-bit position sets, indexed by elapsed ply modulo four.
The current position starts in phase zero. The side to move alternates by phase.
On its own team's turn a mage can move to a cardinal neighbour; with another living
teammate it may also remain still. On the opposing turn its position is unchanged.
Four phases preserve the mandatory checkerboard rhythm of singleton teams: after
four plies each singleton has moved twice. A single union of all future positions
would erase this constraint and miss parity deadlocks.

Board edges, boulders, and sleeping mages block movement. Other living mages do not.
Every real position before the first damage is therefore included in the projected
sets, although the sets also contain mutually incompatible positions and moves.
The spell footprint of each possible moving destination is intersected with the
enemy positions at that phase. Any intersection stops the analysis as inconclusive.
If propagation reaches a fixed point without an intersection, no real first hit
can occur. There are only 256 membership bits per mage, and propagation is monotone.
The implementation never enumerates joint board arrangements or uses a ply cutoff.

Collectible pickups and held abilities are deliberately unsupported. Ignoring their
extra attacks or moves would make a negative result unsound. Boulders are supported
as static blockers. A sleeping mage's held ability has no effect and does not block
analysis. Absence of one team is treated as outside the detector's scope.

When enabled, the game checks at initialization and after each successful move.
It grants every survivor the existing diagonal rune when cardinal contact is
proven impossible and diagonal contact is possible in the same relaxed model.
This second result is not proof of a legal escape; it only rejects cases where
diagonals provably cannot help. No collectible items remain at activation, so the
rune cannot subsequently be replaced. The activation ply is serialized, included
in the search key, and reconstructed through undo. The inactivity clock resets once.
Existing no-legal-move terminals and games with stalemates disabled are unchanged.

The UI observes the presented activation state, not the ahead-of-animation rules
state. The green banner enters from beyond the left canvas edge in 15 wall-clock
frame units, holds for 60, and leaves rightward in 15. The application's clock runs
at 60 units per second independently of display refresh rate. Undo hides an undone
activation; redoing the triggering move announces it again. The banner does not
block movement or AI processing.

Validation includes all 3,600 combinations of two distinct squares on a 3×3 board,
25 spell pairings and both starting teams. Every negative proof is checked against
an exact joint-state BFS using authoritative movement with timeout and overcharge
disabled. Separate 2v2 corridor cases exercise geometry, obstacles, sleepers and
actual post-overcharge contact. Board-edge translation tests exhaust all 64 source
squares and offsets from −8 through +8 on both axes. Rule tests cover excluded
powerups, teammate waiting, future activation after elimination, serialization,
one-time activation and undo. Presentation and browser checks cover impact timing,
the requested banner motion and a real diagonal move.

Native timing data in `detector-benchmark.json` measures the current campaign roots
and 3×3/8×8 parity fixtures, using nine batches of 2,000 calls each. These are observed
fixture timings, not a worst-case guarantee or a WebAssembly benchmark.
