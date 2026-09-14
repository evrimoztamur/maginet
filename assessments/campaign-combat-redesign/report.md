# Combat-focused campaign redesign

Patterns I, Rite II, and Rite IV were redesigned to finish through combat more often. The cardinal map, 36 portals, tutorial, other battles, AI profiles, shared rules and 200-ply cap are unchanged. Human difficulty remains unvalidated.

The [candidate manifest](candidates.json) preserves all screened designs. Each screen has 30 trials in all nine matchups. Selected candidates receive 300 Normal/Normal trials using exactly the preceding assessment’s explicit seed namespace. The preceding 300-trial results are reused rather than rerun. Final first-30 trials overlap the screen and are not independent replications.

## Candidate screening

| Battle | Candidate | W/L/D/U (Normal/Normal) | Inactivity endings | Elimination endings | Selected |
|---|---|---|---:|---:|---|
| Patterns I | [close-patterns](screen/close-patterns/report.md) | 29/1/0/0 | 3 | 22 | no |
| Patterns I | [narrow-patterns](screen/narrow-patterns/report.md) | 28/2/0/0 | 1 | 29 | no |
| Rite II | [open-crossfire](screen/open-crossfire/report.md) | 16/12/2/0 | 2 | 28 | yes |
| Rite II | [narrow-crossfire](screen/narrow-crossfire/report.md) | 10/20/0/0 | 0 | 29 | no |
| Rite IV | [contested-center](screen/contested-center/report.md) | 24/5/1/0 | 7 | 22 | no |
| Rite IV | [single-shield](screen/single-shield/report.md) | 13/13/4/0 | 12 | 18 | no |
| Patterns I | [close-patterns-even](screen/close-patterns-even/report.md) | 19/7/4/0 | 6 | 20 | yes |
| Rite IV | [mobile-capstone](screen/mobile-capstone/report.md) | 20/9/1/0 | 3 | 24 | yes |

Elimination endings have no legal moves and at least one team at zero mana. Other no-legal-move endings can be immobilization, so they are not silently counted as elimination.

## Paired 300-trial followups

| Battle | Before W/L/D/U | After W/L/D/U | Inactivity before → after | Elimination before → after | Win rate before [95% CI] | Win rate after [95% CI] |
|---|---|---|---|---|---|---|
| Rite II | 96/116/88/0 | 166/126/8/0 | 176 → 19 | 123 → 279 | 32.0% [27.0, 37.5]; n=300; U=0; range 32.0–32.0% | 55.3% [49.7, 60.9]; n=300; U=0; range 55.3–55.3% |
| Patterns I | 212/70/18/0 | 204/79/17/0 | 125 → 51 | 158 → 220 | 70.7% [65.3, 75.5]; n=300; U=0; range 70.7–70.7% | 68.0% [62.5, 73.0]; n=300; U=0; range 68.0–68.0% |
| Rite IV | 139/101/60/0 | 207/76/17/0 | 137 → 49 | 159 → 234 | 46.3% [40.8, 52.0]; n=300; U=0; range 46.3–46.3% | 69.0% [63.6, 74.0]; n=300; U=0; range 69.0–69.0% |

Wilson intervals use resolved wins/(wins+losses+draws); unresolved bounds remain explicit. These are marginal intervals, not intervals for the paired difference. Candidate selection is exploratory and can overfit this seed namespace. No fixed win-rate threshold determines acceptance.

## Current full campaign matrices

Untouched cells are explicitly reused from the preceding combined assessment. Redesigned cells use 300 Normal/Normal trials and 30 for the other eight matchups. Each cell reports its actual sample count. Search telemetry remains in the linked dataset reports and combined JSON.

### Basics I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-0-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-0-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-1-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-1-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-00-2-2.json) |

### Basics II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/screen/basics-ii/matchup-00-0-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/screen/basics-ii/matchup-00-0-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-revision/screen/basics-ii/matchup-00-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/screen/basics-ii/matchup-00-1-0.json) | [85.3% [80.9, 88.9]; n=300; U=0; range 85.3–85.3%](../campaign-revision/final/basics-ii/matchup-00-1-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/screen/basics-ii/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/screen/basics-ii/matchup-00-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/screen/basics-ii/matchup-00-2-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/screen/basics-ii/matchup-00-2-2.json) |

