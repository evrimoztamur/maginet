# Revised campaign assessment

Simulated-agent evidence; human difficulty remains unvalidated. Red is the player. All runs retain the original profiles, seed 1 and 200-ply cap. Intervals are nominal 95% Wilson intervals; draws are resolved non-wins. Ranges bound unresolved outcomes.

Eight revised scenarios and eight new 1v1 junction battles have 30 trials in all nine matchups, then 300 Normal/Normal trials. The first 30 final trials overlap the screen and are not independent replications. The new junctions have no original counterparts. Original versions of the eight revised scenarios have matching 300-trial Normal/Normal followups. Every untouched matchup reuses the immutable initial survey (30 trials); no untouched scenario is rerun.

The [map](map.svg) shows the full layout. The [graph](graph.json) is exported from the shared UI/analyser catalogue. The [scenario manifest](scenarios.json) records original and revised codes. Each revised run explicitly uses the original canonical code as its seed namespace. Individual dataset metadata records graph, scenario, configuration, namespace, replay policy, and engine fingerprint. The eight earlier scenario datasets retain their run-time layout metadata; their simulations are reused after the cardinal-layout change because positions and connections do not affect battle simulation. The exported graph is the current progression authority. Every one of its 40 links joins cardinal-neighbour battle cells, including eight new 1v1 junctions, for 36 portals in total.

## Paired Normal/Normal comparisons

| Battle | Original W/L/D/U | Revised W/L/D/U | Original win rate [95% CI] | Revised win rate [95% CI] | Win change | Paired gained/lost wins |
|---|---|---|---|---|---:|---|
| Basics II | 218/82/0/0 | 256/38/6/0 | 72.7% [67.4, 77.4]; n=300; U=0; range 72.7–72.7% | 85.3% [80.9, 88.9]; n=300; U=0; range 85.3–85.3% | +12.7 pp | 40/2 |
| Basics IV | 112/173/15/0 | 283/10/7/0 | 37.3% [32.1, 42.9]; n=300; U=0; range 37.3–37.3% | 94.3% [91.1, 96.4]; n=300; U=0; range 94.3–94.3% | +57.0 pp | 176/5 |
| Patterns I | 96/156/48/0 | 212/70/18/0 | 32.0% [27.0, 37.5]; n=300; U=0; range 32.0–32.0% | 70.7% [65.3, 75.5]; n=300; U=0; range 70.7–70.7% | +38.7 pp | 149/33 |
| Diagonals I | 86/140/74/0 | 260/40/0/0 | 28.7% [23.8, 34.0]; n=300; U=0; range 28.7–28.7% | 86.7% [82.4, 90.1]; n=300; U=0; range 86.7–86.7% | +58.0 pp | 181/7 |
| Beams I | 147/127/26/0 | 222/76/2/0 | 49.0% [43.4, 54.6]; n=300; U=0; range 49.0–49.0% | 74.0% [68.8, 78.6]; n=300; U=0; range 74.0–74.0% | +25.0 pp | 112/37 |
| Shields I | 16/55/229/0 | 300/0/0/0 | 5.3% [3.3, 8.5]; n=300; U=0; range 5.3–5.3% | 100.0% [98.7, 100.0]; n=300; U=0; range 100.0–100.0% | +94.7 pp | 284/0 |
| Rite II | 39/47/214/0 | 96/116/88/0 | 13.0% [9.7, 17.3]; n=300; U=0; range 13.0–13.0% | 32.0% [27.0, 37.5]; n=300; U=0; range 32.0–32.0% | +19.0 pp | 80/23 |
| Rite IV | 133/115/52/0 | 139/101/60/0 | 44.3% [38.8, 50.0]; n=300; U=0; range 44.3–44.3% | 46.3% [40.8, 52.0]; n=300; U=0; range 46.3–46.3% | +2.0 pp | 75/69 |

Paired counts describe shared trial namespaces, not identical move trajectories after the scenario changes. The marginal Wilson intervals are not confidence intervals for the paired difference. No fixed win-rate target determines acceptance.

## Full revised matrices

Each cell reports its own actual sample count. Normal/Normal uses 300 trials on revised battles; their other cells use 30. Untouched battles and tutorial reuse 30-trial baseline cells. Tutorial opponents remain Easy with rule stalemates disabled.

