# Deadlock overcharge: paired campaign assessment

Simulated agents, not human playtesting. Red is the player. All before/after pairs use identical current scenarios, seeds, profile budgets and trial indices. Historical assessments are unchanged.

Across 7,920 paired games per arm, draws change from **666 to 662**, and Red wins from **4,532 to 4,534**. Overcharge activates in 34 games. The [interpretation](interpretation.md) discusses the size of the effect, followup evidence and coverage limits.

The screen covers 264 matchup cells and 7,920 games per ruleset. Each cell uses 30 trials, seed 1 and a 200-ply safety cap. Easy/Normal/Hard use depths 2/4/8, node caps 1,000/5,000/20,000 and the existing ranked selection probabilities. Tutorial remains Easy-opponent with inactivity and overcharge disabled. Randomized Ascension III is excluded, as in the earlier fixed-scenario analyses.

The rule proves absence of damage using four-phase independent position bitboards, ignoring living-piece collisions. It excludes any remaining collectible or held ability. It grants every survivor a diagonal rune only when cardinal contact is impossible and diagonal contact is possible in the abstraction. It runs at initialization and after moves, before inactivity adjudication; already terminal no-move positions remain terminal. Overcharge resets the inactivity counter once. The ordinary timeout and mana tiebreak remain in force.

## Full screen

| Metric | Before | After |
|---|---:|---:|
| Red wins | 4532 | 4534 |
| Red losses | 2722 | 2724 |
| Draws | 666 | 662 |
| Safety-limit games | 0 | 0 |
| Elimination endings | 4361 | 4366 |
| Immobilization endings | 59 | 60 |
| Inactivity endings | 3500 | 3494 |
| SafetyLimit endings | 0 | 0 |
| Mean plies | 24.74 | 24.75 |
| Actual overcharge activations | 0 | 34 |

Paired outcome changes: 16. Actual activation survivor counts: `{2: 34}`. Aggregate totals mix all difficulty profiles and should not be read as a human difficulty rating.

## Normal / Normal screen

W/L/D/U means Red wins, losses, draws and safety-limit unresolved games. Win percentages use all trials; unresolved counts are shown explicitly. p is the exact paired McNemar test for a Red win versus any other outcome, unadjusted and exploratory. Thirty trials give coarse estimates.

| Battle | N per arm | Before W/L/D/U | After W/L/D/U | Red wins | Δ pp | Overcharges | Paired p |
|---|---:|---|---|---|---:|---:|---:|
| Basics I | 30 | 30/0/0/0 | 30/0/0/0 | 100.0% → 100.0% | +0.0 | 0 | 1.0000 |
| Basics II | 30 | 20/10/0/0 | 20/10/0/0 | 66.7% → 66.7% | +0.0 | 0 | 1.0000 |
| Basics III | 30 | 26/4/0/0 | 26/4/0/0 | 86.7% → 86.7% | +0.0 | 0 | 1.0000 |
| Basics IV | 30 | 29/1/0/0 | 29/1/0/0 | 96.7% → 96.7% | +0.0 | 0 | 1.0000 |
| Patterns I | 30 | 19/8/3/0 | 19/8/3/0 | 63.3% → 63.3% | +0.0 | 0 | 1.0000 |
| Patterns II | 30 | 18/8/4/0 | 18/8/4/0 | 60.0% → 60.0% | +0.0 | 0 | 1.0000 |
| Patterns III | 30 | 21/3/6/0 | 21/3/6/0 | 70.0% → 70.0% | +0.0 | 0 | 1.0000 |
| Diagonals I | 30 | 29/1/0/0 | 29/1/0/0 | 96.7% → 96.7% | +0.0 | 0 | 1.0000 |
| Diagonals II | 30 | 13/12/5/0 | 13/12/5/0 | 43.3% → 43.3% | +0.0 | 0 | 1.0000 |
| Diagonals III | 30 | 14/12/4/0 | 14/12/4/0 | 46.7% → 46.7% | +0.0 | 0 | 1.0000 |
| Diagonals IV | 30 | 10/11/9/0 | 10/11/9/0 | 33.3% → 33.3% | +0.0 | 0 | 1.0000 |
| Beams I | 30 | 19/11/0/0 | 19/11/0/0 | 63.3% → 63.3% | +0.0 | 0 | 1.0000 |
| Beams II | 30 | 11/11/8/0 | 11/11/8/0 | 36.7% → 36.7% | +0.0 | 0 | 1.0000 |
| Beams III | 30 | 12/15/3/0 | 12/15/3/0 | 40.0% → 40.0% | +0.0 | 0 | 1.0000 |
| Shields I | 30 | 30/0/0/0 | 30/0/0/0 | 100.0% → 100.0% | +0.0 | 0 | 1.0000 |
| Shields II | 30 | 5/23/2/0 | 5/23/2/0 | 16.7% → 16.7% | +0.0 | 0 | 1.0000 |
| Shields III | 30 | 16/12/2/0 | 16/12/2/0 | 53.3% → 53.3% | +0.0 | 0 | 1.0000 |
| Challenge I | 30 | 11/10/9/0 | 11/10/9/0 | 36.7% → 36.7% | +0.0 | 0 | 1.0000 |
| Challenge II | 30 | 10/11/9/0 | 10/11/9/0 | 33.3% → 33.3% | +0.0 | 0 | 1.0000 |
| Challenge III | 30 | 7/14/9/0 | 7/14/9/0 | 23.3% → 23.3% | +0.0 | 0 | 1.0000 |
| Challenge IV | 30 | 11/10/9/0 | 11/10/9/0 | 36.7% → 36.7% | +0.0 | 0 | 1.0000 |
| Rite I | 30 | 17/11/2/0 | 17/11/2/0 | 56.7% → 56.7% | +0.0 | 0 | 1.0000 |
| Rite II | 30 | 18/12/0/0 | 18/12/0/0 | 60.0% → 60.0% | +0.0 | 0 | 1.0000 |
| Rite III | 30 | 13/10/7/0 | 13/10/7/0 | 43.3% → 43.3% | +0.0 | 0 | 1.0000 |
| Rite IV | 30 | 22/5/3/0 | 22/5/3/0 | 73.3% → 73.3% | +0.0 | 0 | 1.0000 |
| Ascension I | 30 | 15/9/6/0 | 15/9/6/0 | 50.0% → 50.0% | +0.0 | 0 | 1.0000 |
| Ascension II | 30 | 7/18/5/0 | 7/18/5/0 | 23.3% → 23.3% | +0.0 | 0 | 1.0000 |
| Crossfire | 30 | 30/0/0/0 | 30/0/0/0 | 100.0% → 100.0% | +0.0 | 0 | 1.0000 |
| Side Step | 30 | 21/3/6/0 | 21/3/6/0 | 70.0% → 70.0% | +0.0 | 0 | 1.0000 |

