# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Diagonals I

Map (6, -1); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 60.0% [42.3, 75.4]; n=30; U=0; range 60.0–60.0% |
| Normal | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% |
| Hard | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% |

- Easy/Easy: W/L/D/U 22/8/0/0; mean 9.1 plies (5–24); 4706 nodes; node saturation 0.0%; fallbacks 0/274.

- Easy/Normal: W/L/D/U 20/10/0/0; mean 8.3 plies (5–16); 17241 nodes; node saturation 0.0%; fallbacks 0/248.

- Easy/Hard: W/L/D/U 18/12/0/0; mean 9.3 plies (5–22); 174250 nodes; node saturation 0.0%; fallbacks 0/278.

- Normal/Easy: W/L/D/U 28/2/0/0; mean 6.9 plies (5–15); 18457 nodes; node saturation 0.0%; fallbacks 0/206.

- Normal/Normal: W/L/D/U 26/4/0/0; mean 6.9 plies (5–16); 28834 nodes; node saturation 0.0%; fallbacks 0/208.

- Normal/Hard: W/L/D/U 25/5/0/0; mean 8.1 plies (5–18); 166581 nodes; node saturation 0.0%; fallbacks 0/243.

- Hard/Easy: W/L/D/U 28/2/0/0; mean 10.4 plies (5–19); 215128 nodes; node saturation 0.0%; fallbacks 0/312.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 11.4 plies (5–27); 263557 nodes; node saturation 0.0%; fallbacks 0/341.

- Hard/Hard: W/L/D/U 26/4/0/0; mean 10.3 plies (5–18); 427951 nodes; node saturation 0.0%; fallbacks 0/310.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Diagonals I: player Easy→Hard sensitivity against Normal: +30.0 points.
