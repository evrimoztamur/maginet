# TODO

- Solver
  - Follow up candidate campaign spikes with 300–1,000 paired-seed trials and human playtesting
  - Investigate draw-heavy Shields I / Rite II and calibrate saturated Hard search budgets
  - Review whether duplicate Rite III / Rite IV content is intentional (preserve progress until decided)
- iOS/Android
  - Move to a wgpu/winit based setup
  - Drag/drop gestures
  - Use Compound Interest platform
- Bugs
  - Editor prefab online lobbies dont show up
  - Rematch doesnt allow you to select/move
  - Close menu after successful rematch
  - Join/leave indications
  - Player connection status?
- Editor
  - Simulation results interface
  - serde for level styles

## DONE

- UX and learning pass
  - Enlarge audio, editor, main/campaign, battle menu/undo/rematch/continue controls
  - Rebalance button sizes and gutters; center editor rows and restore board-edge dimension controls with room for larger boards
  - Match difficulty choices to the On/Off button group design
  - Explain the campaign reset on its first press and count down remaining presses
  - Keep campaign focus near displayed portals while dragging and snapping
  - Show neutral attack patterns on selection, above sleepers/stones and beyond board edges
  - Inspect enemy mages without movement controls; hide the cursor on touch devices
  - Teach movement, spells/patterns and undo through automatic hints; show final blow only for a winning move
  - Follow tutorial victory with diagonal, shield, and beam explanation slides
  - Replace Diagonals I with an X-mage duel requiring the rune to attack; retain old stars
  - Add temporary reviewer-only level unlocking without forging completion stars
  - Verify workspace rules, desktop/touch browser flows, mobile bridge events, and Android compilation/unit tests

- Scenario analyser and exploratory campaign assessment
  - Shared full/demo catalogue, preserving codes, positions, styles, tutorial identity, and saved progress
  - Native analyse/campaign/generate CLI with deterministic node budgets and bounded parallelism
  - Fallible code parsing, reproducible trials, incremental checkpoints, and verified resumption
  - Per-game telemetry, aggregate JSON, Wilson intervals, unresolved bounds, and skill matrices
  - Full 30-game survey: 246 matchups / 7,380 games, with named followup recommendations in assessments/campaign-seed-1/interpretation.md
  - Workspace/demo tests, native/Wasm builds, browser campaign smoke check, and independent complete-data audit

- Solver foundation and AI difficulty
  - Shared iterative alpha-beta search with deterministic evaluation and terminal handling
  - Bounded transposition cache, ordering, node/deadline limits, and completed root scores
  - Seeded Easy/Normal/Hard selection and saved settings; tutorial/menu Easy overrides
  - Worker searches with revision checks, cancellation, and legal failure fallback
  - Exhaustive comparisons, release benchmarks, and automated browser lifecycle checks

- Pixel-grid rendering
  - Snap final sprite transforms to whole native pixels, including jumps and flips
  - Render game and UI at native resolution before enlargement
  - Fit integer device-pixel scaling to the window without smoothing
  - Keep mouse/touch mapping aligned through scaling and portrait rotation

- Campaign progression
  - Add a gently bobbing star counter with earned/total stars
  - Put the guided tutorial at the campaign entrance and require completion
  - Hide distant locked level names with ???
  - Use later tilesets in battles and their map portals
  - Twinkle only when returning from a newly completed level
  - Preserve completed levels when replaying and losing

- Animations
  - Decouple visual positions, health, pickups, and turn UI from game state
  - Queue turn playback and one-shot UI signals
  - Animate pawns with a gentle 12-pixel lift and sharp landing
  - Flip attacking sprites for 100 ms, starting 50 ms after missile launch
  - Accelerating 200 ms magic missiles with fireworks on impact
  - Fill travelled missile segments each frame with cropped spritesheet-quadrant trails
  - Animate undo in reverse turn order, restoring health and pickups on landing
  - Keep tutorial hints and queued online turns consistent with playback

- Lobby list UI
  - Test expiration and timestamps
  - Align buttons
  - Test with many lobbies
  - Profile jank on lobby list load (did not exist with two lobbies)
  - Replace teleport sign UI with lobby number
  - Find lobby button
  - Include powerups in preview
- UX
  - Clarify which side is you
- Deployment
  - Fix canvas sizing
  - Disable hotkeys for debugging on deploy version
  - Cropped demo ex. multiplayer and only 3-4 levels
  - Demo
    - Replace itch version
    - Replace evrim.zone version
    - Replace Steam version
- Arena
  - Pan/Move and lock onto levels
  - Previewing levels from list of codes/positions
  - Level names
  - Lock levels
  - Battle/Locked buttons
  - Select levels
  - Enter level and exit at same position back into Arena UI
  - Track wins
    - Record win upon exiting from won state
  - Lock level if no explicit incoming connection from a won battle
  - Flipping mages when level won (+ medal from Mrmo)
  - 31 arena portals: 30 battles plus the tutorial, including five hidden levels
- Mechanics
  - Powerups
    - Diagonals
    - Plus-beam
    - Defensive mode
    - Rock as obstacle powerup
- Level/Editor
  - Select tileset
  - Fix menu state logic
  - Incorporate other boulder styles

## SKIP

- Tutorial
  - Hint that you can move diagonally for the final blow
  - Place the player into a 2v2 mini-battle for them to test their skills after they successfully finish the tutorial
  - Explain that the staff icon corresponds to the pattern
- Mechanics
  - Trial new mechanics
    - Board shenanigans
      - Force-shift all mages cardinal directions
      - Piece modifiers <https://nestorgames.com/rulebooks/ESSENTIA_EN.pdf>
    - Piece shenanigans
      - Promotion
        - On kill receive diagonals?
        - On low health turn defensive?
- Sound
  - State transitions
    - State requests background music
    - App handles transitions between pieces
    - Fade in-out
- Documentation UI
  - Table of contents
  - Text plus rendered image
    - Hardcode?


## Campaign revision followups

- Implemented: shared explicit progression graph with cardinal-neighbour links, three optional practice paths and an ordered post-capstone challenge series, focused powerup introductions, distinct Rite IV, profile selection, paired seed namespaces, deterministic replays, staged assessments and source-attributed combined matrices.
- Human playtests: verify that each pickup introduction communicates its mechanic and tolerates a novice mistake; automated recovery examples do not validate instruction quality.
- Combat redesign completed for Patterns I, Rite II and Rite IV: inactivity endings fell to 51/19/49 per 300 Normal/Normal games. Human-test the remaining retreat loops and immobilization endings; see assessments/campaign-combat-redesign/interpretation.md.
- Check perceived difficulty after the forgiving Basics battles and Shields I. Keep optional challenges demanding.
- See `assessments/campaign-revision/interpretation.md` for evidence and limitations; preserve the original assessment.

- Exploration layout: removed all eight filler Junctions; grouped numbered practice battles and Challenges into coherent paths; arrows originate only at playable, uncompleted battles. Human-test map discoverability, the required Rite sequence, and the Challenge I–IV series; its numbering is route order, not a validated difficulty ranking.

- Hidden campaign additions: Crossfire beam puzzle, bidirectional Side Step diagonal puzzle, and a green 4×4 Chaos Ascension III with stable completion. Verify human discoverability and puzzle clarity; randomized Ascension III is excluded from fixed-scenario statistics.
