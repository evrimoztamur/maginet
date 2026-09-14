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
| Easy | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% | 20.0% [9.5, 37.3]; n=30; U=0; range 20.0–20.0% |
| Normal | 93.3% [78.7, 98.2]; n=30; U=0; range 93.3–93.3% | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 24/6/0/0; mean 21.9 plies (15–38); 31588 nodes; node saturation 0.0%; fallbacks 0/658.

- Easy/Normal: W/L/D/U 9/21/0/0; mean 21.8 plies (16–32); 247278 nodes; node saturation 0.0%; fallbacks 0/655.

- Easy/Hard: W/L/D/U 6/21/3/0; mean 23.0 plies (13–38); 3607475 nodes; node saturation 21.0%; fallbacks 0/691.

- Normal/Easy: W/L/D/U 28/1/1/0; mean 19.4 plies (13–27); 219848 nodes; node saturation 0.0%; fallbacks 0/583.

- Normal/Normal: W/L/D/U 24/5/1/0; mean 23.1 plies (14–34); 447649 nodes; node saturation 0.0%; fallbacks 0/692.

- Normal/Hard: W/L/D/U 12/15/3/0; mean 24.0 plies (15–34); 3923888 nodes; node saturation 21.2%; fallbacks 0/721.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 24.0 plies (15–43); 4142818 nodes; node saturation 22.4%; fallbacks 0/719.

- Hard/Normal: W/L/D/U 26/1/3/0; mean 24.3 plies (15–41); 4312200 nodes; node saturation 22.1%; fallbacks 0/729.

- Hard/Hard: W/L/D/U 23/6/1/0; mean 25.4 plies (18–43); 7801127 nodes; node saturation 40.2%; fallbacks 0/763.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Custom scenario: player Easy→Hard sensitivity against Normal: +56.7 points.