### Basics I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-0-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-0-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-1-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-1-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-00-2-2.json) |

### Basics II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](screen/basics-ii/matchup-00-0-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](screen/basics-ii/matchup-00-0-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](screen/basics-ii/matchup-00-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](screen/basics-ii/matchup-00-1-0.json) | [85.3% [80.9, 88.9]; n=300; U=0; range 85.3–85.3%](final/basics-ii/matchup-00-1-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](screen/basics-ii/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/basics-ii/matchup-00-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/basics-ii/matchup-00-2-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/basics-ii/matchup-00-2-2.json) |

### Basics III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7%](../campaign-seed-1/matchup-02-0-0.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-02-0-1.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-02-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-seed-1/matchup-02-1-0.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-seed-1/matchup-02-1-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-seed-1/matchup-02-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-02-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-02-2-1.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-seed-1/matchup-02-2-2.json) |

### Basics IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](screen/basics-iv/matchup-00-0-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](screen/basics-iv/matchup-00-0-1.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](screen/basics-iv/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/basics-iv/matchup-00-1-0.json) | [94.3% [91.1, 96.4]; n=300; U=0; range 94.3–94.3%](final/basics-iv/matchup-00-1-1.json) | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](screen/basics-iv/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/basics-iv/matchup-00-2-0.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](screen/basics-iv/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/basics-iv/matchup-00-2-2.json) |

### Patterns I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](screen/patterns-i/matchup-00-0-0.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](screen/patterns-i/matchup-00-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](screen/patterns-i/matchup-00-0-2.json) |
| Normal | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](screen/patterns-i/matchup-00-1-0.json) | [70.7% [65.3, 75.5]; n=300; U=0; range 70.7–70.7%](final/patterns-i/matchup-00-1-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](screen/patterns-i/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/patterns-i/matchup-00-2-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](screen/patterns-i/matchup-00-2-1.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](screen/patterns-i/matchup-00-2-2.json) |

### Patterns II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-seed-1/matchup-05-0-0.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-05-0-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-seed-1/matchup-05-0-2.json) |
| Normal | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-seed-1/matchup-05-1-0.json) | [60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0%](../campaign-seed-1/matchup-05-1-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-seed-1/matchup-05-1-2.json) |
| Hard | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-seed-1/matchup-05-2-0.json) | [66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7%](../campaign-seed-1/matchup-05-2-1.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-seed-1/matchup-05-2-2.json) |

### Patterns III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-seed-1/matchup-06-0-0.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-seed-1/matchup-06-0-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-seed-1/matchup-06-0-2.json) |
| Normal | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-seed-1/matchup-06-1-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-seed-1/matchup-06-1-1.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-seed-1/matchup-06-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-06-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-seed-1/matchup-06-2-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-seed-1/matchup-06-2-2.json) |

### Diagonals I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](screen/diagonals-i/matchup-00-0-0.json) | [66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7%](screen/diagonals-i/matchup-00-0-1.json) | [60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0%](screen/diagonals-i/matchup-00-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](screen/diagonals-i/matchup-00-1-0.json) | [86.7% [82.4, 90.1]; n=300; U=0; range 86.7–86.7%](final/diagonals-i/matchup-00-1-1.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](screen/diagonals-i/matchup-00-1-2.json) |
| Hard | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](screen/diagonals-i/matchup-00-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/diagonals-i/matchup-00-2-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](screen/diagonals-i/matchup-00-2-2.json) |

### Diagonals II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-seed-1/matchup-08-0-0.json) | [20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0%](../campaign-seed-1/matchup-08-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-08-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-08-1-0.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-seed-1/matchup-08-1-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-08-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-seed-1/matchup-08-2-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-seed-1/matchup-08-2-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-seed-1/matchup-08-2-2.json) |

### Diagonals III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-09-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-09-0-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-09-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-09-1-0.json) | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-seed-1/matchup-09-1-1.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-seed-1/matchup-09-1-2.json) |
| Hard | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-seed-1/matchup-09-2-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-seed-1/matchup-09-2-1.json) | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-seed-1/matchup-09-2-2.json) |

