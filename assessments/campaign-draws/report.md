# Inactivity and repetition: paired campaign assessment

Control retains deadlock overcharge. Rules adds pickup resets and sixteen quiet plies; All adds an AI preference for fewer past visits among equal root scores. Nothing is merged. This measures simulated agents, not human play.

The screen uses 30 identical seeded trials in each of 264 profile/scenario cells: 7,920 games per arm. Profiles retain depths 2/4/8, node caps 1,000/5,000/20,000, and existing ranked sampling. The safety cap is 200 plies. Tutorial inactivity remains disabled; the randomized scenario is excluded. Control reuses the completed overcharge screen unchanged.

## Full screen

| Metric | Control | Rules | All |
|---|---:|---:|---:|
| Red wins | 4534 | 4643 | 4671 |
| Red losses | 2724 | 2829 | 2847 |
| Draws | 662 | 448 | 402 |
| Safety-limit games | 0 | 0 | 0 |
| Draw rate | 8.36% | 5.66% | 5.08% |
| Elimination endings | 4366 | 5725 | 5832 |
| Immobilization endings | 60 | 72 | 70 |
| Inactivity endings | 3494 | 2123 | 2018 |
| Mean plies | 24.75 | 31.93 | 31.80 |
| Median plies | 24.0 | 29.0 | 29.0 |
| 95th percentile plies | 51 | 73 | 73 |
| 99th percentile plies | 61 | 92 | 93 |
| Longest game, plies | 94 | 142 | 143 |
| Overcharges | 34 | 42 | 43 |

| Comparison | Δ Red win pp | Δ draw pp | Draw→decisive | Decisive→draw | Changed outcomes |
|---|---:|---:|---:|---:|---:|
| rules_vs_control | +1.38 | -2.70 | 358 | 144 | 757 |
| all_vs_control | +1.73 | -3.28 | 403 | 143 | 853 |
| ai_vs_rules | +0.35 | -0.58 | 138 | 92 | 378 |

Draw transitions include any non-draw outcome; safety-limit counts are shown explicitly above. Aggregate profile mixtures are descriptive, not a human difficulty rating. Shared trial seeds across profiles also limit interpreting pooled paired p-values as independent evidence.

## Normal / Normal screen

W/L/D/U = Red wins / losses / draws / unresolved at the safety cap. Each row has 30 trials per arm; estimates are coarse.

| Battle | Control W/L/D/U | Rules W/L/D/U | All W/L/D/U | All Δ Red win pp |
|---|---|---|---|---:|
| Basics I | 30/0/0/0 | 30/0/0/0 | 30/0/0/0 | +0.0 |
| Basics II | 20/10/0/0 | 20/10/0/0 | 20/10/0/0 | +0.0 |
| Basics III | 26/4/0/0 | 26/4/0/0 | 26/4/0/0 | +0.0 |
| Basics IV | 29/1/0/0 | 28/2/0/0 | 29/1/0/0 | +0.0 |
| Patterns I | 19/8/3/0 | 19/8/3/0 | 19/9/2/0 | +0.0 |
| Patterns II | 18/8/4/0 | 21/8/1/0 | 20/9/1/0 | +6.7 |
| Patterns III | 21/3/6/0 | 22/4/4/0 | 20/7/3/0 | -3.3 |
| Diagonals I | 29/1/0/0 | 29/1/0/0 | 29/1/0/0 | +0.0 |
| Diagonals II | 13/12/5/0 | 16/12/2/0 | 16/13/1/0 | +10.0 |
| Diagonals III | 14/12/4/0 | 15/13/2/0 | 14/15/1/0 | +0.0 |
| Diagonals IV | 10/11/9/0 | 14/9/7/0 | 15/9/6/0 | +16.7 |
| Beams I | 19/11/0/0 | 19/11/0/0 | 19/11/0/0 | +0.0 |
| Beams II | 11/11/8/0 | 12/10/8/0 | 11/11/8/0 | +0.0 |
| Beams III | 12/15/3/0 | 11/16/3/0 | 13/16/1/0 | +3.3 |
| Shields I | 30/0/0/0 | 30/0/0/0 | 30/0/0/0 | +0.0 |
| Shields II | 5/23/2/0 | 5/23/2/0 | 5/23/2/0 | +0.0 |
| Shields III | 16/12/2/0 | 17/11/2/0 | 17/11/2/0 | +3.3 |
| Challenge I | 11/10/9/0 | 11/14/5/0 | 11/13/6/0 | +0.0 |
| Challenge II | 10/11/9/0 | 13/12/5/0 | 13/13/4/0 | +10.0 |
| Challenge III | 7/14/9/0 | 7/20/3/0 | 9/19/2/0 | +6.7 |
| Challenge IV | 11/10/9/0 | 12/12/6/0 | 13/13/4/0 | +6.7 |
| Rite I | 17/11/2/0 | 15/12/3/0 | 15/11/4/0 | -6.7 |
| Rite II | 18/12/0/0 | 18/12/0/0 | 18/12/0/0 | +0.0 |
| Rite III | 13/10/7/0 | 18/6/6/0 | 15/8/7/0 | +6.7 |
| Rite IV | 22/5/3/0 | 22/5/3/0 | 23/6/1/0 | +3.3 |
| Ascension I | 15/9/6/0 | 17/9/4/0 | 16/10/4/0 | +3.3 |
| Ascension II | 7/18/5/0 | 9/17/4/0 | 11/16/3/0 | +13.3 |
| Crossfire | 30/0/0/0 | 30/0/0/0 | 30/0/0/0 | +0.0 |
| Side Step | 21/3/6/0 | 22/3/5/0 | 25/2/3/0 | +13.3 |

