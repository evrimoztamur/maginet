# Campaign revision interpretation

The revised opening battles and powerup introductions are more forgiving for these agents. Combat begins much more reliably in Diagonals I and Rite II. This does **not** establish smooth human difficulty: Patterns I, Rite II and Rite IV still have substantial inactivity endings, and automated players can sometimes skip an introduction's pickup.

The [combined report](report.md) contains all 318 matchup cells, the 300-trial paired comparisons, explicit attribution of untouched baseline results, ordered main-route comparisons, optional-route comparisons, and replay accounting. The [original assessment](../campaign-seed-1/interpretation.md) is unchanged. All eight original 300-trial followups reproduce their existing first 30 records, including seeds, outcomes and search telemetry. All final first-30 records match their corresponding screen. The overlap is not additional independent evidence.

## Content decisions

- **Basics II:** Red starts with four mana instead of two, retaining the one-versus-two encounter and opposing forces. Normal/Normal wins rise from 218/300 to 256/300. Six draws remain. Every game includes damage; eight revised wins end on inactivity, so the improvement is not exclusively elimination wins.
- **Basics IV:** Both Red mages start with three mana instead of two. All six obstacles, mage patterns and positions remain. Wins rise from 112 to 283; 32 wins are awarded after inactivity. The obstacle lesson still has legal routes through the same board. This is deliberately forgiving, not evidence that its human difficulty is calibrated.
- **Patterns I:** The first mana-only candidate raised wins to 152/300 but produced 170 inactivity endings (90 wins). It is preserved under [candidates](candidates/README.md). The accepted candidate also reduces the board from 6×6 to 5×5, retaining every mage/obstacle position and giving Red five mana. It produces 212 wins, 70 losses and 18 draws, with 125 inactivity endings including 87 wins. Original counts were 96/156/48 and 146 inactivity endings including 30 wins. The smaller board increases elimination wins from 62 in the first candidate to 125, but the remaining mana-decided wins mean passivity is **not solved**.
- **Diagonals I:** A 4×3 board, one adjacent diagonal pickup, and a five-mana Red Plus mage against three-mana Blue provide an accessible lesson. No other pickup or obstacle distracts from diagonal movement. Wins rise from 86 to 260; no draws remain in 300 trials, but two wins still use inactivity.
- **Beams I:** A compact 5×4, two-versus-two encounter uses only two beam pickups. Red has four mana per mage and Blue three. Inspected beams can strike two enemies; taking a beam at an unhelpful time is not always fatal. Results are 222 wins, 76 losses and two draws, with nine inactivity wins.
- **Shields I:** The retreat space shrinks from 8×3 to 4×3, with two shields and no diagonal pickups or obstacles. Red's five mana lets it survive reflected damage against three-mana Blue. All 300 Normal/Normal trials win; 299 end without legal moves and one on inactivity. This is a teaching encounter, not a win-rate target. Some winning play bypasses shields, so effectiveness as a human lesson remains unvalidated.
- **Rite II:** The board shrinks from 8×8 to 6×5, the two-versus-two forces start on rows 1 and 3, and only two central obstacles remain. Diagonal and shield choices remain. Original play includes 188 entirely damage-free games; revised play includes none. Draws fall from 214 to 88, but 176/300 endings still use inactivity. This is improved contact, **not a solved draw problem**.
- **Rite IV:** The duplicate is replaced by a distinct 5×5, three-versus-three battle with all three pickups and two obstacles. Results are 139 wins, 101 losses and 60 draws; 137 endings use inactivity. The previous duplicate's 133/115/52 is a scenario comparison, not a claim of equivalent content. Multiple approaches work, but substantial unresolved design concerns remain around late inactivity and draws.

No optional battle was softened. Apart from the eight added junction battles, all other scenario definitions, tutorial instructions, AI profiles, shared rules, win-required progression, and the 200-ply cap remain unchanged.

## Inspected deterministic play

Trial numbers below are zero-based and refer to the named `final/<battle>/matchup-00-1-1.json` traces unless marked original. Coordinates are zero-based. Each hit entry represents one point of mana damage; shield reflection can include the attacking mage's own tile.