### Diagonals IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-seed-1/matchup-10-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-10-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-10-0-2.json) |
| Normal | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-seed-1/matchup-10-1-0.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-seed-1/matchup-10-1-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-seed-1/matchup-10-1-2.json) |
| Hard | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-seed-1/matchup-10-2-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-seed-1/matchup-10-2-1.json) | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-seed-1/matchup-10-2-2.json) |

### Beams I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](screen/beams-i/matchup-00-0-0.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](screen/beams-i/matchup-00-0-1.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](screen/beams-i/matchup-00-0-2.json) |
| Normal | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](screen/beams-i/matchup-00-1-0.json) | [74.0% [68.8, 78.6]; n=300; U=0; range 74.0–74.0%](final/beams-i/matchup-00-1-1.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](screen/beams-i/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/beams-i/matchup-00-2-0.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](screen/beams-i/matchup-00-2-1.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](screen/beams-i/matchup-00-2-2.json) |

### Beams II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-seed-1/matchup-12-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-12-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-12-0-2.json) |
| Normal | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-seed-1/matchup-12-1-0.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-seed-1/matchup-12-1-1.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-12-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-seed-1/matchup-12-2-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](../campaign-seed-1/matchup-12-2-1.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-seed-1/matchup-12-2-2.json) |

### Beams III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-13-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-13-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-13-0-2.json) |
| Normal | [56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7%](../campaign-seed-1/matchup-13-1-0.json) | [40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0%](../campaign-seed-1/matchup-13-1-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-13-1-2.json) |
| Hard | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-seed-1/matchup-13-2-0.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-seed-1/matchup-13-2-1.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-13-2-2.json) |

### Shields I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/shields-i/matchup-00-0-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/shields-i/matchup-00-0-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/shields-i/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/shields-i/matchup-00-1-0.json) | [100.0% [98.7, 100.0]; n=300; U=0; range 100.0–100.0%](final/shields-i/matchup-00-1-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/shields-i/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/shields-i/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/shields-i/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](screen/shields-i/matchup-00-2-2.json) |

### Shields II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-seed-1/matchup-15-0-0.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-seed-1/matchup-15-0-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-seed-1/matchup-15-0-2.json) |
| Normal | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-seed-1/matchup-15-1-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-15-1-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-seed-1/matchup-15-1-2.json) |
| Hard | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-15-2-0.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-seed-1/matchup-15-2-1.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-seed-1/matchup-15-2-2.json) |

### Shields III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-16-0-0.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-seed-1/matchup-16-0-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-seed-1/matchup-16-0-2.json) |
| Normal | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-seed-1/matchup-16-1-0.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-16-1-1.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-seed-1/matchup-16-1-2.json) |
| Hard | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-seed-1/matchup-16-2-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-seed-1/matchup-16-2-1.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-16-2-2.json) |

### Challenge I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-seed-1/matchup-17-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-17-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-17-0-2.json) |
| Normal | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-seed-1/matchup-17-1-0.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-seed-1/matchup-17-1-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-seed-1/matchup-17-1-2.json) |
| Hard | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-seed-1/matchup-17-2-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](../campaign-seed-1/matchup-17-2-1.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-17-2-2.json) |

### Challenge II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-seed-1/matchup-18-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-18-0-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-seed-1/matchup-18-0-2.json) |
| Normal | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-seed-1/matchup-18-1-0.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-seed-1/matchup-18-1-1.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-seed-1/matchup-18-1-2.json) |
| Hard | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-18-2-0.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-seed-1/matchup-18-2-1.json) | [30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0%](../campaign-seed-1/matchup-18-2-2.json) |

### Challenge III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-seed-1/matchup-19-0-0.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-19-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-19-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-19-1-0.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-seed-1/matchup-19-1-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-19-1-2.json) |
| Hard | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](../campaign-seed-1/matchup-19-2-0.json) | [40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0%](../campaign-seed-1/matchup-19-2-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-19-2-2.json) |

### Challenge IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-seed-1/matchup-20-0-0.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-seed-1/matchup-20-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-20-0-2.json) |
| Normal | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-20-1-0.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-seed-1/matchup-20-1-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](../campaign-seed-1/matchup-20-1-2.json) |
| Hard | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](../campaign-seed-1/matchup-20-2-0.json) | [60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0%](../campaign-seed-1/matchup-20-2-1.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-seed-1/matchup-20-2-2.json) |