## Preselected followups

Six battles were selected from the old screen for their inactivity-draw counts before looking at new results. Each receives 300 Normal/Normal trials per arm. First 30 trials must reproduce the screen exactly; only the remaining 270 enter confirmation tests. Holm adjustment covers win and draw tests for All versus Control and All versus Rules across all six battles (24 tests).

| Battle | Control W/L/D/U | Rules W/L/D/U | All W/L/D/U |
|---|---|---|---|
| Patterns II | 161/65/74/0 | 165/94/41/0 | 162/103/35/0 |
| Side Step | 203/10/87/0 | 225/20/55/0 | 244/16/40/0 |
| Challenge II | 108/111/81/0 | 126/120/54/0 | 129/119/52/0 |
| Challenge IV | 97/107/96/0 | 106/134/60/0 | 100/140/60/0 |
| Diagonals IV | 111/120/69/0 | 112/131/57/0 | 120/123/57/0 |
| Beams II | 122/117/61/0 | 139/110/51/0 | 141/114/45/0 |

| Battle | Comparison, new 270 trials | Δ Red win pp | Win Holm p | Δ draw pp | Draw Holm p |
|---|---|---:|---:|---:|---:|
| Patterns II | all_vs_control | -0.37 | 1.00000 | -13.33 | 0.00031 |
| Patterns II | ai_vs_rules | -0.74 | 1.00000 | -2.22 | 1.00000 |
| Side Step | all_vs_control | +13.70 | 0.00000 | -16.30 | 0.00000 |
| Side Step | ai_vs_rules | +5.93 | 0.29860 | -4.81 | 0.70547 |
| Challenge II | all_vs_control | +6.67 | 0.02719 | -8.89 | 0.00146 |
| Challenge II | ai_vs_rules | +1.11 | 1.00000 | -0.37 | 1.00000 |
| Challenge IV | all_vs_control | +0.37 | 1.00000 | -11.48 | 0.00743 |
| Challenge IV | ai_vs_rules | -2.59 | 1.00000 | +0.74 | 1.00000 |
| Diagonals IV | all_vs_control | +1.48 | 1.00000 | -3.33 | 1.00000 |
| Diagonals IV | ai_vs_rules | +2.59 | 1.00000 | +0.37 | 1.00000 |
| Beams II | all_vs_control | +7.04 | 0.34705 | -5.93 | 0.41434 |
| Beams II | ai_vs_rules | +1.11 | 1.00000 | -2.22 | 1.00000 |

## Reproduce

Freeze the generator and audit executables at the revisions in [experiment.json](experiment.json) into `target/draw-experiment/{control,rules,all}/`. Then run:

```sh
python3 scripts/run-draws.py screen
python3 scripts/run-draws.py followup --arms control rules all
python3 scripts/report-draws.py
```

[comparison.json](comparison.json) includes every profile/scenario cell, paired transitions, Wilson win intervals and telemetry. [interpretation.md](interpretation.md) explains practical effects and limitations. [validation.json](validation.json) records tests and replay audits. Raw checkpoints include full move, pickup, damage and overcharge traces. Frozen binaries are ignored build artifacts; exact source commits are retained.
