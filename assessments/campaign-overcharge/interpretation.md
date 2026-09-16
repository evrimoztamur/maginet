# Findings and limits

This experiment tests a conservative structural-deadlock rule, using the campaign
as it existed in the working checkout at the start of the task. Both arms use that
same catalogue. It does not compare the changed rules against outdated scenario
codes from an earlier campaign assessment. The earlier deterministic profiles,
seed convention, 30-trial difficulty matrix and 200-ply safety cap are retained.

Across 7,920 screen games per arm, draws fall from 666 to 662 (8.409% to 8.359%).
Red wins change from 4,532 to 4,534, and only 16 paired outcomes change. Overcharge
activates in 34 games, all with two survivors. Inactivity endings fall from 3,500
to 3,494, while elimination endings rise from 4,361 to 4,366. Neither arm hits the
200-ply safety cap. The aggregate paired draw test is p=0.3877, and the paired Red
win test is p=0.7905. The overall effect is small and is not statistically clear.

This is a fast, targeted fix for proven locks. The campaign results do not support
presenting it as a substantial reduction in draws generally.

## Balance followups

Six scenarios were selected because a screen trial changed outcome or actually
activated overcharge. Each received 300 Normal/Normal trials per arm. All six
first-30 trial sets reproduce their corresponding screen cells exactly.

| Battle | Red wins before → after | Draws before → after | Actual activations |
|---|---:|---:|---:|
| Basics III | 254 → 254 | 1 → 1 | 0 |
| Basics IV | 280 → 282 | 4 → 3 | 3 |
| Patterns I | 198 → 202 | 21 → 14 | 17 |
| Patterns III | 219 → 218 | 32 → 31 | 1 |
| Diagonals IV | 111 → 111 | 68 → 69 | 1 |
| Crossfire | 281 → 282 | 11 → 9 | 3 |

The largest observed Red win-rate change is +1.33 percentage points in Patterns I.
No selected battle shows a statistically clear Red win-rate change. This is not an
equivalence test, and does not establish that the rule has no balance effect.

Patterns I provides the strongest indication of benefit: draws fall from 7.0% to
4.7%. In the independent 270-trial portion, eight old draws become decisive and one
old decisive result becomes a draw. The unadjusted paired draw test is p=0.0391;
Holm adjustment across the twelve win/draw tests for the six selected scenarios gives
p=0.4688. The reduction is promising but is not statistically confirmed after this
adjustment. Small changes in the other scenarios should not be treated as reliable
difficulty shifts. The same seeds are paired, not independent before/after samples.

## What changed in actual play

The following zero-based trial indices refer to the Patterns I Normal/Normal
followup files. Full replays and seeds are retained in each arm's checkpoint.

- Trial 31: the old game draws by inactivity after 27 plies with 2/2 mana. The new
  game overcharges its two survivors at ply 26 and ends in a Red elimination win at
  ply 35 with 1/0 mana.
- Trial 181: an old 27-ply inactivity draw becomes a Blue elimination win at ply 32
  after overcharge at ply 23. The rule does not simply give the player the old draw.
- Trial 200: overcharge activates at ply 22, but the game still ends by inactivity
  at ply 30, now awarding Blue the win on 3/4 mana. More legal movement does not
  guarantee elimination.
- Trial 226: overcharge activates at ply 22 and the game still draws by inactivity
  at ply 30. The ordinary evasion problem remains.
- Trial 43: the outcome changes from a draw to a Red elimination win even though
  overcharge never activates in the played line. Search evaluates the new rule in
  hypothetical continuations, so its choices can change before an actual trigger.

## Coverage

At the end of the 7,920 baseline screen games, 3,500 games ended by inactivity. The
detector classifies 28 of those final positions as impossible contact, 1,039 as
possible contact in its relaxed model, and 2,433 as unsupported because of held
abilities or remaining pickups. These are final-position diagnostics, not an exact
count of all structural deadlocks: possible and unsupported are both inconclusive.

The implementation supports any number of mages and has verified 2v2 geometry
fixtures. Actual activation survivor counts are reported separately in the screen
and followup data; synthetic multi-mage coverage must not be confused with observed
campaign benefit. All 34 screen activations and all 25 followup activations occur
with two survivors; this campaign sample demonstrates no multi-mage benefit.

The scope deliberately excludes existing diagonal movement, beams, shields and
remaining collectible runes from the proof. This protects powerup-dependent
teaching scenarios but limits coverage. It also leaves existing immobilization
endings unchanged. It rejects positions where even relaxed diagonal movement cannot
produce contact. If the goal is a large reduction in all draws, the next experiment
would need a broader powerup-aware detector or a separate rule for inactivity and
evasion, with another paired balance comparison.

## Performance and validation

On the machine recorded in `experiment.json`, the 3×3 parity proof has a median
0.36 µs per call and the 8×8 parity proof 0.79 µs across nine batches of 2,000 calls.
The largest median among all tested fixtures is 0.79 µs. These are native timings
under concurrent simulation load, not browser or worst-case guarantees. The AI
simulations use deterministic node limits, so machine load does not change results.

Workspace tests, exact small-board proof checks, the WebAssembly build, the mobile
feature compile check and the canvas banner check pass. The browser check records
the green banner's actual 250/1000/250 ms sequence and a legal diagonal move. The
presentation test verifies that activation appears at the hit's displayed impact,
and undo restores the preceding state. Each completed dataset has an authoritative
replay audit checking move legality, pickups, damage, mana, activation, seeds and
outcomes; the standard campaign audit checks aggregate counts and search budgets.

The branch remains an isolated experiment. The original checkout was not changed.
See [report.md](report.md) for full screen totals and all difficulty comparisons,
[design.md](design.md) for the proof and limitations, and
[deadlock-banner.png](deadlock-banner.png) for the banner screenshot.
