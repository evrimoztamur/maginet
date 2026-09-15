//! Client-only playback. Rules advance immediately; these snapshots describe what is visible.
use std::collections::VecDeque;

use shared::{Game, Mage, Mages, Position, PowerUp, Turn};

#[derive(Clone, Copy, Debug)]
pub struct DragLanding {
    pub ground: (f64, f64),
    pub offset_y: f64,
}

pub const LANDING_FRAMES: u64 = 8;
pub const MOVE_FRAMES: u64 = 18;
pub const MISSILE_FRAMES: u64 = 12;
const MISSILE_LEAD_FRAMES: u64 = 3; // Launch 50 ms before landing.
const CAST_DELAY_FRAMES: u64 = 3; // Flip 50 ms after launch.
const CAST_FRAMES: u64 = 6; // Hold the flip for 100 ms.

#[derive(Default)]
pub struct MagePose {
    pub offset_y: f64,
    pub flip_x: bool,
}

struct Transition {
    before: Game,
    after: Game,
    turn: Turn,
    hits: Vec<Position>,
    start: u64,
    mage_index: usize,
    reverse: bool,
    landing: Option<DragLanding>,
}

impl Transition {
    fn move_frames(&self) -> u64 {
        if self.landing.is_some() {
            LANDING_FRAMES
        } else {
            MOVE_FRAMES
        }
    }

    fn launch(&self) -> u64 {
        self.start + self.move_frames() - MISSILE_LEAD_FRAMES
    }

    fn end(&self) -> u64 {
        if self.hits.is_empty() {
            self.start + self.move_frames()
        } else {
            self.launch() + MISSILE_FRAMES
        }
    }

    fn progress(&self, frame: u64) -> f64 {
        (frame.saturating_sub(self.start) as f64 / self.move_frames() as f64).min(1.0)
    }

    fn source(&self, target: Position) -> Position {
        if target == self.turn.1 {
            // Entering enemy shield coverage damages the moving mage.
            let team = self.before.live_occupant(&self.turn.0).unwrap().team;
            if let Some(defender) = self.before.iter_mages().find(|mage| {
                mage.is_alive()
                    && mage.is_defensive()
                    && mage.team != team
                    && self
                        .before
                        .targets(mage, mage.position)
                        .iter()
                        .any(|(_, p)| *p == target)
            }) {
                return defender.position;
            }
        }
        self.turn.1
    }
}

/// One-shot cues consumed by the UI, never sent back to the rules engine.
#[derive(Debug, PartialEq)]
pub enum Signal {
    Move,
    Pickup(PowerUp),
    Impact(Vec<Position>),
}

pub struct Presentation {
    settled: Game,
    queue: VecDeque<Transition>,
    sampled_at: Option<u64>,
}

impl Presentation {
    pub fn new(game: &Game) -> Self {
        Self {
            settled: game.clone(),
            queue: VecDeque::new(),
            sampled_at: None,
        }
    }

    pub fn busy(&self) -> bool {
        !self.queue.is_empty()
    }

    pub fn game(&self) -> &Game {
        self.queue.front().map_or(&self.settled, |t| &t.before)
    }

    #[cfg(test)]
    pub fn enqueue(
        &mut self,
        before: Game,
        after: &Game,
        turn: Turn,
        hits: Vec<Position>,
        frame: u64,
    ) {
        self.enqueue_landing(before, after, turn, hits, frame, None);
    }

    pub fn enqueue_landing(
        &mut self,
        before: Game,
        after: &Game,
        turn: Turn,
        hits: Vec<Position>,
        frame: u64,
        landing: Option<DragLanding>,
    ) {
        let earliest = self
            .sampled_at
            .map_or(frame, |previous| frame.max(previous + 1));
        let start = self
            .queue
            .back()
            .map_or(earliest, |t| t.end().max(earliest));
        let mage_index = before.live_occupant(&turn.0).unwrap().index;
        self.queue.push_back(Transition {
            mage_index,
            reverse: false,
            landing,
            before,
            after: after.clone(),
            turn,
            hits,
            start,
        });
    }

