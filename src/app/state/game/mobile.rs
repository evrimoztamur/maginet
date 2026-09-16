use super::*;
use crate::app::{pointer::GestureEvent, CanvasSettings};

type Destination = (Position, Position, Vec<(bool, Position)>);

pub(super) fn destinations(game: &shared::Game, mage: &Mage) -> Vec<Destination> {
    game.available_moves(mage)
        .into_iter()
        .map(|(at, direction, _)| (at, direction, game.targets(mage, at)))
        .collect()
}

// White copies are initialized in the atlas; damage uses the original cyan pixels.
pub(super) fn movement_arrow(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlCanvasElement,
    diagonal: bool,
    damage: bool,
) -> Result<(), JsValue> {
    draw_sprite(
        context,
        atlas,
        (if damage { 0.0 } else { 160.0 }) + if diagonal { 16.0 } else { 0.0 },
        if damage { 32.0 } else { 144.0 },
        16.0,
        16.0,
        -8.0,
        -8.0,
    )
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum MobileHit {
    Mage(usize),
    Destination(Position),
    Center,
    Board,
    Block,
}

// Slot motion is keyed by mage identity, so selection survives visual reordering.
pub(super) struct RosterSlot {
    index: usize,
    from: (i32, i32),
    target: (i32, i32),
    started: u64,
}
impl RosterSlot {
    fn position(&self, frame: u64) -> (i32, i32) {
        let t = (frame.saturating_sub(self.started) as f64 / 12.0).min(1.0);
        let t = t * t * (3.0 - 2.0 * t);
        (
            (self.from.0 as f64 + (self.target.0 - self.from.0) as f64 * t).round() as i32,
            (self.from.1 as f64 + (self.target.1 - self.from.1) as f64 * t).round() as i32,
        )
    }
    fn retarget(&mut self, target: (i32, i32), frame: u64) {
        if self.target != target {
            self.from = self.position(frame);
            self.target = target;
            self.started = frame;
        }
    }
}

fn roster_key(view: BoardView, orientation: bool, flipped: bool, mage: &Mage) -> (i8, i8, usize) {
    let p = view.tile(mage.position);
    let (x, y) = if orientation { (-p.1, p.0) } else { (p.0, p.1) };
    if flipped {
        (-x, -y, mage.index)
    } else {
        (x, y, mage.index)
    }
}

#[derive(Clone, Copy)]
struct MobileLayout {
    left: i32,
    bottom: i32,
    right: i32,
    columns: usize,
}
impl MobileLayout {
    fn new(s: &CanvasSettings, flipped: bool) -> Self {
        let inset = |name: &str| {
            if cfg!(feature = "mobile") {
                js_sys::Reflect::get(&crate::window(), &name.into())
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0)
                    * 272.0
                    / crate::window().inner_height().unwrap().as_f64().unwrap()
            } else {
                0.0
            }
        };
        let left = 6 + inset(if flipped {
            "maginetSafeRight"
        } else {
            "maginetSafeLeft"
        })
        .ceil() as i32;
        let right = s.element_width() as i32
            - 6
            - inset(if flipped {
                "maginetSafeLeft"
            } else {
                "maginetSafeRight"
            })
            .ceil() as i32;
        let bottom = s.element_height() as i32
            - 6
            - inset(if flipped {
                "maginetSafeTop"
            } else {
                "maginetSafeBottom"
            })
            .ceil() as i32;
        Self {
            left,
            right,
            bottom,
            columns: ((right - left - 100) / 42).max(1) as usize,
        }
    }
    fn roster(self, i: usize, count: usize) -> (i32, i32) {
        let columns = if count == 4 { 2 } else { count.min(3) }
            .min(self.columns)
            .max(1);
        let rows = count.div_ceil(columns);
        let top_count = count - (rows - 1) * columns;
        let (row, column, row_count) = if i < top_count {
            (0, i, top_count)
        } else {
            (
                1 + (i - top_count) / columns,
                (i - top_count) % columns,
                columns,
            )
        };
        (
            self.left + 20 + column as i32 * 42 + (columns - row_count) as i32 * 21,
            self.bottom - 22 - (rows - 1 - row) as i32 * 46,
        )
    }
    fn center(self) -> (i32, i32) {
        (self.right - 45, self.bottom - 45)
    }
    fn button(self, dir: Position) -> (i32, i32) {
        let c = self.center();
        (c.0 + dir.0 as i32 * 30, c.1 + dir.1 as i32 * 30)
    }
}
fn physical(s: &CanvasSettings, p: (i32, i32)) -> (i32, i32) {
    let p = (p.0 + s.padding().0, p.1 + s.padding().1);
    if s.orientation {
        (s.element_width() as i32 - 1 - p.1, p.0)
    } else {
        p
    }
}
fn logical(s: &CanvasSettings, p: (i32, i32)) -> (i32, i32) {
    crate::app::Pointer::location_from_real(s, p)
}
fn roster_inside(p: (i32, i32), c: (i32, i32)) -> bool {
    p.0 >= c.0 - 20 && p.0 < c.0 + 20 && p.1 >= c.1 - 22 && p.1 < c.1 + 22
}
fn inside(p: (i32, i32), c: (i32, i32)) -> bool {
    p.0 >= c.0 - 14 && p.0 < c.0 + 14 && p.1 >= c.1 - 14 && p.1 < c.1 + 14
}

