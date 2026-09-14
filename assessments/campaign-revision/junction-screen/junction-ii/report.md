# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Junction II

Map (10, -2); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% |
| Normal | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |
| Hard | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% |

- Easy/Easy: W/L/D/U 21/9/0/0; mean 8.9 plies (8–11); 3910 nodes; node saturation 0.0%; fallbacks 0/267.

- Easy/Normal: W/L/D/U 23/7/0/0; mean 9.6 plies (8–12); 12927 nodes; node saturation 0.0%; fallbacks 0/287.

- Easy/Hard: W/L/D/U 22/8/0/0; mean 9.5 plies (8–12); 76927 nodes; node saturation 0.0%; fallbacks 0/286.

- Normal/Easy: W/L/D/U 23/7/0/0; mean 10.4 plies (8–15); 16630 nodes; node saturation 0.0%; fallbacks 0/311.

- Normal/Normal: W/L/D/U 26/4/0/0; mean 10.5 plies (8–15); 26656 nodes; node saturation 0.0%; fallbacks 0/314.

- Normal/Hard: W/L/D/U 24/6/0/0; mean 10.4 plies (8–13); 95034 nodes; node saturation 0.0%; fallbacks 0/312.

- Hard/Easy: W/L/D/U 26/4/0/0; mean 11.1 plies (8–15); 101209 nodes; node saturation 0.0%; fallbacks 0/334.

- Hard/Normal: W/L/D/U 25/5/0/0; mean 11.2 plies (8–15); 112544 nodes; node saturation 0.0%; fallbacks 0/335.

- Hard/Hard: W/L/D/U 24/6/0/0; mean 11.1 plies (8–15); 184130 nodes; node saturation 0.0%; fallbacks 0/334.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

