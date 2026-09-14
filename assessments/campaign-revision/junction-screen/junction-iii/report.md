# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Junction III

Map (11, -2); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 30/0/0/0; mean 7.3 plies (5–15); 3034 nodes; node saturation 0.0%; fallbacks 0/220.

- Easy/Normal: W/L/D/U 30/0/0/0; mean 7.5 plies (5–15); 9502 nodes; node saturation 0.0%; fallbacks 0/226.

- Easy/Hard: W/L/D/U 30/0/0/0; mean 7.4 plies (5–13); 41246 nodes; node saturation 0.0%; fallbacks 0/222.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 7.0 plies (5–15); 9899 nodes; node saturation 0.0%; fallbacks 0/210.

- Normal/Normal: W/L/D/U 30/0/0/0; mean 6.9 plies (5–15); 15597 nodes; node saturation 0.0%; fallbacks 0/208.

- Normal/Hard: W/L/D/U 30/0/0/0; mean 7.3 plies (5–17); 44252 nodes; node saturation 0.0%; fallbacks 0/220.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 8.4 plies (5–17); 56770 nodes; node saturation 0.0%; fallbacks 0/252.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 8.3 plies (5–17); 61503 nodes; node saturation 0.0%; fallbacks 0/248.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 8.3 plies (5–17); 91837 nodes; node saturation 0.0%; fallbacks 0/250.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