### Basics III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7%](../campaign-revision/../campaign-seed-1/matchup-02-0-0.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-02-0-1.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-02-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/../campaign-seed-1/matchup-02-1-0.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/../campaign-seed-1/matchup-02-1-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-revision/../campaign-seed-1/matchup-02-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-02-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-02-2-1.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/../campaign-seed-1/matchup-02-2-2.json) |

### Basics IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/screen/basics-iv/matchup-00-0-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/screen/basics-iv/matchup-00-0-1.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/screen/basics-iv/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/basics-iv/matchup-00-1-0.json) | [94.3% [91.1, 96.4]; n=300; U=0; range 94.3–94.3%](../campaign-revision/final/basics-iv/matchup-00-1-1.json) | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/screen/basics-iv/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/screen/basics-iv/matchup-00-2-0.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/screen/basics-iv/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/basics-iv/matchup-00-2-2.json) |

### Patterns I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](screen/close-patterns-even/matchup-00-0-0.json) | [40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0%](screen/close-patterns-even/matchup-00-0-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](screen/close-patterns-even/matchup-00-0-2.json) |
| Normal | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](screen/close-patterns-even/matchup-00-1-0.json) | [68.0% [62.5, 73.0]; n=300; U=0; range 68.0–68.0%](final/close-patterns-even/matchup-00-1-1.json) | [26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7%](screen/close-patterns-even/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/close-patterns-even/matchup-00-2-0.json) | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](screen/close-patterns-even/matchup-00-2-1.json) | [56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7%](screen/close-patterns-even/matchup-00-2-2.json) |

### Patterns II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-revision/../campaign-seed-1/matchup-05-0-0.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-05-0-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-revision/../campaign-seed-1/matchup-05-0-2.json) |
| Normal | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/../campaign-seed-1/matchup-05-1-0.json) | [60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0%](../campaign-revision/../campaign-seed-1/matchup-05-1-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-revision/../campaign-seed-1/matchup-05-1-2.json) |
| Hard | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/../campaign-seed-1/matchup-05-2-0.json) | [66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7%](../campaign-revision/../campaign-seed-1/matchup-05-2-1.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-revision/../campaign-seed-1/matchup-05-2-2.json) |

### Patterns III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-revision/../campaign-seed-1/matchup-06-0-0.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-revision/../campaign-seed-1/matchup-06-0-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-revision/../campaign-seed-1/matchup-06-0-2.json) |
| Normal | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-revision/../campaign-seed-1/matchup-06-1-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/../campaign-seed-1/matchup-06-1-1.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-revision/../campaign-seed-1/matchup-06-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-06-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/../campaign-seed-1/matchup-06-2-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/../campaign-seed-1/matchup-06-2-2.json) |

### Diagonals I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-revision/screen/diagonals-i/matchup-00-0-0.json) | [66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7%](../campaign-revision/screen/diagonals-i/matchup-00-0-1.json) | [60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0%](../campaign-revision/screen/diagonals-i/matchup-00-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/screen/diagonals-i/matchup-00-1-0.json) | [86.7% [82.4, 90.1]; n=300; U=0; range 86.7–86.7%](../campaign-revision/final/diagonals-i/matchup-00-1-1.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](../campaign-revision/screen/diagonals-i/matchup-00-1-2.json) |
| Hard | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/screen/diagonals-i/matchup-00-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/screen/diagonals-i/matchup-00-2-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/screen/diagonals-i/matchup-00-2-2.json) |

### Diagonals II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-revision/../campaign-seed-1/matchup-08-0-0.json) | [20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0%](../campaign-revision/../campaign-seed-1/matchup-08-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-08-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-08-1-0.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-revision/../campaign-seed-1/matchup-08-1-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-08-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/../campaign-seed-1/matchup-08-2-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/../campaign-seed-1/matchup-08-2-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-revision/../campaign-seed-1/matchup-08-2-2.json) |

### Diagonals III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-09-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-09-0-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-09-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-09-1-0.json) | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-revision/../campaign-seed-1/matchup-09-1-1.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-revision/../campaign-seed-1/matchup-09-1-2.json) |
| Hard | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/../campaign-seed-1/matchup-09-2-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/../campaign-seed-1/matchup-09-2-1.json) | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-revision/../campaign-seed-1/matchup-09-2-2.json) |

