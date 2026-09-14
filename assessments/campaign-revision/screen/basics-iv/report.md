# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Basics IV

Map (2, -1); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 27/3/0/0; mean 15.6 plies (7–34); 10714 nodes; node saturation 0.0%; fallbacks 0/467.

- Easy/Normal: W/L/D/U 21/7/2/0; mean 16.0 plies (7–31); 50767 nodes; node saturation 0.0%; fallbacks 0/479.

- Easy/Hard: W/L/D/U 23/6/1/0; mean 12.9 plies (5–26); 640412 nodes; node saturation 0.0%; fallbacks 0/388.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 12.7 plies (7–27); 54822 nodes; node saturation 0.0%; fallbacks 0/381.

- Normal/Normal: W/L/D/U 28/2/0/0; mean 11.2 plies (7–21); 84258 nodes; node saturation 0.0%; fallbacks 0/336.

- Normal/Hard: W/L/D/U 27/3/0/0; mean 10.9 plies (7–24); 569440 nodes; node saturation 0.0%; fallbacks 0/326.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 14.3 plies (7–27); 836136 nodes; node saturation 0.0%; fallbacks 0/429.

- Hard/Normal: W/L/D/U 28/1/1/0; mean 15.0 plies (7–21); 956356 nodes; node saturation 0.0%; fallbacks 0/449.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 14.4 plies (7–24); 1452031 nodes; node saturation 0.0%; fallbacks 0/431.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Basics IV: player Easy→Hard sensitivity against Normal: +23.3 points.