    /// Append inverse snapshots after pending playback, preserving continuity on mid-turn undo.
    /// Replaying history supplies restoration state without running moves backward through rules.
    pub fn rewind(&mut self, game: &Game, turns: usize, frame: u64) {
        let mut before = game.clone();
        for _ in 0..turns.min(game.turns()) {
            let turn = before.last_turn().unwrap();
            let after = before.rewind(1);
            // Find identity in the restored snapshot: the mover may currently be asleep.
            let mage_index = after.live_occupant(&turn.0).unwrap().index;
            let earliest = self
                .sampled_at
                .map_or(frame, |previous| frame.max(previous + 1));
            let start = self
                .queue
                .back()
                .map_or(earliest, |t| t.end().max(earliest));
            self.queue.push_back(Transition {
                before,
                after: after.clone(),
                turn: Turn(turn.1, turn.0),
                hits: Vec::new(),
                start,
                mage_index,
                reverse: true,
                landing: None,
            });
            before = after;
        }
    }

    /// Consume crossed boundaries once, even when a frame was skipped.
    pub fn advance(&mut self, frame: u64) -> Vec<Signal> {
        let mut signals = Vec::new();
        while let Some(t) = self.queue.front() {
            let crossed = |at| frame >= at && self.sampled_at.is_none_or(|previous| previous < at);
            if crossed(t.start) {
                signals.push(Signal::Move);
            }
            if !t.reverse && crossed(t.start + t.move_frames()) {
                if let Some(powerup) = t.before.powerups().get(&t.turn.1) {
                    signals.push(Signal::Pickup(*powerup));
                }
            }
            if frame < t.end() {
                break;
            }
            let transition = self.queue.pop_front().unwrap();
            if !transition.hits.is_empty() {
                signals.push(Signal::Impact(transition.hits));
            }
            self.settled = transition.after;
        }
        self.sampled_at = Some(frame);
        signals
    }

    /// Sprite properties and fractional board coordinates are a pure sample of the timeline.
    pub fn mages(&self, frame: u64) -> Vec<(Mage, (f64, f64))> {
        let mut mages: Vec<_> = self
            .game()
            .iter_mages()
            .map(|mage| {
                let mut visual = mage.clone();
                let mut position = (mage.position.0 as f64, mage.position.1 as f64);
                if let Some(t) = self.queue.front() {
                    if t.mage_index == mage.index {
                        let progress = t.progress(frame);
                        let smooth = progress * progress * (3.0 - 2.0 * progress);
                        position = if let Some(landing) = t.landing {
                            (
                                landing.ground.0 + (t.turn.1 .0 as f64 - landing.ground.0) * smooth,
                                landing.ground.1 + (t.turn.1 .1 as f64 - landing.ground.1) * smooth,
                            )
                        } else {
                            lerp(t.turn.0, t.turn.1, smooth)
                        };
                        if progress == 1.0 {
                            visual.position = t.turn.1;
                            visual.powerup = t.after.get_mage(mage.index).unwrap().powerup;
                        }
                    }
                }
                (visual, position)
            })
            .collect();
        mages.sort_by(|(_, (ax, ay)), (_, (bx, by))| ay.total_cmp(by).then(ax.total_cmp(bx)));
        mages
    }

    /// Body-only pose: the ground position and shadow still follow the glide.
    pub fn pose(&self, index: usize, frame: u64) -> MagePose {
        let Some(t) = self.queue.front() else {
            return MagePose::default();
        };
        let mover = t.before.get_mage(t.mage_index).unwrap();
        let mut pose = MagePose::default();
        if mover.index == index {
            let p = t.progress(frame);
            // Rise gently for most of the move, then accelerate sharply into the landing.
            pose.offset_y = if let Some(landing) = t.landing {
                landing.offset_y * (1.0 - p * p)
            } else if p < 0.75 {
                let rise = p / 0.75;
                -12.0 * rise * rise * (3.0 - 2.0 * rise)
            } else {
                let fall = (p - 0.75) / 0.25;
                -12.0 * (1.0 - fall * fall)
            };
        }
        let cast_start = t.launch() + CAST_DELAY_FRAMES;
        if frame >= cast_start && frame < cast_start + CAST_FRAMES {
            pose.flip_x = t.hits.iter().any(|target| {
                let source = t.source(*target);
                if source == t.turn.1 {
                    mover.index == index
                } else {
                    t.before
                        .live_occupant(&source)
                        .is_some_and(|mage| mage.index == index)
                }
            });
        }
        pose
    }

