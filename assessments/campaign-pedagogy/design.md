# Teaching decisions

The goal is to make each early battle require a decision that follows from the tutorial. Old Basics I simplified the game by removing meaningful enemy counterplay; that is why it is removed. Lower simulated win rates alone would not establish a better lesson.

## Three Basics battles

Coordinates below are zero based. Mana is the current health budget, not a change to the maximum mana system.

| Battle | Construction | Intended decision and evidence |
|---|---|---|
| Basics I | Old Basics II, unchanged. A two-mana Plus against two two-mana enemies. | Move (1,2) → (2,2) to hit both enemies. Exact depth-10 opening search finds a forced win here and forced losses for every other opening. Lowering enemy mana made this a one-move sweep and was rejected. |
| Basics II | Old Basics III, with the Red Diamond reduced from two mana to one; the Knight retains two. Enemies retain two each. | The Knight's (1,3) → (1,2) attack wins against correct replies. Moving the vulnerable Diamond (1,1) → (1,2) loses. Some retreat openings remain unresolved within the horizon, so this is not a claim of a unique solution across every possible game length. |
| Basics III | New 4×4 finale: Red Diamond at (0,3), two mana; Knight at (1,3), one. Blue Plus at (2,0), one; Cross at (3,1), one; Diamond at (0,0), two. | The attractive Knight move (1,3) → (1,2) hits all three enemies but exposes it to an immediate killing reply and loses against correct play. Diamond (0,3) → (0,2) is a forced winning opening. It tests whether the player checks the response instead of blindly maximizing targets hit. |

All three use only movement and attack patterns. No item, obstacle rule or new UI instruction is needed. Native tests complete a depth-10 search and verify the intended winning and losing openings; the finale test also executes the greedy attack and a lethal reply. The terminal scores establish forced outcomes within that horizon, not merely favorable heuristic evaluations.

Reducing the Knight to one mana in the old third board, or strengthening its Blue Diamond, made every opening lose within the same search horizon. Those candidates were rejected. The finale's simulated win rate is higher than new Basics II's, but its lesson requires a different kind of judgment; a monotonic bot win-rate curve was not the selection criterion.

## Later campaign margins

| Battle | Change | Reason |
|---|---|---|
| Diagonals I | Red Cross four → two mana, equal to Blue. | Alignment must be solved with the rune; extra health no longer carries the player. |
| Shields I | Two two-mana Spike mages on a 4×3 board, one Shield. | Taking the Shield at (0,2) from (1,2) also lands a ranged hit. Both opponents can fight, and retaliation changes the subsequent exchange. |
| Patterns I | Red Cross at (1,2), four → three mana. | Keep the mixed-pattern encounter while reducing the margin for an exposed support mage. |
| Patterns III | Red Spike at (1,4), four → three mana. | Make protecting a ranged mage more consequential. |
| Rite IV | Red Cross at (0,3), four → three mana. | Tighten the capstone's error budget without replacing its combined-item problem. |

For both Diagonals I and Shields I, exact opening search verifies a forced win with the rune and a forced loss when it is removed. Shields I also loses on every non-rune opening. These checks establish that the introductory mechanic matters. Simple health reductions on the old Shield board retained its noninteractive structure, so the geometry and mage types changed instead.

Beams I variants were explored but left out: the alternatives did not provide a convincing improvement in both lesson clarity and difficulty. Already demanding optional battles and later encounters remain unchanged. The full catalogue screen verifies their deterministic replays remain identical under the same rules.

## Balance evidence and playtest risks

Candidate exploration covers 42 records (including controls and one duplicate baseline), with 30 Normal/Normal trials each. Eight selected encounters receive 300 trials for each distinct control/revision pair: 15 datasets, because Basics I is an unchanged control. Candidate selection used the screen and opening analysis; the additional 270 trials are reported separately. Corresponding controls and candidates use the same seed namespace, profile budgets and trial indices, even where the board changes. Full move/damage/item traces are retained and authoritatively replayed.

The revised Basics II and finale are substantially less forgiving in these simulations while retaining short forced winning lines. Diagonals I is a much larger difficulty step (94.3% to 52.7% bot wins), and Patterns III also tightens appreciably (72.7% to 49.0%). These are the first human playtest priorities. If they feel obscure, improve the lesson or exposure before adding arbitrary health back. Patterns I's small shift is not statistically confirmed in the holdout.

Rite IV draws rise from 14 to 25 of 300; Patterns III from 23 to 27. Neither draw increase is significant after holdout multiplicity adjustment, but the observations remain relevant to a game whose players dislike draws. The modest Red mana reduction also changes which quiet endings are wins, losses or ties. Watch these encounters rather than assuming all reduced wins reflect better tactical decisions.

## Map, saves and integration

The campaign has three Basics battles, 29 battles in total plus the tutorial, and four demo portals including the tutorial. The later region shifts one map row south so Patterns I is immediately east of Basics III. Cardinal connections, side routes, hidden portals and unlock rules remain intact. Initial visible star total is 25; all 30 portals become visible after discovery.

Existing completions follow the old battles: old II completes new I, old III completes new II, and old IV completes the new finale. Revised named encounters accept their previous canonical save codes. Old Basics I alone does not grant completion of the new, more demanding Basics I.

This work is isolated in `experiment/deadlock-overcharge`. The main checkout has separate tutorial changes through `966746a`, including a Plus opponent and introductory hints. They were read for context but have not been overwritten or pulled into this experiment. This branch's catalogue screen therefore retains its original tutorial fixture. Reconcile the tutorial alias, shared catalogue edits and affected browser fixtures when merging is approved; preserve the user's newer tutorial work. Nothing is merged.
