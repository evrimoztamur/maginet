# Initial campaign assessment — 14 September 2026

The full exploratory survey completed **246 matchups and 7,380 games**, with 30 trials per matchup. It measures these simulated agents, not validated human difficulty. No level, placement, or progression changes were made. The [generated report](report.md) contains every level's matrix, nominal 95% Wilson intervals, lengths, node counts, and progression flags; [metadata](metadata.json) records the exact configuration and catalogue.

The executions produced 3,399 Red wins, 2,965 Blue wins, 1,016 actual draws, and **0 safety-limit terminations**. Draws count as resolved non-wins in Red's win rate. These pooled counts are an accounting check, not an overall campaign difficulty score. Across 205,492 moves, searches visited 1,013,412,581 nodes and used 0 legal fallbacks. The 200-ply cap was sufficient for this sample; it is not a guarantee for other seeds or profiles.

Rite III and Rite IV share the same level code and saved-progress key. Their nine pairs of matchup records are identical despite different presentation styles. They are retained as two portals, as requested, but provide no independent evidence: the 7,380 executions cover 7,110 distinct scenario/profile/trial combinations. This is existing content, not a catalogue extraction error.

## Progression findings

Normal-versus-Normal is the primary comparison. Edges follow cardinal adjacency and increasing shortest distance from the tutorial, rather than catalogue order or map column alone. The automated ≥20-percentage-point flags are:

- Basics I → Basics II: 33.3-point drop; supported candidate.
- Basics III → Basics IV: 46.7-point drop; supported candidate.
- Basics III → Challenge I: 50.0-point drop; supported candidate.
- Patterns II → Diagonals I: 23.3-point drop; followup candidate.
- Patterns III → Beams I: 26.7-point drop; followup candidate.
- Patterns III → Challenge II: 36.7-point drop; supported candidate.
- Diagonals III → Shields I: 46.7-point drop; supported candidate.
- Challenge II → Rite II: 20.0-point drop; followup candidate.
- Rite I → Challenge III: 33.3-point drop; followup candidate.
- Ascension I → Ascension II: 26.7-point drop; followup candidate.

“Supported” means disjoint marginal Wilson intervals and at most 10% unresolved games at both endpoints under the requested rule. It is not a definitive human difficulty rating or a multiple-comparison-adjusted finding. Thirty trials leave substantial uncertainty; overlapping intervals make the other flags followup candidates, not evidence of equal difficulty.

The entrance ramp merits attention: Basics I gives Normal players 30/30 wins, versus 20/30 on Basics II. Basics III returns to 26/30 before Basics IV falls to 12/30 and Challenge I to 11/30. The later Patterns III → Challenge II transition falls from 21/30 to 10/30. These are named places to investigate rather than reasons to automatically reorder or weaken levels.

Low win rates have different causes. Shields I's Normal/Normal result is **0 wins, 4 losses, 26 draws**; Rite II has **4 wins, 7 losses, 19 draws**. Compare that with Shields II's **5 wins, 23 losses, 2 draws**. A draw-heavy stalemate problem needs different investigation from repeated losses. Treating all three as a single difficulty ranking would hide that distinction.

## Skill sensitivity and opponent reversals

The largest observed Easy→Hard player changes against a Normal opponent are:

| Level | Easy player win rate | Hard player win rate | Difference |
|---|---:|---:|---:|
| Beams III | 16.7% | 93.3% | +76.7 points |
| Patterns I | 3.3% | 76.7% | +73.3 points |
| Rite I | 6.7% | 76.7% | +70.0 points |
| Patterns III | 30.0% | 96.7% | +66.7 points |
| Basics IV | 13.3% | 80.0% | +66.7 points |

These large differences show sensitivity to search depth, budget, and ranked sampling. They are not estimates of the effect of human practice or experience.

The largest adjacent opponent-profile reversal is Shields I with a Hard player: moving the opponent from Easy to Normal raises Red wins from 8/30 to 10/30 (+6.7 points). No reversal reaches the report’s 20-point screening threshold. The sample does not establish a general ordering of profiles. Small reversals can arise from the different ranked policies and position-dependent search behavior; larger paired-seed followups are needed to distinguish a stable reversal from sampling variation.

## Search budgets and tutorial

The search statistics below pool both sides of all executed games, including the repeated Rite scenario:

| Profile | Searches | Node-cap stops | Cap-stop fraction | Completed-depth counts |
|---|---:|---:|---:|---|
| Easy | 68,956 | 0 | 0.0% | 2: 68,956 |
| Normal | 68,475 | 633 | 0.9% | 3: 633, 4: 67,842 |
| Hard | 68,061 | 38,305 | 56.3% | 4: 329, 5: 10,810, 6: 15,675, 7: 11,491, 8: 29,756 |

Hard's eight-ply target is often constrained by its 20,000-node cap. Results therefore describe the completed search depths at these budgets. The bounded table and completed-iteration fallback remain in effect. These caps cannot be read as equivalents of the browser's 50/150/500 ms settings. The report names individual saturated matchups for targeted budget checks.

The tutorial is assessed separately with rule stalemates disabled and an Easy opponent:

| Player | Wins / losses / draws / unresolved | Resolved Red win rate [95% Wilson] |
|---|---|---|
| Easy | 28 / 2 / 0 / 0 | 93.3% [78.7%, 98.2%] |
| Normal | 30 / 0 / 0 / 0 | 100.0% [88.6%, 100.0%] |
| Hard | 29 / 1 / 0 / 0 | 96.7% [83.3%, 99.4%] |

This tests the tutorial's underlying game against automated players, not the clarity or effectiveness of its guided instructions. It is excluded from Normal/Normal progression edges.

## Named recommendations

1. **Check the entrance ramp.** Run 300 trials per matchup for Basics I–IV and Challenge I with the same profile definitions, then observe human attempts at Basics II, Basics IV, and Challenge I. Confirm the large drops before proposing any content or placement changes.
2. **Check the Patterns-to-Challenge transition.** Prioritize Patterns III and Challenge II for a 300-trial paired-seed followup. Include Patterns II → Diagonals I and Patterns III → Beams I, whose current intervals overlap. Extend other flagged edges listed above to 300 trials; use 1,000 when the estimated drop remains near the 20-point threshold.
3. **Investigate draw-heavy play.** Inspect representative Shields I and Rite II trials alongside Shields II, which predominantly produces losses. Use the checkpoint trial indices to reproduce games through the simulation API and inspect authoritative moves before considering changes. The present survey does not diagnose which tactic or rule causes the draws.
4. **Calibrate the Hard budget.** For Beams III, Patterns I, Rite I, and the report's saturated late-game matchups, compare 20,000 with 80,000 Hard nodes at the same eight-ply ceiling and paired seeds. Check completed depths and outcomes, then separately benchmark browser time budgets. Do not assume that simply increasing the Hard label makes every position harder.
5. **Confirm the duplicated Rite content.** Decide whether identical Rite III/IV scenarios are intentional in a separate content review. Preserve both positions and their shared saved-progress key until that decision; this stage makes no migration or rebalancing change.
6. **Validate with people before balancing.** Use this survey to select human playtest cases, including draw-heavy levels and highly skill-sensitive positions. Keep editor analysis UI and campaign rebalancing as separate followups.

The [verification record](verification.md) documents the implementation checks and complete-data audit. Larger followups should use new output directories because resumption correctly rejects changes to configuration, catalogue, or engine fingerprint. The first 30 trial indices retain the same seeds when only the requested game count increases.