    pub fn powerups(&self, frame: u64) -> &std::collections::BTreeMap<Position, PowerUp> {
        if let Some(t) = self.queue.front() {
            if t.progress(frame) == 1.0 {
                return t.after.powerups();
            }
        }
        self.game().powerups()
    }

    /// Fill the distance travelled since the previous tick, including the impact endpoint.
    /// Sample before advancing so completed turns still contribute their final trail segment.
    pub fn missile_trail(&self, previous: u64, frame: u64) -> Vec<(f64, f64)> {
        let mut particles = Vec::new();
        if frame <= previous {
            return particles;
        }
        for t in &self.queue {
            if frame < t.launch() || previous >= t.end() {
                continue;
            }
            let from = missile_progress(previous.max(t.launch()) - t.launch());
            let to = missile_progress(frame.min(t.end()) - t.launch());
            for target in &t.hits {
                let start = lerp(t.source(*target), *target, from);
                let end = lerp(t.source(*target), *target, to);
                // One cropped sprite every two board pixels keeps fast motion connected.
                let steps = ((end.0 - start.0).hypot(end.1 - start.1) * 16.0).ceil() as usize;
                if previous < t.launch() {
                    particles.push(start);
                }
                for step in 1..=steps {
                    let p = step as f64 / steps as f64;
                    particles.push((
                        start.0 + (end.0 - start.0) * p,
                        start.1 + (end.1 - start.1) * p,
                    ));
                }
            }
        }
        particles
    }

    #[cfg(test)]
    fn missiles(&self, frame: u64) -> Vec<(f64, f64)> {
        self.queue.front().map_or_else(Vec::new, |t| {
            if frame < t.launch() || frame >= t.end() {
                return Vec::new();
            }
            t.hits
                .iter()
                .map(|target| {
                    lerp(
                        t.source(*target),
                        *target,
                        missile_progress(frame - t.launch()),
                    )
                })
                .collect()
        })
    }
}

// Start with forward speed instead of lingering at the caster, then accelerate into impact.
fn missile_progress(elapsed: u64) -> f64 {
    let t = (elapsed as f64 / MISSILE_FRAMES as f64).min(1.0);
    t * (0.5 + 0.5 * t)
}

fn lerp(from: Position, to: Position, t: f64) -> (f64, f64) {
    (
        from.0 as f64 + (to.0 - from.0) as f64 * t,
        from.1 as f64 + (to.1 - from.1) as f64 * t,
    )
}