// An involution: the same transform maps physical input into player coordinates
// and player coordinates back to physical drawing positions.
fn player_point(s: &CanvasSettings, flipped: bool, p: (i32, i32)) -> (i32, i32) {
    if flipped {
        (
            s.element_width() as i32 - 1 - p.0,
            s.element_height() as i32 - 1 - p.1,
        )
    } else {
        p
    }
}

impl Game {
    fn mobile_flipped(&self) -> bool {
        self.lobby.settings.lobby_sort == LobbySort::Local
            && self.lobby.game.turn_for() != self.view_team
    }
    pub(crate) fn mobile_enabled(app: &AppContext) -> bool {
        crate::app::SettingsMenu::onscreen_controls_enabled() && Self::touch_enabled(app)
    }
    pub(crate) fn touch_enabled(app: &AppContext) -> bool {
        cfg!(feature = "mobile")
            || app.pointer.is_touch()
            || js_sys::Reflect::get(&crate::window(), &"navigator".into())
                .ok()
                .and_then(|n| js_sys::Reflect::get(&n, &"maxTouchPoints".into()).ok())
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0)
                > 0.0
    }
    fn interaction_locked(&self, app: &AppContext) -> bool {
        self.presentation.busy()
            || self.lobby.finished()
            || self.is_interface_active()
            || self.pending_game_change(app.session_id.as_ref())
            || !self.lobby.is_active_player(app.session_id.as_ref())
    }
    fn roster(&self, app: &AppContext) -> Vec<(&Mage, (i32, i32))> {
        let team = if self.lobby.settings.lobby_sort == LobbySort::Local {
            self.lobby.game.turn_for()
        } else {
            self.view_team
        };
        let mut roster: Vec<_> = self
            .lobby
            .game
            .iter_mages()
            .filter(|m| m.team == team)
            .collect();
        roster.sort_by_key(|m| {
            roster_key(
                self.board_view(),
                app.canvas_settings.orientation,
                self.mobile_flipped(),
                m,
            )
        });
        let layout = MobileLayout::new(&app.canvas_settings, self.mobile_flipped());
        let mut slots = self.mobile_roster.borrow_mut();
        slots.retain(|slot| roster.iter().any(|m| m.index == slot.index));
        let count = roster.len();
        roster
            .into_iter()
            .enumerate()
            .map(|(i, mage)| {
                let target = layout.roster(i, count);
                let slot = if let Some(i) = slots.iter().position(|slot| slot.index == mage.index) {
                    i
                } else {
                    slots.push(RosterSlot {
                        index: mage.index,
                        from: target,
                        target,
                        started: app.frame,
                    });
                    slots.len() - 1
                };
                slots[slot].retarget(target, app.frame);
                (mage, slots[slot].position(app.frame))
            })
            .collect()
    }
    fn pad_direction(&self, dir: Position, s: &CanvasSettings) -> Position {
        let dir = self.board_view().direction(dir);
        let dir = if s.orientation {
            Position(-dir.1, dir.0)
        } else {
            dir
        };
        if self.mobile_flipped() {
            Position(-dir.0, -dir.1)
        } else {
            dir
        }
    }
    fn mobile_hit(&self, location: (i32, i32), app: &AppContext) -> MobileHit {
        if !Self::mobile_enabled(app) {
            return MobileHit::Board;
        }
        let s = &app.canvas_settings;
        let layout = MobileLayout::new(s, self.mobile_flipped());
        let p = player_point(s, self.mobile_flipped(), physical(s, location));
        for (mage, center) in self.roster(app) {
            if roster_inside(p, center) {
                return if mage.is_alive() {
                    MobileHit::Mage(mage.index)
                } else {
                    MobileHit::Block
                };
            }
        }
        if inside(p, layout.center()) {
            return MobileHit::Center;
        }
        if let Some(mage) = self.get_active_mage() {
            for (at, dir, _) in destinations(&self.lobby.game, mage) {
                if inside(p, layout.button(self.pad_direction(dir, s))) {
                    return MobileHit::Destination(at);
                }
            }
        }
        let c = layout.center();
        if (p.0 - c.0).abs() <= 45 && (p.1 - c.1).abs() <= 45 {
            MobileHit::Block
        } else {
            MobileHit::Board
        }
    }
    fn destination_tap(&mut self, tile: Position, app: &AppContext) {
        if let Some(mage) = self.get_active_mage() {
            if destinations(&self.lobby.game, mage)
                .iter()
                .any(|(at, _, _)| *at == tile)
            {
                let from = mage.position;
                if self.destination == Some(tile) {
                    self.submit_turn(Turn(from, tile), None, app);
                } else {
                    self.destination = Some(tile);
                }
                return;
            }
        }
        self.select_mage_at(app.session_id.as_ref(), &tile);
        self.play_mage_selection_sound(app);
    }
    pub(super) fn mobile_input(&mut self, app: &AppContext) -> bool {
        if self.interaction_locked(app) {
            self.drag = None;
            self.drag_return = None;
            self.mobile_press = None;
            self.destination = None;
            return !self.is_interface_active()
                && self
                    .location_as_position(app.pointer.location, self.board_offset(), BOARD_SCALE)
                    .is_some();
        }
        // Mobile taps are resolved at release. No legacy press-click can leak to the board.
        let mut consumed = true;
        for event in &app.pointer.gestures {
            match *event {
                GestureEvent::Cancel => {
                    self.drag = None;
                    self.drag_return = None;
                    self.destination = None;
                    self.mobile_press = None;
                }
                GestureEvent::Press(p) => {
                    let hit = self.mobile_hit(p, app);
                    self.mobile_press = Some((hit, p));
                    self.drag = None;
                    self.drag_return = None;
                    let mage = match hit {
                        MobileHit::Center => self.get_active_mage(),
                        MobileHit::Board => self
                            .location_as_position(p, self.board_offset(), BOARD_SCALE)
                            .and_then(|tile| self.lobby.game.live_occupant(&tile))
                            .filter(|m| m.team == self.lobby.game.turn_for()),
                        _ => None,
                    };
                    if let Some(m) = mage {
                        self.drag = Some(MageDrag {
                            index: m.index,
                            origin: m.position,
                            press: p,
                            ground: (m.position.0 as f64, m.position.1 as f64),
                            started: None,
                        });
                    }
                }
                GestureEvent::Move(p) => {
                    if let Some(d) = &mut self.drag {
                        d.update(p, app.frame, self.view_team);
                        if d.started.is_some() {
                            self.active_mage = Some(d.index);
                            self.destination = None;
                        }
                    }
                }
                GestureEvent::Release(p) => {
                    let press = self.mobile_press.take();
                    if let Some(mut d) = self.drag.take() {
                        d.update(p, app.frame, self.view_team);
                        if d.started.is_some() {
                            self.destination = None;
                            self.active_mage = Some(d.index);
                            let pose = d.landing(app.frame);
                            let tile = Position(
                                (pose.ground.0 + 0.5).floor() as i8,
                                (pose.ground.1 + 0.5).floor() as i8,
                            );
                            self.drop_drag(d.index, Turn(d.origin, tile), pose, app);
                            continue;
                        }
                    }
                    if let Some((hit, start)) = press {
                        if (p.0 - start.0).abs().max((p.1 - start.1).abs()) > 4
                            || self.mobile_hit(p, app) != hit
                        {
                            continue;
                        }
                        match hit {
                            MobileHit::Mage(index) => {
                                if self.active_mage != Some(index) {
                                    self.destination = None;
                                }
                                self.active_mage = Some(index);
                                self.play_mage_selection_sound(app);
                            }
                            MobileHit::Destination(tile) => self.destination_tap(tile, app),
                            MobileHit::Board => {
                                if let Some(tile) =
                                    self.location_as_position(p, self.board_offset(), BOARD_SCALE)
                                {
                                    self.destination_tap(tile, app);
                                } else {
                                    self.destination = None;
                                    consumed = false;
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
        consumed
    }
    pub(super) fn draw_mobile(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app: &AppContext,
    ) -> Result<(), JsValue> {
        if !Self::mobile_enabled(app) || self.is_interface_active() {
            return Ok(());
        }
        let s = &app.canvas_settings;
        let layout = MobileLayout::new(s, self.mobile_flipped());
        let place = |p| -> Result<(), JsValue> {
            let p = logical(s, player_point(s, self.mobile_flipped(), p));
            context.translate(p.0 as f64, p.1 as f64)?;
            if s.orientation {
                context.rotate(-std::f64::consts::FRAC_PI_2)?;
            }
            if self.mobile_flipped() {
                context.rotate(std::f64::consts::PI)?;
            }
            Ok(())
        };
        let visual_mages = self.presentation.mages(app.frame);
        for (mage, center) in self.roster(app) {
            let mage = visual_mages
                .iter()
                .find(|(m, _)| m.index == mage.index)
                .map_or(mage, |(m, _)| m);
            context.save();
            place(center)?;
            context.set_global_alpha(if mage.is_alive() { 1.0 } else { 0.6 });
            draw_label(
                context,
                atlas,
                (-20, -22),
                (40, 44),
                if self.active_mage == Some(mage.index) {
                    "#001f1f"
                } else {
                    "#001515"
                },
                &crate::app::ContentElement::None,
                &app.pointer,
                app.frame,
                &LabelTrim::Round,
                false,
            )?;
            context.translate(0.0, 8.0)?;
            self.draw_mage_visual(context, atlas, mage, app.frame)?;
            context.translate(0.0, self.mage_pose(mage.index, app.frame).0.offset_y)?;
            draw_mana(context, atlas, mage)?;
            context.restore();
        }
        context.save();
        place(layout.center())?;
        draw_label(
            context,
            atlas,
            (-14, -14),
            (28, 28),
            "#001515",
            &crate::app::ContentElement::Sprite((128, 8), (8, 8)),
            &app.pointer,
            app.frame,
            &LabelTrim::Round,
            false,
        )?;
        context.restore();
        if let Some(mage) = self.get_active_mage() {
            for (at, dir, targets) in destinations(&self.lobby.game, mage) {
                context.save();
                place(layout.button(self.pad_direction(dir, s)))?;
                context.set_global_alpha(if self.interaction_locked(app) {
                    0.6
                } else {
                    1.0
                });
                let damage = targets.iter().any(|(hit, _)| *hit);
                mobile_button(
                    context,
                    atlas,
                    app,
                    self.destination == Some(at),
                    damage,
                    crate::app::ContentElement::None,
                )?;
                let ri = rotation_from_position(self.pad_direction(dir, s));
                context.rotate((ri / 2) as f64 * std::f64::consts::FRAC_PI_2)?;
                let bop = (app.frame / 10 % 3) as f64 - 2.0;
                context.translate(bop, if ri % 2 == 1 { bop } else { 0.0 })?;
                movement_arrow(context, atlas, ri % 2 == 1, damage)?;
                context.restore();
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compact_rosters_use_two_rows_and_center_the_short_row() {
        let layout = MobileLayout {
            left: 0,
            right: 400,
            bottom: 200,
            columns: 7,
        };
        for (count, expected) in [
            (4, vec![(20, 132), (62, 132), (20, 178), (62, 178)]),
            (
                5,
                vec![(41, 132), (83, 132), (20, 178), (62, 178), (104, 178)],
            ),
            (
                6,
                vec![
                    (20, 132),
                    (62, 132),
                    (104, 132),
                    (20, 178),
                    (62, 178),
                    (104, 178),
                ],
            ),
        ] {
            assert_eq!(
                (0..count)
                    .map(|i| layout.roster(i, count))
                    .collect::<Vec<_>>(),
                expected
            );
        }
    }

    #[test]
    fn roster_scans_visible_columns_and_animates_slot_changes() {
        use shared::MageSort;
        let mages = [
            Mage::new(0, Team::Red, MageSort::Plus, Position(2, 1)),
            Mage::new(1, Team::Red, MageSort::Plus, Position(1, 2)),
            Mage::new(2, Team::Red, MageSort::Plus, Position(1, 0)),
        ];
        for (team, portrait, expected) in [
            (Team::Red, false, [2, 1, 0]),
            (Team::Blue, false, [1, 2, 0]),
            (Team::Red, true, [1, 0, 2]),
            (Team::Blue, true, [2, 0, 1]),
        ] {
            let view = BoardView {
                team,
                width: 8,
                height: 8,
            };
            let mut ordered: Vec<_> = mages.iter().collect();
            ordered.sort_by_key(|m| roster_key(view, portrait, false, m));
            assert_eq!(
                ordered.iter().map(|m| m.index).collect::<Vec<_>>(),
                expected
            );
            ordered.sort_by_key(|m| roster_key(view, portrait, true, m));
            assert_eq!(
                ordered.iter().map(|m| m.index).collect::<Vec<_>>(),
                expected.into_iter().rev().collect::<Vec<_>>()
            );
        }
        let mut slot = RosterSlot {
            index: 1,
            from: (20, 200),
            target: (20, 200),
            started: 0,
        };
        slot.retarget((62, 200), 10);
        assert_eq!(slot.position(10), (20, 200));
        assert_eq!(slot.position(16), (41, 200));
        slot.retarget((104, 154), 16);
        assert_eq!(slot.position(16), (41, 200)); // Retarget from the current visual location.
        assert_eq!(slot.position(28), (104, 154));
        assert_eq!(slot.index, 1);
    }

    #[test]
    fn physical_controls_round_trip_in_both_orientations() {
        for orientation in [false, true] {
            let s = CanvasSettings::new(400, 272, 256, 256, orientation);
            for flipped in [false, true] {
                let layout = MobileLayout::new(&s, flipped);
                for p in [
                    layout.roster(0, 4),
                    layout.center(),
                    layout.button(Position(-1, 1)),
                ] {
                    let screen = player_point(&s, flipped, p);
                    assert_eq!(player_point(&s, flipped, screen), p);
                    assert_eq!(physical(&s, logical(&s, screen)), screen);
                    if flipped {
                        assert!(screen.1 < s.element_height() as i32 / 2);
                    }
                }
            }
            let layout = MobileLayout::new(&s, false);
            for i in 0..32 {
                let p = layout.roster(i, 32);
                assert_eq!(physical(&s, logical(&s, p)), p);
                assert!(roster_inside(p, layout.roster(i, 32)));
                assert!(p.0 < layout.center().0 - 45);
            }
            for dir in [Position(1, 0), Position(-1, -1), Position(0, 1)] {
                let p = layout.button(dir);
                assert_eq!(physical(&s, logical(&s, p)), p);
                assert!(p.0 + 14 < s.element_width() as i32);
                assert!(p.1 + 14 < s.element_height() as i32);
            }
        }
    }
    #[test]
    fn damage_metadata_matches_preview_for_hits_beams_and_retaliation() {
        use shared::{Level, MageSort, PowerUp};
        let red = Mage::new(0, Team::Red, MageSort::Plus, Position(2, 2));
        let blue = Mage::new(1, Team::Blue, MageSort::Plus, Position(4, 2));
        for mode in 0..4 {
            let mut level = Level::default();
            level.mages = vec![red.clone(), blue.clone()];
            if mode == 1 {
                level.mages[1].position = Position(4, 3);
                level.powerups.insert(Position(2, 3), PowerUp::Beam);
            }
            if mode == 2 {
                level.mages[1].powerup = Some(PowerUp::Shield);
            }
            if mode == 3 {
                level.mages[0].powerup = Some(PowerUp::Diagonal);
            }
            let game = shared::Game::new(&level, false).unwrap();
            let mage = game.get_mage(0).unwrap();
            let moves = destinations(&game, mage);
            assert_eq!(moves.len(), if mode == 3 { 8 } else { 4 });
            assert!(moves
                .iter()
                .any(|(_, _, targets)| targets.iter().any(|(hit, _)| *hit)));
            assert!(moves
                .iter()
                .any(|(_, _, targets)| !targets.iter().any(|(hit, _)| *hit)));
            if mode == 1 {
                assert!(moves
                    .iter()
                    .find(|(at, _, _)| *at == Position(2, 3))
                    .unwrap()
                    .2
                    .contains(&(true, Position(4, 3))));
            }
            if mode == 2 {
                assert!(moves
                    .iter()
                    .find(|(at, _, _)| *at == Position(3, 2))
                    .unwrap()
                    .2
                    .contains(&(true, Position(3, 2))));
            }
            for (at, _, targets) in moves {
                assert_eq!(targets, game.targets(mage, at));
            }
        }
    }
}

fn mobile_button(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlCanvasElement,
    app: &AppContext,
    selected: bool,
    damage: bool,
    content: crate::app::ContentElement,
) -> Result<(), JsValue> {
    let mut button = ToggleButtonElement::new(
        (-14, -14),
        (28, 28),
        0,
        LabelTrim::Round,
        if damage {
            LabelTheme::Attack
        } else {
            LabelTheme::Default
        },
        content,
    );
    button.set_selected(selected);
    // Gesture ownership and hit testing are handled in mobile_input.
    let mut pointer = Pointer::default();
    pointer.location = (-1000, -1000);
    button.draw(context, atlas, &pointer, app.frame)
}