### Diagonals IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-revision/../campaign-seed-1/matchup-10-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-10-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-10-0-2.json) |
| Normal | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/../campaign-seed-1/matchup-10-1-0.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-revision/../campaign-seed-1/matchup-10-1-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-revision/../campaign-seed-1/matchup-10-1-2.json) |
| Hard | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/../campaign-seed-1/matchup-10-2-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/../campaign-seed-1/matchup-10-2-1.json) | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-revision/../campaign-seed-1/matchup-10-2-2.json) |

### Beams I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-revision/screen/beams-i/matchup-00-0-0.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-revision/screen/beams-i/matchup-00-0-1.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-revision/screen/beams-i/matchup-00-0-2.json) |
| Normal | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/screen/beams-i/matchup-00-1-0.json) | [74.0% [68.8, 78.6]; n=300; U=0; range 74.0–74.0%](../campaign-revision/final/beams-i/matchup-00-1-1.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/screen/beams-i/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/screen/beams-i/matchup-00-2-0.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/screen/beams-i/matchup-00-2-1.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](../campaign-revision/screen/beams-i/matchup-00-2-2.json) |

### Beams II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-revision/../campaign-seed-1/matchup-12-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-12-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-12-0-2.json) |
| Normal | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/../campaign-seed-1/matchup-12-1-0.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-revision/../campaign-seed-1/matchup-12-1-1.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-12-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/../campaign-seed-1/matchup-12-2-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/../campaign-seed-1/matchup-12-2-1.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-revision/../campaign-seed-1/matchup-12-2-2.json) |

### Beams III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-13-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-13-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-13-0-2.json) |
| Normal | [56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7%](../campaign-revision/../campaign-seed-1/matchup-13-1-0.json) | [40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0%](../campaign-revision/../campaign-seed-1/matchup-13-1-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-13-1-2.json) |
| Hard | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/../campaign-seed-1/matchup-13-2-0.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/../campaign-seed-1/matchup-13-2-1.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-13-2-2.json) |

### Shields I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/shields-i/matchup-00-0-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/shields-i/matchup-00-0-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/shields-i/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/shields-i/matchup-00-1-0.json) | [100.0% [98.7, 100.0]; n=300; U=0; range 100.0–100.0%](../campaign-revision/final/shields-i/matchup-00-1-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/shields-i/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/shields-i/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/shields-i/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/screen/shields-i/matchup-00-2-2.json) |

### Shields II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-revision/../campaign-seed-1/matchup-15-0-0.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-revision/../campaign-seed-1/matchup-15-0-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-revision/../campaign-seed-1/matchup-15-0-2.json) |
| Normal | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-revision/../campaign-seed-1/matchup-15-1-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-15-1-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-revision/../campaign-seed-1/matchup-15-1-2.json) |
| Hard | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-15-2-0.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-revision/../campaign-seed-1/matchup-15-2-1.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-revision/../campaign-seed-1/matchup-15-2-2.json) |

### Shields III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-16-0-0.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-revision/../campaign-seed-1/matchup-16-0-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-revision/../campaign-seed-1/matchup-16-0-2.json) |
| Normal | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-revision/../campaign-seed-1/matchup-16-1-0.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-16-1-1.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-revision/../campaign-seed-1/matchup-16-1-2.json) |
| Hard | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/../campaign-seed-1/matchup-16-2-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/../campaign-seed-1/matchup-16-2-1.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-16-2-2.json) |

### Challenge I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-revision/../campaign-seed-1/matchup-17-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-17-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-17-0-2.json) |
| Normal | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-revision/../campaign-seed-1/matchup-17-1-0.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-revision/../campaign-seed-1/matchup-17-1-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-revision/../campaign-seed-1/matchup-17-1-2.json) |
| Hard | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/../campaign-seed-1/matchup-17-2-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/../campaign-seed-1/matchup-17-2-1.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-17-2-2.json) |

### Challenge II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-revision/../campaign-seed-1/matchup-18-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-18-0-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-revision/../campaign-seed-1/matchup-18-0-2.json) |
| Normal | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-revision/../campaign-seed-1/matchup-18-1-0.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-revision/../campaign-seed-1/matchup-18-1-1.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-revision/../campaign-seed-1/matchup-18-1-2.json) |
| Hard | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-18-2-0.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-revision/../campaign-seed-1/matchup-18-2-1.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-revision/../campaign-seed-1/matchup-18-2-2.json) |

