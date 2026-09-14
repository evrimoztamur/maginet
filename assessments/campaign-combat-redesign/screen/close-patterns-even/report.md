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
| Easy | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Normal | 83.3% [66.4, 92.7]; n=30; U=0; range 83.3–83.3% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% |
| Hard | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 90.0% [74.4, 96.5]; n=30; U=0; range 90.0–90.0% | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% |

- Easy/Easy: W/L/D/U 15/11/4/0; mean 26.9 plies (17–36); 31206 nodes; node saturation 0.0%; fallbacks 0/807.

- Easy/Normal: W/L/D/U 12/15/3/0; mean 25.5 plies (17–48); 216116 nodes; node saturation 0.0%; fallbacks 0/764.

- Easy/Hard: W/L/D/U 2/28/0/0; mean 26.6 plies (14–43); 3783505 nodes; node saturation 17.4%; fallbacks 0/799.

- Normal/Easy: W/L/D/U 25/1/4/0; mean 26.4 plies (19–36); 224526 nodes; node saturation 0.0%; fallbacks 0/791.

- Normal/Normal: W/L/D/U 19/7/4/0; mean 25.7 plies (17–40); 410895 nodes; node saturation 0.0%; fallbacks 0/772.

- Normal/Hard: W/L/D/U 8/22/0/0; mean 27.1 plies (20–38); 4281416 nodes; node saturation 18.7%; fallbacks 0/814.

- Hard/Easy: W/L/D/U 29/0/1/0; mean 28.4 plies (17–57); 4655204 nodes; node saturation 19.8%; fallbacks 0/853.

- Hard/Normal: W/L/D/U 27/2/1/0; mean 26.5 plies (15–38); 4471551 nodes; node saturation 19.6%; fallbacks 0/795.

- Hard/Hard: W/L/D/U 17/11/2/0; mean 29.1 plies (18–52); 8737221 nodes; node saturation 35.3%; fallbacks 0/873.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Custom scenario: player Easy→Hard sensitivity against Normal: +50.0 points.