### Rite I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-seed-1/matchup-21-0-0.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-21-0-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-21-0-2.json) |
| Normal | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-seed-1/matchup-21-1-0.json) | [56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7%](../campaign-seed-1/matchup-21-1-1.json) | [33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3%](../campaign-seed-1/matchup-21-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-21-2-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-21-2-1.json) | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-seed-1/matchup-21-2-2.json) |

### Rite II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0%](screen/rite-ii/matchup-00-0-0.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](screen/rite-ii/matchup-00-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](screen/rite-ii/matchup-00-0-2.json) |
| Normal | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](screen/rite-ii/matchup-00-1-0.json) | [32.0% [27.0, 37.5]; n=300; U=0; range 32.0–32.0%](final/rite-ii/matchup-00-1-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](screen/rite-ii/matchup-00-1-2.json) |
| Hard | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](screen/rite-ii/matchup-00-2-0.json) | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](screen/rite-ii/matchup-00-2-1.json) | [26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7%](screen/rite-ii/matchup-00-2-2.json) |

### Rite III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-seed-1/matchup-23-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-23-0-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-23-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-23-1-0.json) | [43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3%](../campaign-seed-1/matchup-23-1-1.json) | [13.3% [5.3, 29.7]; n=30; U=0; range 13.3–13.3%](../campaign-seed-1/matchup-23-1-2.json) |
| Hard | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](../campaign-seed-1/matchup-23-2-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-23-2-1.json) | [36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7%](../campaign-seed-1/matchup-23-2-2.json) |

### Rite IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7%](screen/rite-iv/matchup-00-0-0.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](screen/rite-iv/matchup-00-0-1.json) | [10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0%](screen/rite-iv/matchup-00-0-2.json) |
| Normal | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](screen/rite-iv/matchup-00-1-0.json) | [46.3% [40.8, 52.0]; n=300; U=0; range 46.3–46.3%](final/rite-iv/matchup-00-1-1.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](screen/rite-iv/matchup-00-1-2.json) |
| Hard | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](screen/rite-iv/matchup-00-2-0.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](screen/rite-iv/matchup-00-2-1.json) | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](screen/rite-iv/matchup-00-2-2.json) |

### Ascension I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-seed-1/matchup-25-0-0.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-25-0-1.json) | [0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0%](../campaign-seed-1/matchup-25-0-2.json) |
| Normal | [66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7%](../campaign-seed-1/matchup-25-1-0.json) | [50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0%](../campaign-seed-1/matchup-25-1-1.json) | [16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7%](../campaign-seed-1/matchup-25-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-seed-1/matchup-25-2-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](../campaign-seed-1/matchup-25-2-1.json) | [46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7%](../campaign-seed-1/matchup-25-2-2.json) |

### Ascension II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0%](../campaign-seed-1/matchup-26-0-0.json) | [26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7%](../campaign-seed-1/matchup-26-0-1.json) | [3.3% [0.6, 16.7]; n=30; U=0; range 3.3–3.3%](../campaign-seed-1/matchup-26-0-2.json) |
| Normal | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](../campaign-seed-1/matchup-26-1-0.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-seed-1/matchup-26-1-1.json) | [6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7%](../campaign-seed-1/matchup-26-1-2.json) |
| Hard | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](../campaign-seed-1/matchup-26-2-0.json) | [53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3%](../campaign-seed-1/matchup-26-2-1.json) | [23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3%](../campaign-seed-1/matchup-26-2-2.json) |

### Junction I

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](junction-screen/junction-i/matchup-00-0-0.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](junction-screen/junction-i/matchup-00-0-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](junction-screen/junction-i/matchup-00-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](junction-screen/junction-i/matchup-00-1-0.json) | [88.0% [83.8, 91.2]; n=300; U=0; range 88.0–88.0%](junction-final/junction-i/matchup-00-1-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](junction-screen/junction-i/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-i/matchup-00-2-0.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-i/matchup-00-2-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-i/matchup-00-2-2.json) |