### Challenge III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-revision/../campaign-seed-1/matchup-19-0-0.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-19-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-19-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-19-1-0.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-revision/../campaign-seed-1/matchup-19-1-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-19-1-2.json) |
| Hard | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/../campaign-seed-1/matchup-19-2-0.json) | [40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0%](../campaign-revision/../campaign-seed-1/matchup-19-2-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-19-2-2.json) |

### Challenge IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-revision/../campaign-seed-1/matchup-20-0-0.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-revision/../campaign-seed-1/matchup-20-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-20-0-2.json) |
| Normal | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-20-1-0.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-revision/../campaign-seed-1/matchup-20-1-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-revision/../campaign-seed-1/matchup-20-1-2.json) |
| Hard | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](../campaign-revision/../campaign-seed-1/matchup-20-2-0.json) | [60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0%](../campaign-revision/../campaign-seed-1/matchup-20-2-1.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-revision/../campaign-seed-1/matchup-20-2-2.json) |

### Rite I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-revision/../campaign-seed-1/matchup-21-0-0.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-21-0-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-21-0-2.json) |
| Normal | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/../campaign-seed-1/matchup-21-1-0.json) | [56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7%](../campaign-revision/../campaign-seed-1/matchup-21-1-1.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-revision/../campaign-seed-1/matchup-21-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-21-2-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-21-2-1.json) | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-revision/../campaign-seed-1/matchup-21-2-2.json) |

### Rite II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](screen/open-crossfire/matchup-00-0-0.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](screen/open-crossfire/matchup-00-0-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](screen/open-crossfire/matchup-00-0-2.json) |
| Normal | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](screen/open-crossfire/matchup-00-1-0.json) | [55.3% [49.7, 60.9]; n=300; U=0; range 55.3–55.3%](final/open-crossfire/matchup-00-1-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](screen/open-crossfire/matchup-00-1-2.json) |
| Hard | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](screen/open-crossfire/matchup-00-2-0.json) | [60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0%](screen/open-crossfire/matchup-00-2-1.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](screen/open-crossfire/matchup-00-2-2.json) |

### Rite III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-revision/../campaign-seed-1/matchup-23-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-23-0-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-23-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-23-1-0.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-revision/../campaign-seed-1/matchup-23-1-1.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-revision/../campaign-seed-1/matchup-23-1-2.json) |
| Hard | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/../campaign-seed-1/matchup-23-2-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-23-2-1.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-revision/../campaign-seed-1/matchup-23-2-2.json) |

### Rite IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](screen/mobile-capstone/matchup-00-0-0.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](screen/mobile-capstone/matchup-00-0-1.json) | [26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7%](screen/mobile-capstone/matchup-00-0-2.json) |
| Normal | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](screen/mobile-capstone/matchup-00-1-0.json) | [69.0% [63.6, 74.0]; n=300; U=0; range 69.0–69.0%](final/mobile-capstone/matchup-00-1-1.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](screen/mobile-capstone/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/mobile-capstone/matchup-00-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/mobile-capstone/matchup-00-2-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](screen/mobile-capstone/matchup-00-2-2.json) |

### Ascension I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-revision/../campaign-seed-1/matchup-25-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-25-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-revision/../campaign-seed-1/matchup-25-0-2.json) |
| Normal | [66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7%](../campaign-revision/../campaign-seed-1/matchup-25-1-0.json) | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-revision/../campaign-seed-1/matchup-25-1-1.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-revision/../campaign-seed-1/matchup-25-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/../campaign-seed-1/matchup-25-2-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/../campaign-seed-1/matchup-25-2-1.json) | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-revision/../campaign-seed-1/matchup-25-2-2.json) |

### Ascension II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0%](../campaign-revision/../campaign-seed-1/matchup-26-0-0.json) | [26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7%](../campaign-revision/../campaign-seed-1/matchup-26-0-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-revision/../campaign-seed-1/matchup-26-0-2.json) |
| Normal | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-revision/../campaign-seed-1/matchup-26-1-0.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-revision/../campaign-seed-1/matchup-26-1-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-revision/../campaign-seed-1/matchup-26-1-2.json) |
| Hard | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-revision/../campaign-seed-1/matchup-26-2-0.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-revision/../campaign-seed-1/matchup-26-2-1.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-revision/../campaign-seed-1/matchup-26-2-2.json) |

