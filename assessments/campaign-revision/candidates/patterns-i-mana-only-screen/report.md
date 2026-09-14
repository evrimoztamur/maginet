# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Patterns I

Map (3, -1); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 63.3% [45.5, 78.1]; n=30; U=0; range 63.3–63.3% | 30.0% [16.7, 47.9]; n=30; U=0; range 30.0–30.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% |

- Easy/Easy: W/L/D/U 20/9/1/0; mean 31.4 plies (19–47); 63236 nodes; node saturation 0.0%; fallbacks 0/942.

- Easy/Normal: W/L/D/U 7/19/4/0; mean 33.0 plies (22–54); 595206 nodes; node saturation 0.0%; fallbacks 0/989.

- Easy/Hard: W/L/D/U 3/26/1/0; mean 32.7 plies (25–48); 7037688 nodes; node saturation 29.6%; fallbacks 0/982.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 28.8 plies (17–43); 560389 nodes; node saturation 0.0%; fallbacks 0/865.

- Normal/Normal: W/L/D/U 19/8/3/0; mean 33.4 plies (25–50); 1158433 nodes; node saturation 0.0%; fallbacks 0/1003.

- Normal/Hard: W/L/D/U 9/20/1/0; mean 34.0 plies (27–43); 8146106 nodes; node saturation 32.2%; fallbacks 0/1020.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 35.9 plies (23–54); 7979276 nodes; node saturation 30.4%; fallbacks 0/1078.

- Hard/Normal: W/L/D/U 23/4/3/0; mean 35.1 plies (27–52); 8291383 nodes; node saturation 31.0%; fallbacks 0/1054.

- Hard/Hard: W/L/D/U 23/3/4/0; mean 35.1 plies (27–50); 15032165 nodes; node saturation 60.8%; fallbacks 0/1052.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Patterns I: player Easy→Hard sensitivity against Normal: +53.3 points.
- Patterns I Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns I Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