/// The rules are deterministic: identical prototype, settings and turns describe the same game.
/// Comparing history also avoids depending on hash-set serialization order or draw order.
pub fn same_history(a: &Game, b: &Game) -> bool {
    a.prototype_code() == b.prototype_code()
        && a.can_stalemate() == b.can_stalemate()
        && a.turns() == b.turns()
        && a.turns_since(0)
            .iter()
            .zip(b.turns_since(0))
            .all(|(a, b)| a.0 == b.0 && a.1 == b.1)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use shared::{Board, Level, MageSort, Team};

    use super::*;

    fn game(powerup: Option<PowerUp>, shield: bool) -> Game {
        let red = Mage::new(0, Team::Red, MageSort::Plus, Position(0, 0));
        let mut blue = Mage::new(1, Team::Blue, MageSort::Plus, Position(3, 0));
        if shield {
            blue.powerup = Some(PowerUp::Shield);
        }
        let powerups = powerup
            .map(|p| (Position(1, 0), p))
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        Game::new(
            &Level::new(
                Board::new(4, 4).unwrap(),
                vec![red, blue],
                powerups,
                Team::Red,
            ),
            false,
        )
        .unwrap()
    }

    fn queue(presentation: &mut Presentation, game: &mut Game, turn: Turn, frame: u64) {
        let before = game.clone();
        let hits = game.take_move(turn.0, turn.1).unwrap();
        presentation.enqueue(before, game, turn, hits, frame);
    }

    #[test]
    fn drag_lands_from_release_and_fires_each_effect_once() {
        let mut game = game(Some(PowerUp::Diagonal), false);
        let mut visual = Presentation::new(&game);
        let turn = Turn(Position(0, 0), Position(1, 0));
        let before = game.clone();
        assert!(!game.try_move(turn.0, Position(-1, 0)));
        assert_eq!(game.turns(), 0);
        let hits = game.take_move(turn.0, turn.1).unwrap();
        visual.enqueue_landing(
            before,
            &game,
            turn,
            hits,
            100,
            Some(DragLanding {
                ground: (0.9, 0.1),
                offset_y: -11.5,
            }),
        );
        assert_eq!(game.turns(), 1);
        let mover = |v: &Presentation, f| {
            v.mages(f)
                .into_iter()
                .find(|(m, _)| m.index == 0)
                .unwrap()
                .1
        };
        assert_eq!(mover(&visual, 100), (0.9, 0.1));
        assert_eq!(
            visual
                .mages(100)
                .iter()
                .filter(|(m, _)| m.index == 0)
                .count(),
            1
        );
        assert_eq!(visual.pose(0, 100).offset_y, -11.5);
        assert!(mover(&visual, 104).0 > 0.9);
        assert_eq!(mover(&visual, 108), (1.0, 0.0));
        assert_eq!(visual.pose(0, 108).offset_y, 0.0);
        assert!(visual.missiles(104).is_empty());
        assert_eq!(visual.missiles(105).len(), 1);
        assert_eq!(
            visual.advance(108),
            vec![Signal::Move, Signal::Pickup(PowerUp::Diagonal)]
        );
        assert!(visual.pose(0, 108).flip_x);
        assert!(visual.advance(108).is_empty());
        assert_eq!(
            visual.advance(117),
            vec![Signal::Impact(vec![Position(3, 0)])]
        );
        assert!(!visual.busy());
        assert!(visual.advance(118).is_empty());
    }

    #[test]
    fn glide_and_missile_hold_health_and_result_until_impact() {
        let mut game = game(None, false);
        game.get_mage_mut(1).unwrap().mana.0 = 1;
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(1, 0)),
            100,
        );
        assert!(game.result().is_some());
        assert!(visual.game().result().is_none());
        assert_eq!(visual.mages(100)[0].1, (0.0, 0.0));
        assert_eq!(visual.mages(109)[0].1, (0.5, 0.0));
        assert_eq!(visual.mages(118)[0].1, (1.0, 0.0));
        assert!(visual.missiles(114).is_empty());
        assert_eq!(visual.missiles(121), vec![(1.75, 0.0)]);
        assert_eq!(visual.advance(126), vec![Signal::Move]);
        assert_eq!(visual.game().get_mage(1).unwrap().mana.0, 1);
        assert_eq!(
            visual.advance(127),
            vec![Signal::Impact(vec![Position(3, 0)])]
        );
        assert_eq!(visual.game().get_mage(1).unwrap().mana.0, 0);
        assert!(visual.game().result().is_some());
        assert!(!visual.busy());
        assert!(visual.advance(127).is_empty());
    }

    #[test]
    fn skipped_frames_drain_signals_in_turn_order_once() {
        let mut game = game(Some(PowerUp::Diagonal), false);
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(1, 0)),
            10,
        );
        queue(
            &mut visual,
            &mut game,
            Turn(Position(3, 0), Position(3, 1)),
            10,
        );
        assert_eq!(
            visual.advance(1000),
            vec![
                Signal::Move,
                Signal::Pickup(PowerUp::Diagonal),
                Signal::Impact(vec![Position(3, 0)]),
                Signal::Move
            ]
        );
        assert_eq!(visual.game().turns(), 2);
        assert!(same_history(visual.game(), &game));
        assert!(visual.advance(1001).is_empty());
    }

    #[test]
    fn pickup_changes_at_arrival_and_shield_missile_comes_from_defender() {
        let mut game = game(Some(PowerUp::Diagonal), true);
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(1, 0)),
            0,
        );
        assert_eq!(visual.powerups(17).len(), 1);
        assert!(visual.powerups(18).is_empty());
        assert!(!visual.mages(17)[0].0.has_diagonals());
        assert!(visual.mages(18)[0].0.has_diagonals());
        assert_eq!(visual.missiles(15), vec![(1.0, 0.0), (3.0, 0.0)]);
        assert_eq!(visual.game().get_mage(0).unwrap().mana.0, 4);
        visual.advance(27);
        assert_eq!(visual.game().get_mage(0).unwrap().mana.0, 3);
    }

    #[test]
    fn reset_discards_queued_damage_and_history_ignores_render_order() {
        let mut game = game(None, false);
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(1, 0)),
            10,
        );
        let rewound = game.rewind(1);
        assert!(!same_history(&game, &rewound));
        visual = Presentation::new(&rewound);
        assert!(visual.advance(100).is_empty());
        assert_eq!(visual.game().get_mage(1).unwrap().mana.0, 4);
        let mut unsorted = rewound.clone();
        unsorted.take_move(Position(0, 0), Position(0, 1)).unwrap();
        let mut sorted = unsorted.clone();
        sorted.sort_mages();
        assert_ne!(
            unsorted.iter_mages().next().unwrap().index,
            sorted.iter_mages().next().unwrap().index
        );
        assert!(same_history(&unsorted, &sorted));
    }

    #[test]
    fn no_hit_turn_finishes_on_arrival_and_same_frame_enqueue_still_signals() {
        let mut game = game(None, false);
        let mut visual = Presentation::new(&game);
        assert!(visual.advance(10).is_empty());
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(0, 1)),
            10,
        );
        assert!(visual.advance(10).is_empty());
        assert_eq!(visual.advance(11), vec![Signal::Move]);
        assert!(visual.advance(11).is_empty());
        assert!(visual.missiles(28).is_empty());
        assert!(visual.busy());
        assert!(visual.advance(29).is_empty());
        assert!(!visual.busy());
        assert_eq!(visual.game().turn_for(), Team::Blue);
        assert_eq!(visual.mages(29)[1].1, (0.0, 1.0));
    }

    #[test]
    fn beam_animates_all_actual_hits_including_friendly_fire() {
        let mut level = Level::new(
            Board::new(4, 4).unwrap(),
            vec![
                Mage::new(0, Team::Red, MageSort::Plus, Position(0, 0)),
                Mage::new(1, Team::Blue, MageSort::Plus, Position(3, 0)),
                Mage::new(2, Team::Red, MageSort::Plus, Position(1, 3)),
            ],
            [(Position(1, 0), PowerUp::Beam)].into_iter().collect(),
            Team::Red,
        );
        level.mages[1].mana.0 = 1;
        let mut game = Game::new(&level, false).unwrap();
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(1, 0)),
            0,
        );
        assert_eq!(visual.missiles(21), vec![(1.75, 0.0), (1.0, 1.125)]);
        assert_eq!(
            visual.advance(18),
            vec![Signal::Move, Signal::Pickup(PowerUp::Beam)]
        );
        assert_eq!(visual.game().get_mage(2).unwrap().mana.0, 4);
        assert_eq!(
            visual.advance(27),
            vec![Signal::Impact(vec![Position(3, 0), Position(1, 3)])]
        );
        assert_eq!(visual.game().get_mage(2).unwrap().mana.0, 3);
        assert_eq!(visual.game().get_mage(1).unwrap().mana.0, 0);
    }

    #[test]
    fn lift_lands_and_cast_flip_starts_later_and_lasts_six_frames() {
        let mut game = game(None, true);
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(1, 0)),
            100,
        );
        assert_eq!(visual.pose(0, 100).offset_y, 0.0);
        assert!(visual.pose(0, 113).offset_y < -11.0);
        assert!(visual.pose(0, 117).offset_y > visual.pose(0, 115).offset_y);
        assert_eq!(visual.pose(0, 118).offset_y, 0.0);
        assert_eq!(visual.pose(1, 113).offset_y, 0.0);
        assert!(visual.missiles(114).is_empty());
        assert_eq!(visual.missiles(115).len(), 2);
        for index in [0, 1] {
            // Mover and retaliating shield mage both cast.
            assert!(!visual.pose(index, 114).flip_x);
            assert!(!visual.pose(index, 115).flip_x);
            assert!(!visual.pose(index, 117).flip_x);
            assert!(visual.pose(index, 118).flip_x);
            assert!(visual.pose(index, 123).flip_x);
            assert!(!visual.pose(index, 124).flip_x);
        }
        visual.advance(127);
        assert!(!visual.pose(0, 142).flip_x);
        let mut game = game.rewind(1);
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(0, 1)),
            100,
        );
        assert!(!visual.pose(0, 118).flip_x);
    }

    #[test]
    fn trail_fills_skipped_distance_through_impact_without_repeating() {
        let mut game = game(None, false);
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(1, 0)),
            100,
        );
        assert_eq!(visual.missile_trail(114, 115), vec![(1.0, 0.0)]);
        let trail = visual.missile_trail(114, 127);
        assert_eq!(trail.first(), Some(&(1.0, 0.0)));
        assert_eq!(trail.last(), Some(&(3.0, 0.0)));
        assert!(trail
            .windows(2)
            .all(|pair| (pair[1].0 - pair[0].0).abs() <= 0.0625));
        assert!(visual.missile_trail(120, 120).is_empty());
        assert!(visual.missile_trail(127, 128).is_empty());
        assert!(!visual.missile_trail(115, 116).is_empty());
    }

    #[test]
    fn undo_reverses_each_turn_and_restores_damage_on_landing() {
        let mut game = game(Some(PowerUp::Diagonal), false);
        let initial = game.clone();
        game.take_move(Position(0, 0), Position(1, 0)).unwrap();
        game.take_move(Position(3, 0), Position(3, 1)).unwrap();
        let mut visual = Presentation::new(&game);
        visual.rewind(&game, 2, 100);
        assert_eq!(
            visual
                .mages(109)
                .iter()
                .find(|(m, _)| m.index == 1)
                .unwrap()
                .1,
            (3.0, 0.5)
        );
        assert_eq!(visual.game().get_mage(1).unwrap().mana.0, 3);
        assert_eq!(visual.advance(118), vec![Signal::Move, Signal::Move]);
        assert_eq!(visual.game().turns(), 1);
        assert!(visual.powerups(118).is_empty());
        assert_eq!(visual.mages(127)[0].1, (0.5, 0.0));
        assert!(visual.missile_trail(100, 135).is_empty());
        assert!(visual.advance(136).is_empty());
        assert!(!visual.busy());
        assert!(same_history(visual.game(), &initial));
        assert_eq!(visual.game().get_mage(1).unwrap().mana.0, 4);
        assert_eq!(
            visual.powerups(136).get(&Position(1, 0)),
            Some(&PowerUp::Diagonal)
        );
    }

    #[test]
    fn undo_during_playback_preserves_the_current_pose_and_then_rewinds() {
        let mut game = game(None, false);
        let mut visual = Presentation::new(&game);
        queue(
            &mut visual,
            &mut game,
            Turn(Position(0, 0), Position(1, 0)),
            100,
        );
        visual.advance(109);
        let position = visual.mages(109)[0].1;
        let lift = visual.pose(0, 109).offset_y;
        visual.rewind(&game, 2, 109); // Only one turn exists.
        assert_eq!(visual.mages(109)[0].1, position);
        assert_eq!(visual.pose(0, 109).offset_y, lift);
        assert_eq!(
            visual.advance(127),
            vec![Signal::Impact(vec![Position(3, 0)]), Signal::Move]
        );
        assert_eq!(visual.mages(136)[0].1, (0.5, 0.0));
        visual.advance(145);
        assert_eq!(visual.game().turns(), 0);
        assert_eq!(visual.game().get_mage(1).unwrap().mana.0, 4);
        assert!(!visual.busy());
    }

    #[test]
    fn undo_can_move_a_mage_that_fell_asleep_in_shield_coverage() {
        let mut game = game(None, true);
        game.get_mage_mut(0).unwrap().mana.0 = 1;
        game.take_move(Position(0, 0), Position(1, 0)).unwrap();
        assert!(!game.get_mage(0).unwrap().is_alive());
        let mut visual = Presentation::new(&game);
        visual.rewind(&game, 1, 100);
        assert_eq!(visual.mages(109)[0].1, (0.5, 0.0));
        assert!(visual.pose(0, 109).offset_y < 0.0);
        visual.advance(118);
        assert!(visual.game().get_mage(0).unwrap().is_alive());
        assert_eq!(visual.game().get_mage(0).unwrap().position, Position(0, 0));
    }
}