### Junction I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/junction-screen/junction-i/matchup-00-0-0.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](../campaign-revision/junction-screen/junction-i/matchup-00-0-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/junction-screen/junction-i/matchup-00-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/junction-screen/junction-i/matchup-00-1-0.json) | [88.0% [83.8, 91.2]; n=300; U=0; range 88.0–88.0%](../campaign-revision/junction-final/junction-i/matchup-00-1-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/junction-screen/junction-i/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-i/matchup-00-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-i/matchup-00-2-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-i/matchup-00-2-2.json) |

### Junction II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/junction-screen/junction-ii/matchup-00-0-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/junction-screen/junction-ii/matchup-00-0-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-revision/junction-screen/junction-ii/matchup-00-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/junction-screen/junction-ii/matchup-00-1-0.json) | [75.7% [70.5, 80.2]; n=300; U=0; range 75.7–75.7%](../campaign-revision/junction-final/junction-ii/matchup-00-1-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/junction-screen/junction-ii/matchup-00-1-2.json) |
| Hard | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/junction-screen/junction-ii/matchup-00-2-0.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](../campaign-revision/junction-screen/junction-ii/matchup-00-2-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/junction-screen/junction-ii/matchup-00-2-2.json) |

### Junction III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iii/matchup-00-0-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iii/matchup-00-0-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iii/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iii/matchup-00-1-0.json) | [100.0% [98.7, 100.0]; n=300; U=0; range 100.0–100.0%](../campaign-revision/junction-final/junction-iii/matchup-00-1-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iii/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iii/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iii/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iii/matchup-00-2-2.json) |

### Junction IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/junction-screen/junction-iv/matchup-00-0-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-revision/junction-screen/junction-iv/matchup-00-0-1.json) | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-revision/junction-screen/junction-iv/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-iv/matchup-00-1-0.json) | [88.7% [84.6, 91.8]; n=300; U=0; range 88.7–88.7%](../campaign-revision/junction-final/junction-iv/matchup-00-1-1.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/junction-screen/junction-iv/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-iv/matchup-00-2-0.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/junction-screen/junction-iv/matchup-00-2-1.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/junction-screen/junction-iv/matchup-00-2-2.json) |

### Junction V

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-v/matchup-00-0-0.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](../campaign-revision/junction-screen/junction-v/matchup-00-0-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/junction-screen/junction-v/matchup-00-0-2.json) |
| Normal | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-v/matchup-00-1-0.json) | [84.7% [80.2, 88.3]; n=300; U=0; range 84.7–84.7%](../campaign-revision/junction-final/junction-v/matchup-00-1-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/junction-screen/junction-v/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-v/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-v/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-v/matchup-00-2-2.json) |

### Junction VI

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-revision/junction-screen/junction-vi/matchup-00-0-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/junction-screen/junction-vi/matchup-00-0-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/junction-screen/junction-vi/matchup-00-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/junction-screen/junction-vi/matchup-00-1-0.json) | [95.3% [92.3, 97.2]; n=300; U=0; range 95.3–95.3%](../campaign-revision/junction-final/junction-vi/matchup-00-1-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-vi/matchup-00-1-2.json) |
| Hard | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/junction-screen/junction-vi/matchup-00-2-0.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/junction-screen/junction-vi/matchup-00-2-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-vi/matchup-00-2-2.json) |

### Junction VII

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-vii/matchup-00-0-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-vii/matchup-00-0-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-vii/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-vii/matchup-00-1-0.json) | [100.0% [98.7, 100.0]; n=300; U=0; range 100.0–100.0%](../campaign-revision/junction-final/junction-vii/matchup-00-1-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-vii/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-vii/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-vii/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-vii/matchup-00-2-2.json) |

### Junction VIII

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-revision/junction-screen/junction-viii/matchup-00-0-0.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](../campaign-revision/junction-screen/junction-viii/matchup-00-0-1.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-revision/junction-screen/junction-viii/matchup-00-0-2.json) |
| Normal | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-viii/matchup-00-1-0.json) | [92.3% [88.8, 94.8]; n=300; U=0; range 92.3–92.3%](../campaign-revision/junction-final/junction-viii/matchup-00-1-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-revision/junction-screen/junction-viii/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-viii/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/junction-screen/junction-viii/matchup-00-2-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/junction-screen/junction-viii/matchup-00-2-2.json) |

