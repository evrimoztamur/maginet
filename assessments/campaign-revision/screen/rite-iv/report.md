# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 9 / 9 matchups complete, 270 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-e9168b4395067652`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Rite IV

Map (12, -1); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | 56.7% [39.2, 72.6]; n=30; U=0; range 56.7–56.7% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% | 23.3% [11.8, 40.9]; n=30; U=0; range 23.3–23.3% |
| Hard | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 73.3% [55.6, 85.8]; n=30; U=0; range 73.3–73.3% | 46.7% [30.2, 63.9]; n=30; U=0; range 46.7–46.7% |

- Easy/Easy: W/L/D/U 17/9/4/0; mean 25.6 plies (16–40); 49248 nodes; node saturation 0.0%; fallbacks 0/767.

- Easy/Normal: W/L/D/U 2/26/2/0; mean 25.8 plies (13–45); 465252 nodes; node saturation 0.0%; fallbacks 0/773.

- Easy/Hard: W/L/D/U 3/24/3/0; mean 27.2 plies (18–49); 5032497 nodes; node saturation 25.8%; fallbacks 0/815.

- Normal/Easy: W/L/D/U 29/1/0/0; mean 23.3 plies (15–33); 444813 nodes; node saturation 0.0%; fallbacks 0/699.

- Normal/Normal: W/L/D/U 14/7/9/0; mean 27.6 plies (19–35); 835485 nodes; node saturation 0.0%; fallbacks 0/827.

- Normal/Hard: W/L/D/U 7/18/5/0; mean 25.5 plies (16–41); 4762712 nodes; node saturation 24.2%; fallbacks 0/764.

- Hard/Easy: W/L/D/U 26/1/3/0; mean 25.8 plies (13–37); 4942320 nodes; node saturation 27.0%; fallbacks 0/775.

- Hard/Normal: W/L/D/U 22/6/2/0; mean 25.2 plies (15–34); 5122445 nodes; node saturation 26.4%; fallbacks 0/757.

- Hard/Hard: W/L/D/U 14/12/4/0; mean 28.3 plies (18–41); 11116375 nodes; node saturation 55.8%; fallbacks 0/850.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Rite IV: player Easy→Hard sensitivity against Normal: +66.7 points.
- Rite IV Easy/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
- Rite IV Hard/Easy: Red search-budget saturation ≥50%; compare larger node caps.
- Rite IV Hard/Normal: Red search-budget saturation ≥50%; compare larger node caps.
- Rite IV Hard/Hard: Red search-budget saturation ≥50%; compare larger node caps.
- Rite IV Hard/Hard: Blue search-budget saturation ≥50%; compare larger node caps.