- **Diagonals I, win 0:** Red collects at (1,1) on ply 1, then moves diagonally to (2,2) on ply 3 and damages Blue at (2,1). This is a useful diagonal move, not a passive win. **Win 2** initially declines the adjacent pickup, collects it on ply 5 and recovers. Twenty winning trials decline the immediate pickup. **Loss 3** delays repeatedly, lets Blue collect on ply 8, and is eliminated on ply 16. Original draw 8 is among the original passive examples; 69 original trials never deal damage.
- **Beams I, win 0:** After an unproductive Red move on ply 3, Red's ply-5 beam hits Blue at both (0,2) and (3,1); Blue answers with its own two-target beam. Red still wins on ply 11 with two mana remaining. **Win 6** wastes a beam on ply 11 but recovers; 19 winning trials include a Red beam pickup with no damage. **Loss 5** allows Blue's beam first and uses Red's remaining beam without a target. **Draw 88** shows that compactness does not rule out a tied ending.
- **Shields I, win 1:** Both sides collect shields in the first two plies. Red takes reflected damage on plies 3 and 5, surviving with three mana as Blue reaches zero. **Win 2** bypasses both shields entirely, an explicit limitation of the teaching evidence. There are no revised Normal/Normal losses or draws to inspect; the complete nine-matchup screen remains available for weaker-player play. Original draw 0 retreats at one mana each until inactivity; original draw 2 ends with a reflected hit eliminating both mages on ply 14. Of the original 229 draws, 156 are no-legal-move endings and 73 inactivity endings, so the original problem was not purely passive retreat.
- **Patterns I, compact win 0 / loss 3 / draw 20:** These finish after 37/32/39 plies with 23/18/25 total hits. The draw follows substantial combat. The additional board revision was selected for more elimination wins and fewer inactivity endings than the mana-only candidate, rather than its win rate alone.
- **Rite II, original draw 0:** The forces never make contact. **Revised win 1** starts damage on ply 1; **loss 0** starts on ply 2; **draw 2** has repeated damage and pickup exchanges before the final retreat. The draw remains a real failure to finish, despite improved contact.
- **Rite IV, win 0:** Shields are collected on plies 2/3, diagonals on 5/6, and Red takes the central beam on 7, with reflected damage included. Red wins after 35 plies and 23 hits. **Loss 5** and **draw 1** both collect the three mechanic types and finish through inactivity after 20/22 hits. The screen includes wins starting with each of (2,4)→(2,3), (0,3)→(1,3), and (4,3)→(3,3): central pressure, diagonal access, and shield access are all represented.

These are observed recovery examples, not controlled human mistake experiments. Across the accepted final datasets, no game is entirely damage-free and no game reaches the 200-ply cap. Inactivity wins are separately counted rather than presented as elimination wins. Zero observed unresolved games does not establish that every seed resolves.

## Remaining uncertainty

The full matrices show profile sensitivity and difficult optional opponents; no target win-rate threshold is used to accept or reject them. Thirty trials for untouched content and off-diagonal matchups leave broad intervals. The 300-trial followups narrow uncertainty only for Normal/Normal on revised/original targets. Multiple comparisons are exploratory, and the Wilson intervals are marginal intervals rather than paired-difference intervals. Hard's fixed node cap continues to constrain its nominal depth in larger battles.

Human playtests should prioritize whether players notice and understand the three introductions, whether the forgiving Basics IV creates a perceived jump into Patterns, and whether late Rite draws are frustrating. Those questions remain open.


## Cardinal-neighbour junction battles

The final map preserves the named original routes and adds eight distinct 1v1 battles to fill every corridor cell. Junction I is the shared split after Shields I; Junctions II–V connect Challenge III to Rite IV; Junctions VI–VIII connect Challenge IV to Rite IV. No connection skips a cell or runs diagonally. There are 36 portals and 40 unique connections. Demo membership remains the tutorial and four Basics battles.

Each added battle was screened with 30 trials in all nine matchups and followed with 300 Normal/Normal trials. They have no original counterpart, and their explicit seed namespaces are their own canonical scenario codes. Their first 30 final records exactly reproduce the screen.

| Junction | Normal/Normal W/L/D/U | Inactivity endings | No-damage games |
|---|---|---:|---:|
| I | 264/34/2/0 | 1 | 0 |
| II | 227/73/0/0 | 0 | 0 |
| III | 300/0/0/0 | 2 | 0 |
| IV | 266/34/0/0 | 0 | 0 |
| V | 254/43/3/0 | 13 | 0 |
| VI | 286/14/0/0 | 0 | 0 |
| VII | 300/0/0/0 | 0 | 0 |
| VIII | 277/19/4/0 | 10 | 0 |

The inspected screen wins finish in 5–11 plies; available losses finish in 8–14 plies. Junction VIII has two inactivity endings in the screen, and final V/VIII retain occasional inactivity endings and draws. These are compact combat encounters, not automatic unlocks: each requires a win and has its own progress key and star. The full matrices and replay accounting include all eight. Their human difficulty is unvalidated.

Earlier target-scenario datasets retain their original execution metadata, including the pre-junction layout and main route. This provenance is intentionally preserved. The combined report reuses those battle results against the current exported graph, since progression metadata cannot affect independent battle simulations. Resuming an archived run against changed layout metadata is correctly rejected; fresh runs use new output directories.
