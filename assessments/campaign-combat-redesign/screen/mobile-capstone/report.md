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
| Easy | 70.0% [52.1, 83.3]; n=30; U=0; range 70.0–70.0% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% |
| Normal | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 53.3% [36.1, 69.8]; n=30; U=0; range 53.3–53.3% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% |

- Easy/Easy: W/L/D/U 21/7/2/0; mean 24.0 plies (15–51); 29019 nodes; node saturation 0.0%; fallbacks 0/721.

- Easy/Normal: W/L/D/U 11/17/2/0; mean 24.4 plies (14–33); 219544 nodes; node saturation 0.0%; fallbacks 0/731.

- Easy/Hard: W/L/D/U 8/21/1/0; mean 25.7 plies (16–38); 3765753 nodes; node saturation 18.4%; fallbacks 0/771.

- Normal/Easy: W/L/D/U 27/1/2/0; mean 22.0 plies (15–31); 232867 nodes; node saturation 0.0%; fallbacks 0/661.

- Normal/Normal: W/L/D/U 20/9/1/0; mean 23.3 plies (15–38); 411788 nodes; node saturation 0.0%; fallbacks 0/698.

- Normal/Hard: W/L/D/U 16/13/1/0; mean 25.6 plies (15–45); 3886876 nodes; node saturation 19.0%; fallbacks 0/768.

- Hard/Easy: W/L/D/U 29/1/0/0; mean 24.6 plies (17–34); 3818157 nodes; node saturation 19.9%; fallbacks 0/739.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 24.3 plies (15–36); 4146189 nodes; node saturation 21.6%; fallbacks 0/728.

- Hard/Hard: W/L/D/U 22/6/2/0; mean 27.2 plies (19–35); 7828013 nodes; node saturation 34.4%; fallbacks 0/816.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Custom scenario: player Easy→Hard sensitivity against Normal: +60.0 points.
