# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Junction I

Map (9, -1); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |
| Normal | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% |

- Easy/Easy: W/L/D/U 27/2/1/0; mean 7.2 plies (3–17); 2786 nodes; node saturation 0.0%; fallbacks 0/216.

- Easy/Normal: W/L/D/U 25/5/0/0; mean 5.4 plies (3–12); 6617 nodes; node saturation 0.0%; fallbacks 0/161.

- Easy/Hard: W/L/D/U 24/6/0/0; mean 5.9 plies (3–16); 37446 nodes; node saturation 0.0%; fallbacks 0/178.

- Normal/Easy: W/L/D/U 28/2/0/0; mean 6.7 plies (5–12); 8828 nodes; node saturation 0.0%; fallbacks 0/200.

- Normal/Normal: W/L/D/U 26/4/0/0; mean 5.9 plies (3–10); 12434 nodes; node saturation 0.0%; fallbacks 0/176.

- Normal/Hard: W/L/D/U 26/4/0/0; mean 6.5 plies (3–16); 40531 nodes; node saturation 0.0%; fallbacks 0/194.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 7.8 plies (5–21); 50490 nodes; node saturation 0.0%; fallbacks 0/233.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 6.8 plies (3–15); 46137 nodes; node saturation 0.0%; fallbacks 0/203.

- Hard/Hard: W/L/D/U 29/1/0/0; mean 6.8 plies (3–13); 76946 nodes; node saturation 0.0%; fallbacks 0/205.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

