# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 1 / 1 matchups complete, 30 games. Red is the player. Seed 1, 30 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

Win rates count draws as resolved non-wins. Cells show resolved Red win % [95% Wilson interval], unresolved count, and possible overall win % range. More than 10% unresolved precludes a definitive ranking. Tutorial uses Easy opponents and disables rule stalemates.

Engine `fnv1a-c440d9b67c0d7740`; bounded table capacity 32768 entries per search.

| Profile | Depth | Nodes/move | Ranked weights |
|---|---:|---:|---|
| Easy | 2 | 1000 | 65/25/10 |
| Normal | 4 | 5000 | 80/15/5 |
| Hard | 8 | 20000 | 90/8/2 |

## Custom scenario

Map (0, 0); distance n/a.

| Player / opponent | Easy | Normal | Hard |
|---|---|---|---|
| Easy | — | — | — |
| Normal | — | 76.7% [59.1, 88.2]; n=30; U=0; range 76.7–76.7% | — |
| Hard | — | — | — |

- Normal/Normal: W/L/D/U 23/7/0/0; mean 3.6 plies (3–10); 15589 nodes; node saturation 0.0%; fallbacks 0/109.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (dead end)


### Optional route 2 (dead end)


### Optional route 3 (dead end)


### Optional route 4 (dead end)


### Optional route 5 (dead end)


### Optional route 6 (dead end)


### Optional route 7 (dead end)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

