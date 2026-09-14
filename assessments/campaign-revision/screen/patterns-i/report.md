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
| Easy | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 100.0% [88.6, 100.0]; n=30; U=0; range 100.0–100.0% | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | 43.3% [27.4, 60.8]; n=30; U=0; range 43.3–43.3% |

- Easy/Easy: W/L/D/U 13/16/1/0; mean 30.8 plies (23–45); 43376 nodes; node saturation 0.0%; fallbacks 0/923.

- Easy/Normal: W/L/D/U 2/25/3/0; mean 28.1 plies (20–43); 331216 nodes; node saturation 0.0%; fallbacks 0/842.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 32.8 plies (16–50); 5664669 nodes; node saturation 21.6%; fallbacks 0/983.

- Normal/Easy: W/L/D/U 24/5/1/0; mean 30.7 plies (21–46); 351886 nodes; node saturation 0.0%; fallbacks 0/920.

- Normal/Normal: W/L/D/U 23/6/1/0; mean 32.6 plies (21–43); 755427 nodes; node saturation 0.0%; fallbacks 0/979.

- Normal/Hard: W/L/D/U 3/27/0/0; mean 35.5 plies (24–47); 6864736 nodes; node saturation 24.6%; fallbacks 0/1064.

- Hard/Easy: W/L/D/U 30/0/0/0; mean 31.8 plies (23–43); 6248938 nodes; node saturation 26.7%; fallbacks 0/954.

- Hard/Normal: W/L/D/U 23/5/2/0; mean 32.6 plies (22–52); 7444445 nodes; node saturation 29.0%; fallbacks 0/979.

- Hard/Hard: W/L/D/U 13/17/0/0; mean 35.4 plies (21–52); 13805481 nodes; node saturation 50.7%; fallbacks 0/1061.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Patterns I: player Easy→Hard sensitivity against Normal: +70.0 points.
- Patterns I Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Patterns I Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
