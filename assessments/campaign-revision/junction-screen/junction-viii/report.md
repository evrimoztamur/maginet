# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Junction VIII

Map (12, 1); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% |

- Easy/Easy: W/L/D/U 24/6/0/0; mean 8.1 plies (5–18); 3999 nodes; node saturation 0.0%; fallbacks 0/244.

- Easy/Normal: W/L/D/U 25/4/1/0; mean 8.0 plies (5–20); 12476 nodes; node saturation 0.0%; fallbacks 0/241.

- Easy/Hard: W/L/D/U 23/6/1/0; mean 8.6 plies (5–20); 103976 nodes; node saturation 0.0%; fallbacks 0/259.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 7.4 plies (5–17); 14436 nodes; node saturation 0.0%; fallbacks 0/221.

- Normal/Normal: W/L/D/U 28/2/0/0; mean 7.1 plies (5–15); 20380 nodes; node saturation 0.0%; fallbacks 0/214.

- Normal/Hard: W/L/D/U 26/4/0/0; mean 7.4 plies (5–20); 85845 nodes; node saturation 0.0%; fallbacks 0/223.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 11.2 plies (5–26); 169578 nodes; node saturation 0.0%; fallbacks 0/337.

- Hard/Normal: W/L/D/U 30/0/0/0; mean 10.9 plies (5–25); 218921 nodes; node saturation 0.0%; fallbacks 0/328.

- Hard/Hard: W/L/D/U 29/1/0/0; mean 10.1 plies (5–26); 327682 nodes; node saturation 0.0%; fallbacks 0/303.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

