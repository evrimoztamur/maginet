# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Beams I

Map (7, -1); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% |

- Easy/Easy: W/L/D/U 19/11/0/0; mean 15.2 plies (7–25); 14604 nodes; node saturation 0.0%; fallbacks 0/457.

- Easy/Normal: W/L/D/U 9/20/1/0; mean 15.8 plies (11–22); 83614 nodes; node saturation 0.0%; fallbacks 0/474.

- Easy/Hard: W/L/D/U 11/18/1/0; mean 16.9 plies (9–26); 1881761 nodes; node saturation 1.0%; fallbacks 0/508.

- Normal/Easy: W/L/D/U 24/5/1/0; mean 12.8 plies (7–26); 74090 nodes; node saturation 0.0%; fallbacks 0/383.

- Normal/Normal: W/L/D/U 25/5/0/0; mean 12.8 plies (7–26); 130503 nodes; node saturation 0.0%; fallbacks 0/385.

- Normal/Hard: W/L/D/U 21/9/0/0; mean 14.9 plies (9–28); 1838225 nodes; node saturation 3.6%; fallbacks 0/447.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 15.4 plies (9–27); 1781377 nodes; node saturation 7.4%; fallbacks 0/462.

- Hard/Normal: W/L/D/U 28/2/0/0; mean 15.1 plies (9–26); 1858726 nodes; node saturation 7.7%; fallbacks 0/454.

- Hard/Hard: W/L/D/U 25/5/0/0; mean 16.4 plies (9–24); 3460610 nodes; node saturation 7.7%; fallbacks 0/491.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Beams I: player Easy→Hard sensitivity against Normal: +63.3 points.
