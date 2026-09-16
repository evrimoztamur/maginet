# Campaign teaching and mana assessment

Three Basics battles follow the tutorial: recognize a two-target attack, coordinate different patterns, then reject a greedy attack that loses a mage. The original Basics I is removed because it removes the need to consider an effective enemy response. The [design](design.md) records the actual decisions and rejected alternatives.

## Matched-seed Normal / Normal followups

Each side has 300 trials with the combined draw rules and AI held constant. Red is the player. W/L/D/U means win/loss/draw/unresolved at the 200-ply cap. The new finale is compared with old Basics IV; new I with old II, and new II with old III. Other rows compare the same named battle.

| New battle | Previous W/L/D/U | Revised W/L/D/U | Red win rate | Δ win pp |
|---|---|---|---|---:|
| basics-i | 218/82/0/0 | 218/82/0/0 | 72.7% → 72.7% | +0.0 |
| basics-ii | 256/44/0/0 | 177/121/2/0 | 85.3% → 59.0% | -26.3 |
| diagonals-i | 283/17/0/0 | 158/142/0/0 | 94.3% → 52.7% | -41.7 |
| basics-iii | 286/11/3/0 | 210/88/2/0 | 95.3% → 70.0% | -25.3 |
| shields-i | 300/0/0/0 | 252/45/3/0 | 100.0% → 84.0% | -16.0 |
| patterns-i | 199/91/10/0 | 185/106/9/0 | 66.3% → 61.7% | -4.7 |
| patterns-iii | 218/59/23/0 | 147/126/27/0 | 72.7% → 49.0% | -23.7 |
| rite-iv | 210/76/14/0 | 158/117/25/0 | 70.0% → 52.7% | -17.3 |

## Separate 270-trial followups

Candidates were explored using 30 trials and exact opening analysis. Every followup reproduces those first 30 trials exactly. Results below exclude them. Holm adjustment covers win and draw metrics for seven changed battles (14 tests). Repositioned Basics I is unchanged and excluded from these tests. This is evidence about these simulated agents, not a human learning curve.

| Battle | Δ win pp | Win Holm p | Δ draw pp | Draw Holm p |
|---|---:|---:|---:|---:|
| basics-ii | -26.7 | 0.00000 | +0.7 | 1.00000 |
| diagonals-i | -42.6 | 0.00000 | +0.0 | 1.00000 |
| basics-iii | -25.6 | 0.00000 | -0.4 | 1.00000 |
| shields-i | -16.3 | 0.00000 | +1.1 | 1.00000 |
| patterns-i | -4.8 | 1.00000 | +0.4 | 1.00000 |
| patterns-iii | -25.2 | 0.00000 | +2.2 | 1.00000 |
| rite-iv | -16.3 | 0.00144 | +3.3 | 1.00000 |

## Final catalogue screen

The full revised catalogue contains 29 battles plus the tutorial. Excluding randomized Ascension III, the screen covers 255 profile/scenario cells and 7,650 games: 4088/3158/404/0 W/L/D/U. All nine Easy/Normal/Hard pairings receive 30 trials; the tutorial retains its Easy opponent. Mean length is 32.25 plies.

All 22 unchanged fixed scenarios reproduce the previous combined-rules screen exactly, including the moved Basics I. Changed level codes intentionally get their normal production seed namespaces here; this full-catalogue screen is not a paired causal comparison. Use the matched-seed followups above to compare revisions.

| Current battle | Normal / Normal W/L/D/U |
|---|---|
| basics-i | 20/10/0/0 |
| basics-ii | 19/11/0/0 |
| basics-iii | 19/11/0/0 |
| patterns-i | 13/14/3/0 |
| patterns-ii | 20/9/1/0 |
| patterns-iii | 19/11/0/0 |
| diagonals-i | 14/16/0/0 |
| diagonals-ii | 16/13/1/0 |
| diagonals-iii | 14/15/1/0 |
| diagonals-iv | 15/9/6/0 |
| beams-i | 19/11/0/0 |
| beams-ii | 11/11/8/0 |
| beams-iii | 13/16/1/0 |
| shields-i | 22/8/0/0 |
| shields-ii | 5/23/2/0 |
| shields-iii | 17/11/2/0 |
| challenge-i | 11/13/6/0 |
| challenge-ii | 13/13/4/0 |
| challenge-iii | 9/19/2/0 |
| challenge-iv | 13/13/4/0 |
| rite-i | 15/11/4/0 |
| rite-ii | 18/12/0/0 |
| rite-iii | 15/8/7/0 |
| rite-iv | 16/10/4/0 |
| ascension-i | 16/10/4/0 |
| ascension-ii | 11/16/3/0 |
| crossfire | 30/0/0/0 |
| side-step | 25/2/3/0 |

## Reproduce

Use the frozen combined-rules generator at the revision in [experiment.json](experiment.json) for candidates, then the campaign revision for the catalogue screen.

```sh
python3 scripts/run-pedagogy.py screen
python3 scripts/run-pedagogy.py followup
cargo run --release -p generate -- campaign --games 30 --seed 1 --max-plies 200 --workers 4 --replays --output assessments/campaign-pedagogy/campaign
python3 scripts/report-pedagogy.py
```

[comparison.json](comparison.json) retains every candidate screen, followup, transition, Wilson win interval and final catalogue cell. [validation.json](validation.json) records tests, browser checks and authoritative replay audits. [design.md](design.md) explains the teaching choices and remaining playtest risks.
