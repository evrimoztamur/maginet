# Combat redesign: decisions and limits

The three accepted redesigns finish through combat substantially more often in the 300-trial Normal/Normal followups. They retain the campaign’s 36 portals and every cardinal connection. The [full report](report.md) includes all eight candidate screens, paired comparisons, current full campaign matrices with reused results explicitly attributed, and progression flags. Earlier assessments remain unchanged.

| Battle | Inactivity endings before → after | Elimination endings before → after | Draws before → after |
|---|---:|---:|---:|
| Patterns I | 125 → 51 | 158 → 220 | 18 → 17 |
| Rite II | 176 → 19 | 123 → 279 | 88 → 8 |
| Rite IV | 137 → 49 | 159 → 234 | 60 → 17 |

All counts are out of 300 games. Elimination requires no legal moves and at least one side at zero mana; other no-legal-move endings are reported separately rather than treated as eliminations. No selected final trial is damage-free or reaches the unchanged 200-ply limit.

## What changed

**Patterns I — a smaller mixed-pattern battle with equal mana.** The board is now 4×4, with three mages per side using Knight, Cross and Plus patterns. Both sides start with four mana per mage. Removing the two obstacles and peripheral retreat space keeps the focus on overlapping attack patterns; there are no powerups before their introductions. The Plus mage adds direct pursuit alongside the more positional Knight and Cross patterns. Red no longer depends on the previous five-mana advantage.

The first two candidates retained a Red mana advantage and won 29/30 and 28/30 screen trials. They ended combat reliably but risked making Patterns I much easier than the unchanged Patterns II. The equal-mana candidate was selected for its tighter board, varied attacks and comparable difficulty to the preceding design: final Red wins are 204/300 versus 212/300 previously. Inactivity wins fall from 87 to 22. Draws barely change, so this is a reduction in passive endings rather than a demonstrated reduction in draw frequency.

**Rite II — open crossfire.** A 4×4 board replaces the 6×5 arena and central obstacles. Each side has one Plus and one Spike mage, with four mana each. Two diagonal pickups and one beam provide pursuit and crossfire opportunities. Shields are removed from this battle; the capstone retains shield combinations. Direct attacks can begin on the first ply, and each surviving mage has attacks at adjacent cardinal positions.

The 4×3 alternative had no inactivity endings in its screen but exposed Red to severe opening pressure, producing 20 losses in 30 games. The selected 4×4 version leaves more repositioning room and still sharply reduces inactivity: 19/300 versus 176/300. Red wins rise from 96 to 166, mostly through elimination rather than the inactivity rule.

**Rite IV — a contested center with mobile attackers.** The capstone becomes a 4×4, three-versus-three encounter with Cross, Spike and Plus mages on each side, all at four mana. Two diagonal pickups, one beam and one shield occupy the center. The two edge obstacles and second shield are removed. A single shield still creates reflected-damage decisions without making every late duel a two-shield exchange. The Spike and Plus patterns give survivors more direct pursuit options.

The initial central-pickup candidate gave Red an immediate two-target Knight attack and produced 24/30 wins, but seven inactivity endings. Moving the diagonal pickups to the flanks restored diverse openings but raised inactivity to 12/30. Replacing the Knight/Diamond mix with Cross/Spike/Plus reduced that to three in the screen. The final 300-trial sample has 207 wins, 76 losses and 17 draws, with 49 inactivity endings. Its 207 wins include 68 diagonal-first, 72 Plus-pressure and 67 Cross-pressure openings, providing evidence of multiple viable approaches rather than one dominant scripted pickup.

## Inspected play

Trial indices below are zero-based in the selected final dataset files. Coordinates are zero-based. Complete moves, pickups, damage, mana and termination causes are recorded in every replay.

- **Patterns I, win 0:** Red opens with Plus pressure from (3,3) to (2,3), then follows Blue’s Cross mage across the board. The game ends by elimination after 33 plies. **Loss 5** and **draw 2** still reach inactivity after combat; draw 2 leaves one mana per side. These examples show why the remaining problem is not claimed solved.
- **Rite II, win 0:** Red attacks on ply 1. Blue collects the beam on ply 4, Red gains diagonal movement on ply 5, and the battle ends by elimination on ply 15. **Loss 1** opens with three quiet moves and lets Blue take the beam and both diagonals; Blue eliminates Red on ply 14. **Draw 4** is a remaining retreat loop at two mana each after the pickups have been used.
- **Rite IV, win 6:** Both sides gain diagonals on the first two plies, Red takes the shield on ply 7, and Blue’s ply-12 beam produces three damage entries, including reflection. Red survives and wins on ply 17. **Loss 1** takes an early shield but still loses after 26 plies. **Draw 10** uses all three pickup types yet ends through inactivity after 30 plies. Access to every mechanic is not a guarantee of a decisive finish.

Browser checks entered each redesigned campaign battle and reproduced its first recorded move, including the resulting team mana and AI request. The revised 4×4 capstone was also visually inspected.

## Remaining uncertainty

Patterns I and Rite IV still end through inactivity in approximately one sixth of the final trials. There are also 29, 2 and 17 immobilization endings respectively in Patterns I, Rite II and Rite IV; the tighter boards can block surviving mages. These are neither inactivity nor elimination and should be watched in human playtests.

The unchanged Basics IV → Patterns I transition remains an exploratory difficulty flag, and the stronger new Rite IV result may move some perceived difficulty toward Ascension I. The report shows those ordered comparisons and keeps optional routes separate. No neighbouring battle was changed to chase a numerical target.

All candidates were screened with 30 games across all nine matchups; the three selected candidates received 300 Normal/Normal trials. Their first 30 trials overlap the screens. The preceding 300-trial results use exactly the same seed namespaces, configuration and engine fingerprint, and are reused without rerunning. Eight-candidate selection on one seed namespace is exploratory and can overfit; neither the Wilson intervals nor the larger sample remove selection bias. Human difficulty and lesson quality remain unvalidated.

Dataset metadata reflects the scenarios as standalone candidates before catalogue adoption. Resuming against changed catalogue identity is intentionally rejected. Use a fresh output root for reproduction with current catalogue metadata.