### Junction II

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](junction-screen/junction-ii/matchup-00-0-0.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](junction-screen/junction-ii/matchup-00-0-1.json) | [73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3%](junction-screen/junction-ii/matchup-00-0-2.json) |
| Normal | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](junction-screen/junction-ii/matchup-00-1-0.json) | [75.7% [70.5, 80.2]; n=300; U=0; range 75.7–75.7%](junction-final/junction-ii/matchup-00-1-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](junction-screen/junction-ii/matchup-00-1-2.json) |
| Hard | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](junction-screen/junction-ii/matchup-00-2-0.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](junction-screen/junction-ii/matchup-00-2-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](junction-screen/junction-ii/matchup-00-2-2.json) |

### Junction III

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iii/matchup-00-0-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iii/matchup-00-0-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iii/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iii/matchup-00-1-0.json) | [100.0% [98.7, 100.0]; n=300; U=0; range 100.0–100.0%](junction-final/junction-iii/matchup-00-1-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iii/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iii/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iii/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iii/matchup-00-2-2.json) |

### Junction IV

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](junction-screen/junction-iv/matchup-00-0-0.json) | [70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0%](junction-screen/junction-iv/matchup-00-0-1.json) | [63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3%](junction-screen/junction-iv/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-iv/matchup-00-1-0.json) | [88.7% [84.6, 91.8]; n=300; U=0; range 88.7–88.7%](junction-final/junction-iv/matchup-00-1-1.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](junction-screen/junction-iv/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-iv/matchup-00-2-0.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](junction-screen/junction-iv/matchup-00-2-1.json) | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](junction-screen/junction-iv/matchup-00-2-2.json) |

### Junction V

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-v/matchup-00-0-0.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](junction-screen/junction-v/matchup-00-0-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](junction-screen/junction-v/matchup-00-0-2.json) |
| Normal | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-v/matchup-00-1-0.json) | [84.7% [80.2, 88.3]; n=300; U=0; range 84.7–84.7%](junction-final/junction-v/matchup-00-1-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](junction-screen/junction-v/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-v/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-v/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-v/matchup-00-2-2.json) |

### Junction VI

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0%](junction-screen/junction-vi/matchup-00-0-0.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](junction-screen/junction-vi/matchup-00-0-1.json) | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](junction-screen/junction-vi/matchup-00-0-2.json) |
| Normal | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](junction-screen/junction-vi/matchup-00-1-0.json) | [95.3% [92.3, 97.2]; n=300; U=0; range 95.3–95.3%](junction-final/junction-vi/matchup-00-1-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-vi/matchup-00-1-2.json) |
| Hard | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](junction-screen/junction-vi/matchup-00-2-0.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](junction-screen/junction-vi/matchup-00-2-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-vi/matchup-00-2-2.json) |

### Junction VII

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-vii/matchup-00-0-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-vii/matchup-00-0-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-vii/matchup-00-0-2.json) |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-vii/matchup-00-1-0.json) | [100.0% [98.7, 100.0]; n=300; U=0; range 100.0–100.0%](junction-final/junction-vii/matchup-00-1-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-vii/matchup-00-1-2.json) |
| Hard | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-vii/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-vii/matchup-00-2-1.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-vii/matchup-00-2-2.json) |

### Junction VIII

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0%](junction-screen/junction-viii/matchup-00-0-0.json) | [83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3%](junction-screen/junction-viii/matchup-00-0-1.json) | [76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7%](junction-screen/junction-viii/matchup-00-0-2.json) |
| Normal | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-viii/matchup-00-1-0.json) | [92.3% [88.8, 94.8]; n=300; U=0; range 92.3–92.3%](junction-final/junction-viii/matchup-00-1-1.json) | [86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7%](junction-screen/junction-viii/matchup-00-1-2.json) |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-viii/matchup-00-2-0.json) | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](junction-screen/junction-viii/matchup-00-2-1.json) | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](junction-screen/junction-viii/matchup-00-2-2.json) |

### Tutorial

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | [93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3%](../campaign-seed-1/matchup-27-0-0.json) | — | — |
| Normal | [100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0%](../campaign-seed-1/matchup-27-1-0.json) | — | — |
| Hard | [96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7%](../campaign-seed-1/matchup-27-2-0.json) | — | — |

## Main-route progression

Normal/Normal ordered-edge comparisons. A ≥20-point drop is a supported exploratory candidate only with disjoint intervals and ≤10% unresolved at both ends. No multiple-comparison correction; optional difficulty is not a core-route failure. Distances use directed shortest paths.

