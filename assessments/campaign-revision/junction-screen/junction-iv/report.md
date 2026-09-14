# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Junction IV

Map (12, -2); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% |

- Easy/Easy: W/L/D/U 26/3/1/0; mean 8.8 plies (3–16); 3348 nodes; node saturation 0.0%; fallbacks 0/265.

- Easy/Normal: W/L/D/U 21/9/0/0; mean 7.2 plies (3–14); 8982 nodes; node saturation 0.0%; fallbacks 0/215.

- Easy/Hard: W/L/D/U 19/11/0/0; mean 8.2 plies (3–18); 57474 nodes; node saturation 0.0%; fallbacks 0/245.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 7.9 plies (3–13); 10545 nodes; node saturation 0.0%; fallbacks 0/236.

- Normal/Normal: W/L/D/U 24/6/0/0; mean 7.5 plies (3–16); 15245 nodes; node saturation 0.0%; fallbacks 0/226.

- Normal/Hard: W/L/D/U 23/7/0/0; mean 8.0 plies (3–16); 51915 nodes; node saturation 0.0%; fallbacks 0/241.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 8.3 plies (3–15); 58298 nodes; node saturation 0.0%; fallbacks 0/249.

- Hard/Normal: W/L/D/U 28/2/0/0; mean 8.0 plies (3–19); 55059 nodes; node saturation 0.0%; fallbacks 0/240.

- Hard/Hard: W/L/D/U 28/2/0/0; mean 7.5 plies (3–15); 80066 nodes; node saturation 0.0%; fallbacks 0/226.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Junction IV: player Easy→Hard sensitivity against Normal: +23.3 points.
