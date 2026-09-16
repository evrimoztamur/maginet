# Assessment

Keep the pickup reset and eight full-turn inactivity clock. Across the unchanged catalogue, the two rule changes reduced draws from 662 to 448 in 7,920 matched games. The AI tie preference reduced that further to 402: 39.3% fewer draws than the control. Control already includes structural-deadlock overcharge, so this isolates what the new inactivity and AI changes add.

Most earlier draws were timeouts in positions where attacks remained possible. They were not proofs of structural deadlock. The bitmask detector should remain conservative: its optimistic movement model can prove that no attack is possible, while a possible attack in that model does not establish a playable or forced route to damage. Powerups are still excluded from that proof. Timeout resolution remains necessary.

The change buys the players time to reposition and treats an item pickup as progress. The rule-only arm adds roughly seven plies to the mean game (24.75 to 31.93), and the 95th percentile rises from 51 to 73. With the AI preference the mean is 31.80, the 95th percentile remains 73, and the longest observed game is 143. No arm reached the 200-ply safety cap. More time is a real pacing cost, not a free balance improvement.

Fewer timeouts also produce more eliminations: 4,366 to 5,832. Inactivity endings fall from 3,494 to 2,018, but only 402 of the latter are equal-mana draws; the others are decided by the existing mana tiebreak. Overcharge activates only 43 times in the combined screen. Broadening overcharge merely because the timer is near expiry would conflate these different situations.

The AI change is a small preference after search: an equally scored move leading to a less-visited position ranks first. It does not spend more search nodes, change evaluation scores, penalize a necessary repeated position below a worse move, or alter the difficulty sampling weights. Actual search remains identical until the final root ordering. Replay and shield state form part of the identity; the inactivity clock is deliberately excluded from repetition identity. This is a preference, not a repetition ban or a new draw rule.

The AI increment is promising but modest: 46 fewer draws across the screen, with 138 former draws resolved and 92 new draws. In the six preselected 270-trial holdouts, none of the AI-only win/draw changes survives the 24-test Holm adjustment. The combined rule package has confirmed draw reductions in Patterns II, Side Step, Challenge II and Challenge IV. It also increases Red wins in Side Step and Challenge II. Other individual changes remain uncertain. See the report for exact counts and adjusted tests.

The recommendation is to keep all three changes for playtesting, with the longer inactivity window as the main contributor. Pooled profile mixtures describe this benchmark, not human success rates. The separate campaign teaching assessment holds these rules constant while changing puzzles, so its difficulty changes must not be attributed to the AI tie preference.

The green Deadlock banner remains a 250 ms entrance, one second centered, and 250 ms exit. Browser rendering confirms the intended color and approximately 15/60/15 frames at 60 Hz. Inactivity pips fill once per two quiet plies and reset when a rune is picked up, including an immediately consumed Beam. Tutorial inactivity remains disabled. Nothing has been merged.
