# Campaign scenario survey

Exploratory simulated-agent difficulty; not validated human difficulty. 1 / 1 matchups complete, 300 games. Red is the player. Seed 1, 300 games per matchup, 200-ply safety limit. Node caps are analysis defaults, not browser time equivalents.

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
| Easy | — | — | — |
| Normal | — | 32.0% [27.0, 37.5]; n=300; U=0; range 32.0–32.0% | — |
| Hard | — | — | — |

- Normal/Normal: W/L/D/U 96/116/88/0; mean 23.6 plies (15–42); 3937382 nodes; node saturation 0.0%; fallbacks 0/7089.

## Main-route progression candidates

Normal/Normal comparisons in teaching-route order. Directed shortest tutorial distances are context only. A ≥20-point drop is supported only with disjoint intervals and ≤10% unresolved at both ends. These exploratory flags are not corrected for multiple comparisons.


### Optional route 1 (final exit one-way)


### Optional route 2 (final exit one-way)


### Optional route 3 (final exit one-way)


### Optional route 4 (final exit one-way)


### Optional route 5 (final exit one-way)


## Sensitivity and followups

Thresholds below are exploratory: ≥20 percentage points for sensitivity/reversals, >10% unresolved, ≥50% searches reaching the node cap. Hard is not assumed stronger in every position. Use larger paired-seed runs to confirm these observations.