## Followups

Affected scenarios receive 300 paired Normal/Normal trials. The first 30 exactly reproduce the screen and are not independent evidence. The remaining 270 provide a separate confirmation sample; Holm adjustment covers both win-rate and draw-rate tests across all selected followups.

| Battle | N per arm | Before W/L/D/U | After W/L/D/U | Red wins | Δ pp | Overcharges | Paired p |
|---|---:|---|---|---|---|---:|---:|
| Basics III | 300 | 254/45/1/0 | 254/45/1/0 | 84.7% → 84.7% | +0.0 | 0 | 1.0000 |
| Basics IV | 300 | 280/16/4/0 | 282/15/3/0 | 93.3% → 94.0% | +0.7 | 3 | 0.5000 |
| Crossfire | 300 | 281/8/11/0 | 282/9/9/0 | 93.7% → 94.0% | +0.3 | 3 | 1.0000 |
| Diagonals IV | 300 | 111/121/68/0 | 111/120/69/0 | 37.0% → 37.0% | +0.0 | 1 | 1.0000 |
| Patterns I | 300 | 198/81/21/0 | 202/84/14/0 | 66.0% → 67.3% | +1.3 | 17 | 0.2891 |
| Patterns III | 300 | 219/49/32/0 | 218/51/31/0 | 73.0% → 72.7% | -0.3 | 1 | 1.0000 |

| Battle | New 270-trial Δ win pp | Win Holm p | Δ draw pp | Draw Holm p |
|---|---:|---:|---:|---:|
| Basics III | +0.0 | 1.00000 | +0.0 | 1.00000 |
| Basics IV | +0.7 | 1.00000 | -0.4 | 1.00000 |
| Crossfire | +0.4 | 1.00000 | -0.7 | 1.00000 |
| Diagonals IV | +0.0 | 1.00000 | +0.4 | 1.00000 |
| Patterns I | +1.5 | 1.00000 | -2.6 | 0.46875 |
| Patterns III | -0.4 | 1.00000 | -0.4 | 1.00000 |

## Coverage and reproducibility

Screen-selected followup IDs: basics-iii, basics-iv, crossfire, diagonals-iv, patterns-i, patterns-iii.

The complete nine-profile comparisons, outcome transition counts, Wilson intervals, node/fallback telemetry, and activation counts are in [comparison.json](comparison.json). Each arm retains metadata, engine fingerprints, per-trial checkpoints, complete move/pickup/damage replays, and the standard campaign report. [experiment.json](experiment.json) records source identity and commands. [detector-benchmark.json](detector-benchmark.json) records native timing batches. [interpretation.md](interpretation.md) discusses findings and limitations.

Run `python3 scripts/report-overcharge.py` after completing both arms and followups. Reproduction must use the baseline snapshot for the before binary and the experiment rules for the after binary. Do not mix checkpoints across engine fingerprints.
