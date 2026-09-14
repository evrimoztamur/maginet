# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Rite II

Map (10, -1); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% | 0.0% [0.0, 11.4]; n=30; U=0; range 0.0–0.0% |
| Normal | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 36.7% [21.9, 54.5]; n=30; U=0; range 36.7–36.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Hard | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 50.0% [33.2, 66.8]; n=30; U=0; range 50.0–50.0% | 26.7% [14.2, 44.4]; n=30; U=0; range 26.7–26.7% |

- Easy/Easy: W/L/D/U 12/6/12/0; mean 25.9 plies (16–44); 31791 nodes; node saturation 0.0%; fallbacks 0/777.

- Easy/Normal: W/L/D/U 2/19/9/0; mean 22.9 plies (16–32); 203157 nodes; node saturation 0.0%; fallbacks 0/686.

- Easy/Hard: W/L/D/U 0/28/2/0; mean 22.3 plies (16–31); 4484366 nodes; node saturation 26.0%; fallbacks 0/670.

- Normal/Easy: W/L/D/U 22/4/4/0; mean 23.9 plies (15–33); 208458 nodes; node saturation 0.0%; fallbacks 0/717.

- Normal/Normal: W/L/D/U 11/13/6/0; mean 23.1 plies (15–35); 402219 nodes; node saturation 0.0%; fallbacks 0/693.

- Normal/Hard: W/L/D/U 3/25/2/0; mean 23.0 plies (16–33); 4599961 nodes; node saturation 24.9%; fallbacks 0/690.

- Hard/Easy: W/L/D/U 22/5/3/0; mean 24.6 plies (13–38); 4795282 nodes; node saturation 24.4%; fallbacks 0/738.

- Hard/Normal: W/L/D/U 15/7/8/0; mean 25.1 plies (14–34); 5237388 nodes; node saturation 27.4%; fallbacks 0/752.

- Hard/Hard: W/L/D/U 8/15/7/0; mean 24.2 plies (15–35); 9429011 nodes; node saturation 50.4%; fallbacks 0/726.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Rite II: player Easy→Hard sensitivity against Normal: +43.3 points.
- Rite II Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite II Normal/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite II Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite II Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