| Edge | Distances | Red win drop | Flag |
|---|---|---:|---|
| Basics I → Basics II | 1 → 2 | +14.7 pp | below spike threshold |
| Basics II → Basics III | 2 → 3 | -1.3 pp | below spike threshold |
| Basics III → Basics IV | 3 → 4 | -7.7 pp | below spike threshold |
| Basics IV → Patterns I | 4 → 5 | +23.7 pp | supported candidate |
| Patterns I → Patterns II | 5 → 6 | +10.7 pp | below spike threshold |
| Patterns II → Patterns III | 6 → 7 | -10.0 pp | below spike threshold |
| Patterns III → Diagonals I | 7 → 8 | -16.7 pp | below spike threshold |
| Diagonals I → Beams I | 8 → 9 | +12.7 pp | below spike threshold |
| Beams I → Shields I | 9 → 10 | -26.0 pp | below spike threshold |
| Shields I → Junction I | 10 → 11 | +12.0 pp | below spike threshold |
| Junction I → Rite I | 11 → 12 | +31.3 pp | supported candidate |
| Rite I → Rite II | 12 → 13 | +24.7 pp | supported candidate |
| Rite II → Rite III | 13 → 14 | -11.3 pp | below spike threshold |
| Rite III → Rite IV | 14 → 15 | -3.0 pp | below spike threshold |
| Rite IV → Ascension I | 15 → 16 | -3.7 pp | below spike threshold |
| Ascension I → Ascension II | 16 → 17 | +26.7 pp | followup candidate |

## Optional routes and shortcuts

Normal/Normal ordered-edge comparisons. A ≥20-point drop is a supported exploratory candidate only with disjoint intervals and ≤10% unresolved at both ends. No multiple-comparison correction; optional difficulty is not a core-route failure. Distances use directed shortest paths.

| Edge | Distances | Red win drop | Flag |
|---|---|---:|---|
| Diagonals I → Diagonals II | 8 → 9 | +43.3 pp | supported candidate |
| Diagonals II → Diagonals III | 9 → 10 | -3.3 pp | below spike threshold |
| Diagonals III → Beams I | 10 → 9 | -27.3 pp | below spike threshold |
| Beams I → Diagonals IV | 9 → 10 | +40.7 pp | supported candidate |
| Diagonals IV → Beams II | 10 → 11 | -3.3 pp | below spike threshold |
| Beams II → Beams III | 11 → 12 | -3.3 pp | below spike threshold |
| Beams III → Challenge II | 12 → 13 | +6.7 pp | below spike threshold |
| Challenge II → Shields I | 13 → 10 | -66.7 pp | below spike threshold |
| Shields I → Junction I | 10 → 11 | +12.0 pp | below spike threshold |
| Junction I → Challenge I | 11 → 12 | +51.3 pp | supported candidate |
| Challenge I → Rite II | 12 → 13 | +4.7 pp | below spike threshold |
| Shields I → Junction I | 10 → 11 | +12.0 pp | below spike threshold |
| Junction I → Challenge III | 11 → 12 | +64.7 pp | supported candidate |
| Challenge III → Junction II | 12 → 13 | -52.3 pp | below spike threshold |
| Junction II → Junction III | 13 → 14 | -24.3 pp | below spike threshold |
| Junction III → Junction IV | 14 → 15 | +11.3 pp | below spike threshold |
| Junction IV → Junction V | 15 → 16 | +4.0 pp | below spike threshold |
| Junction V → Rite IV | 16 → 15 | +38.3 pp | supported candidate |
| Shields I → Shields II | 10 → 11 | +83.3 pp | supported candidate |
| Shields II → Shields III | 11 → 12 | -36.7 pp | below spike threshold |
| Shields III → Challenge IV | 12 → 13 | +16.7 pp | below spike threshold |
| Challenge IV → Junction VI | 13 → 14 | -58.7 pp | below spike threshold |
| Junction VI → Junction VII | 14 → 15 | -4.7 pp | below spike threshold |
| Junction VII → Junction VIII | 15 → 16 | +7.7 pp | below spike threshold |
| Junction VIII → Rite IV | 16 → 15 | +46.0 pp | supported candidate |

## Replay accounting

