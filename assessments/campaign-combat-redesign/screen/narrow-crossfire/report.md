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
| Easy | 66.7% [48.8, 80.8]; n=30; U=0; range 66.7–66.7% | 16.7% [7.3, 33.6]; n=30; U=0; range 16.7–16.7% | 10.0% [3.5, 25.6]; n=30; U=0; range 10.0–10.0% |
| Normal | 80.0% [62.7, 90.5]; n=30; U=0; range 80.0–80.0% | 33.3% [19.2, 51.2]; n=30; U=0; range 33.3–33.3% | 6.7% [1.8, 21.3]; n=30; U=0; range 6.7–6.7% |
| Hard | 86.7% [70.3, 94.7]; n=30; U=0; range 86.7–86.7% | 96.7% [83.3, 99.4]; n=30; U=0; range 96.7–96.7% | 40.0% [24.6, 57.7]; n=30; U=0; range 40.0–40.0% |

- Easy/Easy: W/L/D/U 20/10/0/0; mean 16.2 plies (10–22); 12399 nodes; node saturation 0.0%; fallbacks 0/486.

- Easy/Normal: W/L/D/U 5/25/0/0; mean 15.3 plies (10–21); 70153 nodes; node saturation 0.0%; fallbacks 0/460.

- Easy/Hard: W/L/D/U 3/27/0/0; mean 17.8 plies (12–30); 1742352 nodes; node saturation 3.4%; fallbacks 0/535.

- Normal/Easy: W/L/D/U 24/6/0/0; mean 16.8 plies (11–29); 77106 nodes; node saturation 0.0%; fallbacks 0/503.

- Normal/Normal: W/L/D/U 10/20/0/0; mean 16.6 plies (12–26); 137284 nodes; node saturation 0.0%; fallbacks 0/498.

- Normal/Hard: W/L/D/U 2/28/0/0; mean 17.9 plies (10–27); 2161568 nodes; node saturation 5.9%; fallbacks 0/538.

- Hard/Easy: W/L/D/U 26/4/0/0; mean 17.3 plies (13–33); 1927339 nodes; node saturation 2.5%; fallbacks 0/520.

- Hard/Normal: W/L/D/U 29/1/0/0; mean 17.6 plies (11–25); 2077024 nodes; node saturation 3.4%; fallbacks 0/527.

- Hard/Hard: W/L/D/U 12/18/0/0; mean 17.5 plies (14–23); 4588651 nodes; node saturation 8.7%; fallbacks 0/526.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

- Custom scenario: player Easy→Hard sensitivity against Normal: +80.0 points.