### Tutorial

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-revision/../campaign-seed-1/matchup-27-0-0.json) | — | — |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-revision/../campaign-seed-1/matchup-27-1-0.json) | — | — |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-revision/../campaign-seed-1/matchup-27-2-0.json) | — | — |

## Main route

Normal/Normal comparisons; ≥20-point drops are exploratory flags, supported only by disjoint intervals and ≤10% unresolved at both endpoints. No multiple-comparison correction. Directed shortest distances are context.

| Edge | Distances | Red win drop | Flag |
|---|---|---:|---|
| Basics I → Basics II | 1 → 2 | +14.7 pp | below threshold |
| Basics II → Basics III | 2 → 3 | -1.3 pp | below threshold |
| Basics III → Basics IV | 3 → 4 | -7.7 pp | below threshold |
| Basics IV → Patterns I | 4 → 5 | +26.3 pp | supported candidate |
| Patterns I → Patterns II | 5 → 6 | +8.0 pp | below threshold |
| Patterns II → Patterns III | 6 → 7 | -10.0 pp | below threshold |
| Patterns III → Diagonals I | 7 → 8 | -16.7 pp | below threshold |
| Diagonals I → Beams I | 8 → 9 | +12.7 pp | below threshold |
| Beams I → Shields I | 9 → 10 | -26.0 pp | below threshold |
| Shields I → Junction I | 10 → 11 | +12.0 pp | below threshold |
| Junction I → Rite I | 11 → 12 | +31.3 pp | supported candidate |
| Rite I → Rite II | 12 → 13 | +1.3 pp | below threshold |
| Rite II → Rite III | 13 → 14 | +12.0 pp | below threshold |
| Rite III → Rite IV | 14 → 15 | -25.7 pp | below threshold |
| Rite IV → Ascension I | 15 → 16 | +19.0 pp | below threshold |
| Ascension I → Ascension II | 16 → 17 | +26.7 pp | followup candidate |

## Optional routes

Normal/Normal comparisons; ≥20-point drops are exploratory flags, supported only by disjoint intervals and ≤10% unresolved at both endpoints. No multiple-comparison correction. Directed shortest distances are context.

| Edge | Distances | Red win drop | Flag |
|---|---|---:|---|
| Diagonals I → Diagonals II | 8 → 9 | +43.3 pp | supported candidate |
| Diagonals II → Diagonals III | 9 → 10 | -3.3 pp | below threshold |
| Diagonals III → Beams I | 10 → 9 | -27.3 pp | below threshold |
| Beams I → Diagonals IV | 9 → 10 | +40.7 pp | supported candidate |
| Diagonals IV → Beams II | 10 → 11 | -3.3 pp | below threshold |
| Beams II → Beams III | 11 → 12 | -3.3 pp | below threshold |
| Beams III → Challenge II | 12 → 13 | +6.7 pp | below threshold |
| Challenge II → Shields I | 13 → 10 | -66.7 pp | below threshold |
| Shields I → Junction I | 10 → 11 | +12.0 pp | below threshold |
| Junction I → Challenge I | 11 → 12 | +51.3 pp | supported candidate |
| Challenge I → Rite II | 12 → 13 | -18.7 pp | below threshold |
| Shields I → Junction I | 10 → 11 | +12.0 pp | below threshold |
| Junction I → Challenge III | 11 → 12 | +64.7 pp | supported candidate |
| Challenge III → Junction II | 12 → 13 | -52.3 pp | below threshold |
| Junction II → Junction III | 13 → 14 | -24.3 pp | below threshold |
| Junction III → Junction IV | 14 → 15 | +11.3 pp | below threshold |
| Junction IV → Junction V | 15 → 16 | +4.0 pp | below threshold |
| Junction V → Rite IV | 16 → 15 | +15.7 pp | below threshold |
| Shields I → Shields II | 10 → 11 | +83.3 pp | supported candidate |
| Shields II → Shields III | 11 → 12 | -36.7 pp | below threshold |
| Shields III → Challenge IV | 12 → 13 | +16.7 pp | below threshold |
| Challenge IV → Junction VI | 13 → 14 | -58.7 pp | below threshold |
| Junction VI → Junction VII | 14 → 15 | -4.7 pp | below threshold |
| Junction VII → Junction VIII | 15 → 16 | +7.7 pp | below threshold |
| Junction VIII → Rite IV | 16 → 15 | +23.3 pp | supported candidate |

See [interpretation and replay findings](interpretation.md) for design decisions and remaining limitations.
