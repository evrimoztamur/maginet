# Why the remaining games draw

This diagnoses the existing 7,920-game campaign screen in each arm. It adds no
gameplay changes and runs no new AI balance simulation. The original screen and
its checksums remain unchanged. The overcharge feature remains in the experiment.

## Main finding

Most draws are inactivity adjudications in positions where damage remains
reachable. The current rule ends the game after eight damage-free plies once the
opening grace has elapsed, then compares total mana. Equal mana produces a draw.
Collecting a powerup does not reset that timer. The opening grace depends on the
original mage count, including sleepers, so some games have a longer quiet period.

| Draw ending | Before | With overcharge |
|---|---:|---:|
| Inactivity with equal mana | 625 | 620 |
| Both teams completely eliminated | 35 | 35 |
| No legal move, survivors on both teams | 6 | 7 |
| Total | 666 | 662 |

An exact search through legal moves, with the inactivity clock suppressed and all
other rules retained, found a path to damage in **619 of the 620** remaining
inactivity draws. In 614, the shortest path takes at most eight plies; five need
9–13 plies. One is permanently locked even with the clock removed. The equivalent
baseline count is eleven permanently locked inactivity draws. These are final
positions from different played lines, not ten guaranteed paired draw conversions.

These paths allow cooperation. They prove damage is possible, not that either
side can force it or profit from it. Damage includes shield retaliation and beam
friendly fire, consistent with what resets the existing clock. Nineteen of the
619 shortest witness paths end in damage only to the moving side. No path search
hit its depth or work limit in either arm.

## What happens before expiry

Among the 620 remaining inactivity draws:

- 188 have an immediately available damaging move when the clock expires; 184
  can immediately damage an enemy, and 177 can do so without taking damage on
  that same move. This says nothing about the opponent's following response.
- 255 passed up at least one enemy-damaging move during the final eight plies.
  Such a move might have been tactically losing; this is not a count of AI errors.
- 88 collected a powerup during those last eight plies. The board changed in a
  finite, meaningful way, but the inactivity clock kept running.
- 43 encountered some full position three times; 328 never repeated a full
  position even twice. Repetition alone does not explain most current draws.
- 380 had more than one survivor on at least one team; 240 were 1v1.
- 133 ended without any damage during the entire game.

These categories overlap. They are observations from these AI profiles and this
campaign sample, not estimates of human behaviour.

The engine also gives the agents a reason to avoid combat: a timeout is a win for
the side ahead in mana and a draw when tied. Search can prefer that result to a
losing exchange. Its nonterminal evaluation uses mana and distance from the board
centre; it does not directly reward approaching an attack or avoiding repetition.
Difficulty sampling can also select a lower-ranked move. The replay diagnosis
does not apportion causal responsibility among tactics, horizon limits and sampling.

## Recommended next comparison

Keep the conservative overcharge rule. It addresses a real but small category.
Test changes to inactivity separately, using the same paired campaign protocol:

1. Reset the inactivity clock on collecting a powerup as well as damage. There
   are finitely many pickups, so this cannot indefinitely refresh the clock.
2. Compare that variant with a sixteen-quiet-ply allowance: eight moves per side.
   Sixteen is a candidate to measure, not an established optimum.
3. Separately test an AI preference for nonrepeating positions among equally
   evaluated moves. Do not reward a losing attack merely to avoid a draw.

Measure draw rate, Red win rate, long-game percentiles and safety-cap frequency.
Changing this timer also changes mana-based timeout wins, so the balance effects
extend beyond games that currently draw. An exact repetition rule is cheap to
track, but adding a new repetition-to-draw rule could make draws easier to obtain.

Do not refresh the clock merely because an attack remains possible: that is true
in almost every remaining timeout draw and would permit endless evasion. Moving
closer is also reversible and does not necessarily help an attack pattern. If the
goal becomes eliminating most draws even under deliberate evasion, an overtime
objective or increasing pressure would change the incentives; a deadlock detector
alone cannot achieve that. Longer time alone may only prolong those games.

## Reproduce and inspect

From the experiment worktree:

```sh
cargo run --release -p generate --example diagnose_draws -- assessments/campaign-overcharge/before before > /tmp/maginet-draw-diagnosis-before.json
cargo run --release -p generate --example diagnose_draws -- assessments/campaign-overcharge/after after > /tmp/maginet-draw-diagnosis-after.json
```

The diagnostic replays every drawn game through the authoritative engine and
checks damage, outcome, turn count and overcharge activation. It probes actual
legal successors for damage opportunities and runs breadth-first search until the
first damage, with a limit of 64 plies and 50,000 tested moves per game. Every found
witness is replayed again from its root. Only state-graph exhaustion is labelled
impossible; depth and work cutoffs are explicitly inconclusive.

Position identity includes side to move, mages, mana, powers, remaining props,
sleepers, one-time overcharge consumption and the effective shield cache. Only
history and inactivity counters are excluded; fixed board geometry is shared by
all positions in a game's search. The clock is suppressed after later overcharge
events too. This is an offline diagnostic, not a new runtime detector or a solver
for adversarial forced wins.

[summary.json](summary.json) contains the counts and source/data hashes.
[before.json](before.json) and [after.json](after.json) contain compact per-game
diagnoses, snapshots and shortest damage witnesses. Scenario, profiles and trial
identify the original replay. Baseline gameplay is reproduced with the same
overcharge rule switch used by the original authoritative replay audit.