Inactivity is the existing no-combat timer, which may award either side a win by remaining mana. “No damage” means no recorded hit at any point, a stricter passive-ending indicator. All trace files include moves, pickups, hit tiles, mana totals and termination cause.

| Scenario | Version | Inactivity endings / wins | No-damage games / wins | First Win / Loss / Draw trials |
|---|---|---|---|---|
| Basics II | [original](original/basics-ii/matchup-00-1-1.json) | 3 / 2 | 0 / 0 | 1 / 0 / — |
| Basics II | [final](final/basics-ii/matchup-00-1-1.json) | 13 / 8 | 0 / 0 | 0 / 10 / 26 |
| Basics IV | [original](original/basics-iv/matchup-00-1-1.json) | 59 / 14 | 0 / 0 | 0 / 3 / 80 |
| Basics IV | [final](final/basics-iv/matchup-00-1-1.json) | 38 / 32 | 0 / 0 | 0 / 10 / 89 |
| Patterns I | [original](original/patterns-i/matchup-00-1-1.json) | 146 / 30 | 0 / 0 | 4 / 0 / 13 |
| Patterns I | [final](final/patterns-i/matchup-00-1-1.json) | 125 / 87 | 0 / 0 | 0 / 3 / 20 |
| Diagonals I | [original](original/diagonals-i/matchup-00-1-1.json) | 149 / 16 | 69 / 0 | 0 / 1 / 8 |
| Diagonals I | [final](final/diagonals-i/matchup-00-1-1.json) | 2 / 2 | 0 / 0 | 0 / 3 / — |
| Beams I | [original](original/beams-i/matchup-00-1-1.json) | 125 / 44 | 0 / 0 | 0 / 2 / 10 |
| Beams I | [final](final/beams-i/matchup-00-1-1.json) | 13 / 9 | 0 / 0 | 0 / 5 / 88 |
| Shields I | [original](original/shields-i/matchup-00-1-1.json) | 93 / 4 | 0 / 0 | 47 / 1 / 0 |
| Shields I | [final](final/shields-i/matchup-00-1-1.json) | 1 / 1 | 0 / 0 | 0 / — / — |
| Rite II | [original](original/rite-ii/matchup-00-1-1.json) | 295 / 37 | 188 / 0 | 5 / 1 / 0 |
| Rite II | [final](final/rite-ii/matchup-00-1-1.json) | 176 / 50 | 0 / 0 | 1 / 0 / 2 |
| Rite IV | [original](original/rite-iv/matchup-00-1-1.json) | 214 / 83 | 0 / 0 | 8 / 0 / 1 |
| Rite IV | [final](final/rite-iv/matchup-00-1-1.json) | 137 / 32 | 0 / 0 | 0 / 5 / 1 |
| Junction I | [junction-final](junction-final/junction-i/matchup-00-1-1.json) | 1 / 0 | 0 / 0 | 0 / 2 / 173 |
| Junction II | [junction-final](junction-final/junction-ii/matchup-00-1-1.json) | 0 / 0 | 0 / 0 | 1 / 0 / — |
| Junction III | [junction-final](junction-final/junction-iii/matchup-00-1-1.json) | 2 / 2 | 0 / 0 | 0 / — / — |
| Junction IV | [junction-final](junction-final/junction-iv/matchup-00-1-1.json) | 0 / 0 | 0 / 0 | 0 / 7 / — |
| Junction V | [junction-final](junction-final/junction-v/matchup-00-1-1.json) | 13 / 10 | 0 / 0 | 1 / 0 / 204 |
| Junction VI | [junction-final](junction-final/junction-vi/matchup-00-1-1.json) | 0 / 0 | 0 / 0 | 0 / 11 / — |
| Junction VII | [junction-final](junction-final/junction-vii/matchup-00-1-1.json) | 0 / 0 | 0 / 0 | 0 / — / — |
| Junction VIII | [junction-final](junction-final/junction-viii/matchup-00-1-1.json) | 10 / 6 | 0 / 0 | 0 / 12 / 141 |

See [interpretation](interpretation.md) for inspected teaching sequences, remaining draw problems, and limitations. Individual screen reports retain all search telemetry, sensitivity/reversal flags and node saturation findings. The combined JSON preserves all aggregate telemetry with source attribution.
