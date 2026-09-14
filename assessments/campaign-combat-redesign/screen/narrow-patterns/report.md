# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Custom scenario

Map (0, 0); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% |
| Normal | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% |

- Easy/Easy: W/L/D/U 29/0/1/0; mean 15.9 plies (7–40); 11526 nodes; node saturation 0.0%; fallbacks 0/476.

- Easy/Normal: W/L/D/U 28/2/0/0; mean 15.9 plies (9–29); 63056 nodes; node saturation 0.0%; fallbacks 0/478.

- Easy/Hard: W/L/D/U 25/5/0/0; mean 17.3 plies (9–40); 1598252 nodes; node saturation 5.0%; fallbacks 0/519.

- Normal/Easy: W/L/D/U 30/0/0/0; mean 14.8 plies (7–27); 74756 nodes; node saturation 0.0%; fallbacks 0/444.

- Normal/Normal: W/L/D/U 28/2/0/0; mean 16.1 plies (9–27); 129412 nodes; node saturation 0.0%; fallbacks 0/482.

- Normal/Hard: W/L/D/U 29/1/0/0; mean 13.5 plies (7–27); 1502853 nodes; node saturation 6.4%; fallbacks 0/405.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 17.9 plies (7–42); 1481852 nodes; node saturation 0.4%; fallbacks 0/538.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 17.8 plies (9–31); 1578021 nodes; node saturation 0.4%; fallbacks 0/534.

- Hard/Hard: W/L/D/U 30/0/0/0; mean 16.5 plies (7–33); 2864114 nodes; node saturation 5.5%; fallbacks 0/495.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

