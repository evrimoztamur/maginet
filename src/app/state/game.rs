mod deadlock_banner;
mod highlights;
mod mobile;
mod selection;
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use mobile::*;
use selection::SelectionMemory;
use shared::{
    Board, BoardStyle, Difficulty, GameResult, LoadoutMethod, Lobby, LobbyError, LobbyID,
    LobbySettings, LobbySort, Mage, Mages, Message, Position, Team, Turn,
};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{ArenaMenu, Editor, SkirmishMenu, State};
use crate::{
    app::{
        board_view::BoardView,
        presentation::{DragLanding, Presentation, Signal},
        Alignment, App, AppContext, ClipId, ConfirmButtonElement, LabelTheme, LabelTrim, Particle,
        ParticleSort, ParticleSystem, Pointer, StateSort, ToggleButtonElement, UIElement, UIEvent,
        BOARD_SCALE,
    },
    draw::{
        draw_board, draw_crosshair, draw_label, draw_mage, draw_mage_with_motion, draw_mana,
        draw_powerup, draw_sprite, draw_text, rotation_from_position, text_length,
    },
    net::{
        client_timestamp, create_new_lobby, fetch, request_state, request_turns_since,
        send_message, send_ready, send_rematch, MessagePool,
    },
    tuple_as,
};

const BUTTON_REMATCH: usize = 1;
const BUTTON_LEAVE: usize = 2;
const BUTTON_MENU: usize = 10;
const BUTTON_UNDO: usize = 20;

struct DragReturn {
    index: usize,
    origin: Position,
    from: DragLanding,
    started: u64,
}
impl DragReturn {
    fn pose(&self, frame: u64) -> DragLanding {
        let remaining = (1.0 - frame.saturating_sub(self.started) as f64 / 12.0)
            .clamp(0.0, 1.0)
            .powi(3);
        DragLanding {
            ground: (
                self.origin.0 as f64 + (self.from.ground.0 - self.origin.0 as f64) * remaining,
                self.origin.1 as f64 + (self.from.ground.1 - self.origin.1 as f64) * remaining,
            ),
            offset_y: self.from.offset_y * remaining,
        }
    }
}

struct MageDrag {
    index: usize,
    origin: Position,
    press: (i32, i32),
    ground: (f64, f64),
    started: Option<u64>,
}

impl MageDrag {
    fn update(&mut self, location: (i32, i32), frame: u64, team: Team) {
        let dx = location.0 - self.press.0;
        let dy = location.1 - self.press.1;
        if dx as i64 * dx as i64 + dy as i64 * dy as i64 > 16 {
            self.started.get_or_insert(frame);
        }
        self.ground = (
            self.origin.0 as f64 + dx as f64 / BOARD_SCALE.0 as f64,
            self.origin.1 as f64
                + dy as f64 / BOARD_SCALE.1 as f64 * if team == Team::Blue { -1.0 } else { 1.0 },
        );
    }

    fn landing(&self, frame: u64) -> DragLanding {
        DragLanding {
            ground: self.ground,
            offset_y: -12.0
                + (frame.saturating_sub(self.started.unwrap_or(frame)) as f64
                    * std::f64::consts::TAU
                    / 60.0)
                    .sin(),
        }
    }
}

pub struct Game {
    view_team: Team,
    create_requested: bool,
    ai_pending: Option<crate::ai::Pending>,
    ai_revision: u32,
    ai_request: u32,
    difficulty: Difficulty,
    tutorial: bool,
    button_rematch: ConfirmButtonElement,
    button_leave: ConfirmButtonElement,
    button_menu: ToggleButtonElement,
    button_undo: ConfirmButtonElement,
    lobby: Lobby,
    presentation: Presentation,
    deadlock_banner: deadlock_banner::DeadlockBanner,
    last_visual_frame: u64,
    last_impact_frame: u64,
    last_move_frame: u64,
    last_hits: Vec<Position>,
    active_mage: Option<usize>,
    selection_memory: SelectionMemory,
    drag: Option<MageDrag>,
    drag_return: Option<DragReturn>,
    destination: Option<Position>,
    mobile_press: Option<(MobileHit, (i32, i32))>,
    mobile_roster: RefCell<Vec<RosterSlot>>,
    local_landing: Option<(Turn, DragLanding)>,
    particle_system: ParticleSystem,
    message_pool: Rc<RefCell<MessagePool>>,
    message_closure: Closure<dyn FnMut(JsValue)>,
    board_dirty: bool,
    shake_frame: (u64, usize),
    recorded_result: bool,
    newly_won: bool,
    result_menu_at: Option<u64>,
}

impl Game {
    pub fn new(mut lobby_settings: LobbySettings) -> Game {
        if matches!(lobby_settings.loadout_method, LoadoutMethod::ArenaChaos(..)) {
            lobby_settings.seed = (js_sys::Math::random() * 9_007_199_254_740_991.0) as u64;
        }
        let message_pool = Rc::new(RefCell::new(MessagePool::new()));

        let message_closure = {
            let message_pool = message_pool.clone();

            Closure::<dyn FnMut(JsValue)>::new(move |value| {
                let mut message_pool = message_pool.borrow_mut();
                if let Ok(message) = serde_wasm_bindgen::from_value::<Message>(value) {
                    message_pool.push(message);
                }
            })
        };

        let button_menu = ToggleButtonElement::new(
            (-168, -40),
            (32, 32),
            BUTTON_MENU,
            LabelTrim::Round,
            LabelTheme::Bright,
            crate::app::ContentElement::Sprite((112, 32), (16, 16)),
        );

        let button_undo = ConfirmButtonElement::new(
            (-168, 0),
            (32, 32),
            BUTTON_UNDO,
            LabelTrim::Round,
            LabelTheme::Action,
            crate::app::ContentElement::Sprite((144, 16), (16, 16)),
        );

        let button_rematch = ConfirmButtonElement::new(
            (-60, -28),
            (120, 32),
            BUTTON_REMATCH,
            LabelTrim::Glorious,
            LabelTheme::Action,
            crate::app::ContentElement::Text("Rematch".to_string(), Alignment::Center),
        );

        let button_leave = ConfirmButtonElement::new(
            (-60, 16),
            (120, 32),
            BUTTON_LEAVE,
            LabelTrim::Return,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Leave".to_string(), Alignment::Center),
        );

        let lobby = Lobby::new(lobby_settings, client_timestamp());
        Game {
            view_team: lobby.settings.player_team,
            create_requested: false,
            ai_pending: None,
            ai_revision: 0,
            ai_request: 0,
            difficulty: Difficulty::from_preference(&App::kv_get("difficulty")),
            tutorial: false,
            button_rematch,
            button_leave,
            button_menu,
            button_undo,
            presentation: Presentation::new(&lobby.game),
            deadlock_banner: deadlock_banner::DeadlockBanner::default(),
            last_visual_frame: 0,
            last_impact_frame: 0,
            lobby,
            last_move_frame: 0,
            last_hits: Vec::new(),
            active_mage: None,
            selection_memory: SelectionMemory::default(),
            drag: None,
            drag_return: None,
            destination: None,
            mobile_press: None,
            mobile_roster: RefCell::new(Vec::new()),
            local_landing: None,
            particle_system: ParticleSystem::default(),
            message_pool,
            message_closure,
            board_dirty: true,
            recorded_result: false,
            newly_won: false,
            result_menu_at: None,
            shake_frame: (0, 0),
        }
    }

    fn pending_game_change(&self, session: Option<&String>) -> bool {
        self.message_pool
            .borrow()
            .messages
            .iter()
            .any(|message| match message {
                Message::Turn(_) => true,
                Message::Turns(turns) => !turns.is_empty(),
                Message::Lobby(lobby) => {
                    !crate::app::presentation::same_history(&self.lobby.game, &lobby.game)
                        || !lobby.is_active_player(session)
                }
                _ => false,
            })
    }

    fn submit_turn(&mut self, turn: Turn, landing: Option<DragLanding>, app: &AppContext) -> bool {
        if self.presentation.busy()
            || self.lobby.finished()
            || self.is_interface_active()
            || self.pending_game_change(app.session_id.as_ref())
            || !self.lobby.is_active_player(app.session_id.as_ref())
            || !self.lobby.game.try_move(turn.0, turn.1)
        {
            return false;
        }
        if !self.lobby.is_local() {
            if let Some(session) = &app.session_id {
                send_message(
                    self.lobby_id().unwrap(),
                    session.clone(),
                    Message::Turn(turn),
                );
            }
        }
        if let Some(mage) = self.lobby.game.live_occupant(&turn.0) {
            self.selection_memory.remember(mage);
        }
        self.drag_return = None;
        self.button_undo.cancel_confirmation();
        self.local_landing = landing.map(|pose| (turn, pose));
        self.message_pool
            .borrow_mut()
            .messages
            .push(Message::Turn(turn));
        self.active_mage = None;
        self.destination = None;
        self.mobile_press = None;
        self.last_move_frame = app.frame;
        true
    }

    fn drop_drag(&mut self, index: usize, turn: Turn, pose: DragLanding, app: &AppContext) {
        self.drag_return = None;
        if !self.submit_turn(turn, Some(pose), app) {
            self.drag_return = Some(DragReturn {
                index,
                origin: turn.0,
                from: pose,
                started: app.frame,
            });
        }
    }

    fn drag_input(&mut self, app: &AppContext) -> bool {
        if app.pointer.is_touch() {
            return self.mobile_input(app);
        }
        use crate::app::pointer::GestureEvent;
        let mut consumed = self.drag.as_ref().is_some_and(|d| d.started.is_some());
        // A queued server update must be applied before another local action is accepted.
        if self.presentation.busy()
            || self.lobby.finished()
            || self.is_interface_active()
            || !self.lobby.is_active_player(app.session_id.as_ref())
            || self.pending_game_change(app.session_id.as_ref())
        {
            self.drag = None;
            self.drag_return = None;
            self.destination = None;
            self.mobile_press = None;
            return consumed;
        }
        for event in &app.pointer.gestures {
            match *event {
                GestureEvent::Cancel => {
                    self.drag = None;
                    self.drag_return = None;
                    consumed = true;
                }
                GestureEvent::Press(press) => {
                    self.drag = None;
                    self.drag_return = None;
                    if let Some(tile) =
                        self.location_as_position(press, self.board_offset(), BOARD_SCALE)
                    {
                        if let Some(mage) = self.lobby.game.live_occupant(&tile) {
                            if mage.team == self.lobby.game.turn_for() {
                                self.drag = Some(MageDrag {
                                    index: mage.index,
                                    origin: tile,
                                    press,
                                    ground: (tile.0 as f64, tile.1 as f64),
                                    started: None,
                                });
                            }
                        }
                    }
                }
                GestureEvent::Move(location) => {
                    if let Some(drag) = &mut self.drag {
                        drag.update(location, app.frame, self.view_team);
                        if drag.started.is_some() {
                            self.active_mage = Some(drag.index);
                            consumed = true;
                        }
                    }
                }
                GestureEvent::Release(location) => {
                    if let Some(mut drag) = self.drag.take() {
                        drag.update(location, app.frame, self.view_team);
                        if drag.started.is_some() {
                            consumed = true;
                            self.active_mage = Some(drag.index);
                            let pose = drag.landing(app.frame);
                            let tile = Position(
                                (pose.ground.0 + 0.5).floor() as i8,
                                (pose.ground.1 + 0.5).floor() as i8,
                            );
                            if self
                                .lobby
                                .game
                                .live_occupant(&drag.origin)
                                .is_some_and(|m| m.index == drag.index)
                            {
                                self.drop_drag(drag.index, Turn(drag.origin, tile), pose, app);
                            }
                        }
                    }
                }
            }
        }
        consumed
    }

    pub fn particle_system(&mut self) -> &mut ParticleSystem {
        &mut self.particle_system
    }

    pub fn newly_won(&self) -> bool {
        self.newly_won
    }

    pub fn is_animating(&self) -> bool {
        self.presentation.busy()
    }

    pub fn visual_game(&self) -> &shared::Game {
        self.presentation.game()
    }

    pub fn lobby(&self) -> &Lobby {
        &self.lobby
    }

    pub fn lobby_id(&self) -> Result<LobbyID, LobbyError> {
        self.lobby
            .settings
            .lobby_sort
            .lobby_id()
            .ok_or(LobbyError("lobby has no ID".to_string()))
    }

    /// Converts a canvas location to a board [`Position`].
    pub fn location_as_position(
        &self,
        location: (i32, i32),
        offset: (i32, i32),
        scale: (i32, i32),
    ) -> Option<Position> {
        self.board_view().location(location, offset, scale)
    }

    fn board_view(&self) -> BoardView {
        let (width, height) = self.lobby.game.board_size();
        BoardView {
            team: self.view_team,
            width: width as i32,
            height: height as i32,
        }
    }

    fn is_mage_active(&self, mage: &Mage) -> bool {
        match self.active_mage {
            Some(active_mage) => active_mage == mage.index,
            None => false,
        }
    }

    fn get_active_mage(&self) -> Option<&Mage> {
        if let Some(index) = self.active_mage {
            if let Some(mage) = self.lobby.game.get_mage(index) {
                return Some(mage);
            }
        }

        None
    }

    fn get_movable_mage(&self) -> Option<&Mage> {
        self.get_active_mage()
            .filter(|mage| mage.is_alive() && mage.team == self.lobby.game.turn_for())
    }

    pub fn select_mage_at(&mut self, session_id: Option<&String>, selected_tile: &Position) {
        self.destination = None;
        if !self.presentation.busy()
            && !self.lobby.finished()
            && self.lobby.is_active_player(session_id)
        {
            self.active_mage = self
                .lobby
                .game
                .live_occupant(selected_tile)
                .map(|mage| mage.index);
        }
    }

    pub fn deselect_mage(&mut self) {
        self.active_mage = None;
        self.destination = None;
        self.mobile_press = None;
    }

    pub fn play_mage_selection_sound(&self, app_context: &AppContext) {
        match self.active_mage {
            Some(_) => app_context.audio_system.play_clip(ClipId::MageSelect),
            None => app_context.audio_system.play_clip(ClipId::MageDeselect),
        }
    }

    pub fn with_tutorial(mut self) -> Self {
        self.difficulty = Difficulty::Easy;
        self.tutorial = true;
        self
    }

    fn request_ai(&mut self, difficulty: Difficulty) {
        if self.presentation.busy()
            || self.ai_pending.is_some()
            || self.lobby.game.result().is_some()
        {
            return;
        }
        self.ai_request = self.ai_request.wrapping_add(1);
        let seed = self.lobby.game.history_seed(self.lobby.settings.seed);
        self.ai_pending = Some(crate::ai::Pending::new(
            &self.lobby.game,
            difficulty,
            seed,
            self.ai_request,
            self.ai_revision,
        ));
    }

    pub fn take_best_turn_quick(&mut self) {
        self.request_ai(Difficulty::Easy);
    }
    #[cfg(not(feature = "deploy"))]
    pub fn take_best_turn(&mut self) {
        self.request_ai(self.difficulty);
    }

    pub fn board_offset(&self) -> (i32, i32) {
        let board_size = self.lobby().game.board_size();

        (
            ((8 - board_size.0) as i32 * BOARD_SCALE.0) / 2,
            ((8 - board_size.1) as i32 * BOARD_SCALE.1) / 2,
        )
    }

    pub fn frames_since_last_move(&self, frame: u64) -> u64 {
        frame.saturating_sub(self.last_move_frame)
    }

    fn mage_pose(&self, index: usize, frame: u64) -> (crate::app::presentation::MagePose, bool) {
        let mut pose = self.presentation.pose(index, frame);
        let drag = self
            .drag
            .as_ref()
            .filter(|d| d.index == index && d.started.is_some());
        if let Some(returning) = self.drag_return.as_ref().filter(|r| r.index == index) {
            pose.offset_y = returning.pose(frame).offset_y;
        }
        if let Some(drag) = drag {
            pose.offset_y = drag.landing(frame).offset_y;
        }
        (pose, drag.is_some())
    }

    // Board and roster use the same sprite, playback pose, hit flash, and selection marker.
    fn draw_mage_visual(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        mage: &Mage,
        frame: u64,
    ) -> Result<(), JsValue> {
        let (pose, dragged) = self.mage_pose(mage.index, frame);
        context.save();
        if frame.saturating_sub(self.last_impact_frame) < 32
            && frame.saturating_sub(self.last_impact_frame) % 16 < 8
            && self.last_hits.contains(&mage.position)
        {
            context.set_global_composite_operation("lighter")?;
        }
        draw_mage_with_motion(
            context,
            atlas,
            mage,
            frame,
            self.presentation.game().turn_for(),
            self.lobby.all_ready() || self.lobby.is_local(),
            self.presentation.game().result(),
            !self.presentation.busy() && !dragged,
            pose.offset_y,
            pose.flip_x,
        )?;
        context.restore();
        if self.is_mage_active(mage) {
            draw_sprite(
                context,
                atlas,
                72.0,
                0.0,
                8.0,
                5.0,
                -3.0,
                pose.offset_y - 17.0 - (frame / 6 % 6) as f64,
            )?;
        }
        Ok(())
    }

    pub fn draw_game(
        &mut self,
        context: &CanvasRenderingContext2d,
        _interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        frame: u64,
        pointer: &Pointer,
        onscreen_controls: bool,
    ) -> Result<(), JsValue> {
        let view = self.board_view();
        let face_to_face = onscreen_controls && self.lobby.settings.lobby_sort == LobbySort::Local;
        let board_scale = tuple_as!(BOARD_SCALE, f64);
        let board_offset = tuple_as!(self.board_offset(), f64);

        // let (board_width, board_height) = self.presentation.game().board_size();

        if self.board_dirty {
            self.board_dirty = false;
            draw_board(atlas, 256.0, 0.0, self.presentation.game().board(), 8, 8).unwrap();
            draw_board(
                atlas,
                384.0,
                256.0,
                &Board::with_style(3, 3, BoardStyle::Teleport).unwrap(),
                3,
                3,
            )
            .unwrap();
        }

        // DRAW background layer (board + UI block)

        // DRAW board

        context.save();

        if frame.saturating_sub(self.shake_frame.0) < 20 {
            let magnitude = self.shake_frame.1.saturating_sub(1) as f64 * 0.65;
            context.translate(
                ((frame as f64 * 0.15 * self.shake_frame.1 as f64).sin() * (magnitude)).round(),
                ((frame as f64 * 0.3 * self.shake_frame.1 as f64).sin() * (magnitude)).round(),
            )?;
        }

        {
            context.save();

            context.save();
            if view.team == Team::Blue {
                context.translate(0.0, 256.0)?;
                context.scale(1.0, -1.0)?;
            }
            draw_sprite(context, atlas, 256.0, 0.0, 256.0, 256.0, 0.0, 0.0)?;
            context.restore();

            context.translate(board_offset.0, board_offset.1)?;

            // DRAW particles

            self.particle_system()
                .tick_and_draw_view(context, atlas, frame, Some(view))?;

            // DRAW powerups
            for (position, powerup) in self.presentation.powerups(frame) {
                context.save();

                context.translate(
                    16.0 + view.tile(*position).0 as f64 * board_scale.0,
                    16.0 + view.tile(*position).1 as f64 * board_scale.1,
                )?;
                draw_powerup(context, atlas, position, powerup, frame)?;

                if let Some(particle_sort) = ParticleSort::for_powerup(powerup) {
                    for _ in 0..1 {
                        let d = js_sys::Math::random() * std::f64::consts::TAU;
                        let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                        self.particle_system.add(Particle::new(
                            (position.0 as f64, position.1 as f64),
                            (d.cos() * v, d.sin() * v),
                            (js_sys::Math::random() * 20.0) as u64,
                            particle_sort,
                        ));
                    }
                }

                context.restore();
            }

            let mut visual_mages = self.presentation.mages(frame);
            visual_mages.sort_by(|(_, a), (_, b)| {
                view.point(*a)
                    .1
                    .total_cmp(&view.point(*b).1)
                    .then(a.0.total_cmp(&b.0))
            });
            if let Some(returning) = &self.drag_return {
                for (mage, position) in &mut visual_mages {
                    if mage.index == returning.index {
                        *position = returning.pose(frame).ground;
                    }
                }
            }
            let dragging = self.drag.as_ref().filter(|d| d.started.is_some());
            if let Some(drag) = dragging {
                for (mage, position) in &mut visual_mages {
                    if mage.index == drag.index {
                        *position = drag.ground;
                    }
                }
                visual_mages.sort_by_key(|(mage, _)| mage.index == drag.index);
            }
            let persistent_location = self
                .destination
                .map(|p| {
                    let p = view.tile(p);
                    (
                        board_offset.0 as i32 + p.0 as i32 * 32 + 16,
                        board_offset.1 as i32 + p.1 as i32 * 32 + 16,
                    )
                })
                .unwrap_or(if pointer.is_touch() {
                    (-1000, -1000)
                } else {
                    pointer.location
                });
            let preview_location = dragging.map_or(persistent_location, |d| {
                let ground = view.point(d.ground);
                (
                    (board_offset.0 + (ground.0 + 0.5) * board_scale.0).floor() as i32,
                    (board_offset.1 + (ground.1 + 0.5) * board_scale.1).floor() as i32,
                )
            });
            {
                let board_offset = self.board_offset();

                // DRAW markers
                context.save();

                for (mage, _) in &visual_mages {
                    if mage.is_alive() && mage.is_defensive() {
                        for (_, position) in self.presentation.game().targets(mage, mage.position) {
                            let position = view.tile(position);
                            match mage.team {
                                Team::Red => {
                                    draw_sprite(
                                        context,
                                        atlas,
                                        32.0,
                                        16.0,
                                        16.0,
                                        16.0,
                                        position.0 as f64 * 32.0 + 8.0,
                                        position.1 as f64 * 32.0 + 8.0,
                                    )?;
                                    // draw_crosshair(context, atlas, &position, (32.0, 16.0), 1)?;
                                }
                                Team::Blue => {
                                    draw_sprite(
                                        context,
                                        atlas,
                                        48.0,
                                        16.0,
                                        16.0,
                                        16.0,
                                        position.0 as f64 * 32.0 + 8.0,
                                        position.1 as f64 * 32.0 + 8.0,
                                    )?;
                                    // draw_crosshair(context, atlas, &position, (48.0, 16.0), 1)?;
                                }
                            }
                        }
                    }
                }

                if let Some(mage) = self.get_movable_mage() {
                    let available_moves = destinations(self.presentation.game(), mage);
                    for (position, dir, targets) in &available_moves {
                        let position = view.tile(*position);
                        let ri = rotation_from_position(view.direction(*dir));
                        let is_diagonal = ri % 2 == 1;
                        context.save();
                        context.translate(
                            (position.0 as f64 + 0.5) * board_scale.0,
                            (position.1 as f64 + 0.5) * board_scale.1,
                        )?;
                        context.rotate((ri / 2) as f64 * std::f64::consts::PI / 2.0)?;
                        let damage = targets.iter().any(|(hit, _)| *hit);
                        let bop = (frame / 10 % 3) as f64 + if damage { 6.0 } else { 0.0 };
                        context.translate(bop - 4.0, if is_diagonal { bop - 4.0 } else { 0.0 })?;
                        movement_arrow(context, atlas, is_diagonal, damage)?;
                        context.restore();
                    }
                }

                if let Some(selected_tile) =
                    self.location_as_position(preview_location, board_offset, BOARD_SCALE)
                {
                    draw_crosshair(
                        context,
                        atlas,
                        &view.tile(selected_tile),
                        (32.0, 32.0),
                        frame,
                    )?;
                }

                context.restore();
            }

            {
                // DRAW mages
                for (mage, position) in &visual_mages {
                    let (pose, _) = self.mage_pose(mage.index, frame);
                    context.save();

                    context.translate(
                        16.0 + view.point(*position).0 * board_scale.0,
                        16.0 + view.point(*position).1 * board_scale.1,
                    )?;

                    // Rotate the whole visual around its tile, including shadow and selection.
                    let facing = if face_to_face && mage.team == Team::Blue {
                        -1.0
                    } else {
                        1.0
                    };
                    context.scale(facing, facing)?;
                    self.draw_mage_visual(context, atlas, mage, frame)?;

                    if mage.is_alive() {
                        if mage.has_diagonals() {
                            for _ in 0..(frame / 3 % 2) {
                                let d = js_sys::Math::random() * -std::f64::consts::PI * 0.9;
                                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                                self.particle_system.add(Particle::new(
                                    (
                                        position.0 + facing * d.cos() * 0.4,
                                        position.1
                                            + (pose.offset_y / board_scale.1 - 0.15
                                                + d.sin() * 0.4)
                                                * facing
                                                * if view.team == Team::Blue { -1.0 } else { 1.0 },
                                    ),
                                    (
                                        facing * d.cos() * v,
                                        d.sin()
                                            * v
                                            * facing
                                            * if view.team == Team::Blue { -1.0 } else { 1.0 },
                                    ),
                                    (js_sys::Math::random() * 30.0) as u64,
                                    ParticleSort::Diagonals,
                                ));
                            }
                        } else if mage.is_defensive() {
                            for _ in 0..(frame / 3 % 2) {
                                let d = js_sys::Math::random() * -std::f64::consts::PI * 0.9;
                                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                                self.particle_system.add(Particle::new(
                                    (
                                        position.0 + facing * d.cos() * 0.4,
                                        position.1
                                            + (pose.offset_y / board_scale.1 - 0.15
                                                + d.sin() * 0.4)
                                                * facing
                                                * if view.team == Team::Blue { -1.0 } else { 1.0 },
                                    ),
                                    (
                                        facing * d.cos() * v,
                                        d.sin()
                                            * v
                                            * facing
                                            * if view.team == Team::Blue { -1.0 } else { 1.0 },
                                    ),
                                    (js_sys::Math::random() * 30.0) as u64,
                                    ParticleSort::Shield,
                                ));
                            }
                        }
                    }

                    context.restore();
                }

                // DRAW mana bars for all mages
                for (mage, position) in &visual_mages {
                    let (pose, _) = self.mage_pose(mage.index, frame);
                    context.save();

                    context.translate(
                        16.0 + view.point(*position).0 * board_scale.0,
                        16.0 + view.point(*position).1 * board_scale.1,
                    )?;
                    if face_to_face && mage.team == Team::Blue {
                        context.scale(-1.0, -1.0)?;
                    }
                    context.translate(0.0, pose.offset_y)?;
                    draw_mana(context, atlas, mage)?;

                    context.restore();
                }
            }

            if !self.presentation.busy() && !self.is_interface_active() {
                let hovered =
                    self.location_as_position(preview_location, self.board_offset(), BOARD_SCALE);
                let selected = self.get_active_mage().or_else(|| {
                    hovered.and_then(|tile| self.presentation.game().live_occupant(&tile))
                });
                if let Some(mage) = selected {
                    let destination = self.get_movable_mage().and_then(|movable| {
                        hovered.filter(|tile| {
                            self.presentation
                                .game()
                                .legal_turns()
                                .contains(&Turn(movable.position, *tile))
                        })
                    });
                    for (damage, tile) in
                        highlights::attack_highlights(self.presentation.game(), mage, destination)
                    {
                        let tile = view.tile(tile);
                        draw_sprite(
                            context,
                            atlas,
                            if damage { 32.0 } else { 64.0 },
                            256.0,
                            32.0,
                            32.0,
                            tile.0 as f64 * 32.0,
                            tile.1 as f64 * 32.0,
                        )?;
                        if damage {
                            draw_crosshair(context, atlas, &tile, (64.0, 32.0), frame)?;
                        }
                    }
                }
            }

            context.translate(-board_offset.0, -board_offset.1)?;

            if !self.lobby.all_ready() && !self.lobby.is_local() {
                draw_sprite(context, atlas, 384.0, 256.0, 96.0, 96.0, 80.0, 80.0)?;

                let lobby_id = self.lobby_id().unwrap_or(0);

                draw_label(
                    _interface_context,
                    atlas,
                    (80 + (96 - 72) / 2, 80 + (96 - 16) / 2),
                    (72, 16),
                    "#2a9f55",
                    &crate::app::ContentElement::Text(format!("{lobby_id}"), Alignment::Center),
                    pointer,
                    frame,
                    &LabelTrim::Glorious,
                    false,
                )?;

                // while lid != 0 {
                //     let tz = lid.trailing_zeros();
                //     let x = tz % 4;
                //     let y = tz / 4;

                //     lid ^= 1 << tz;

                //     draw_sprite(
                //         context,
                //         atlas,
                //         96.0,
                //         32.0,
                //         16.0,
                //         16.0,
                //         x as f64 * board_scale.0 + 72.0,
                //         y as f64 * board_scale.1 + 72.0,
                //     )?;
                // }
            }

            context.restore();
        }

        context.restore();

        context.save();

        context.translate(6.0 - self.board_offset().0 as f64 + 128.0, -40.0 + 128.0)?;

        if self.presentation.game().can_stalemate() {
            let quiet_turns = self.presentation.game().quiet_turns();
            for i in 1..=shared::Game::STALEMATE_TURNS {
                if quiet_turns >= i {
                    if i % 2 == 1 {
                        draw_sprite(
                            context,
                            atlas,
                            128.0,
                            8.0,
                            8.0,
                            8.0,
                            128.0,
                            0.0 + (i * 8) as f64,
                        )?;
                    } else {
                        draw_sprite(
                            context,
                            atlas,
                            136.0,
                            0.0,
                            8.0,
                            16.0,
                            128.0,
                            0.0 + (i * 8 - 8) as f64,
                        )?;
                    }
                } else {
                    draw_sprite(
                        context,
                        atlas,
                        128.0,
                        0.0,
                        8.0,
                        8.0,
                        128.0,
                        0.0 + (i * 8) as f64,
                    )?;
                }
            }
        }

        context.restore();

        Ok(())
    }

    pub fn tick_game(&mut self, frame: u64, app_context: &AppContext) {
        if self.last_move_frame == 0 {
            self.last_move_frame = frame;
        }

        let session_id = &app_context.session_id;
        if let Some(team) = self.lobby.player_team(session_id.as_ref()) {
            self.view_team = team;
        }
        if self.lobby.settings.lobby_sort == LobbySort::Online(0) && !self.create_requested {
            if let Some(session_id) = session_id {
                if let Some(promise) =
                    create_new_lobby(self.lobby.settings.clone(), session_id.clone())
                {
                    let _ = promise.then(&self.message_closure);
                    self.create_requested = true;
                }
            }
        }

        let all_ready = self.lobby.all_ready();

        if !self.presentation.busy() {
            if let Some(turn) = self
                .ai_pending
                .as_ref()
                .and_then(|pending| pending.poll(&self.lobby.game, self.ai_revision))
            {
                self.ai_pending = None;
                if let Some(turn) = turn {
                    self.message_pool.borrow_mut().push(Message::Turn(turn));
                }
            }
        }
        if self.lobby.has_ai()
            && self.lobby.game.turn_for() != self.lobby.settings.player_team
            && !self.presentation.busy()
            && frame - self.last_move_frame > 45
            && !self.lobby.finished()
            && self.message_pool.borrow().messages.is_empty()
        {
            self.request_ai(self.difficulty);
        }

        let mut message_pool = self.message_pool.borrow_mut();

        if let Some(lobby_id) = self.lobby.settings.lobby_sort.lobby_id() {
            if message_pool.available(frame) {
                if all_ready {
                    if self.is_interface_active() {
                        let _ = fetch(&request_state(lobby_id)).then(&self.message_closure);
                    } else {
                        // let _ = fetch(&request_turns_since(lobby_id, self.lobby.game.turns()))
                        //     .then(&self.message_closure);

                        request_turns_since(
                            lobby_id,
                            session_id.clone().unwrap(),
                            self.lobby.game.turns(),
                        )
                        .map(|promise| promise.then(&self.message_closure));
                    }
                } else if self.lobby.settings.lobby_sort != LobbySort::Online(0) {
                    let _ = fetch(&request_state(lobby_id)).then(&self.message_closure);
                }

                message_pool.block(frame);
            }
        }

        for message in &message_pool.messages {
            match message {
                Message::Turns(_) | Message::Turn(_) => {
                    let turns = match message {
                        Message::Turns(turns) => turns.clone(),
                        Message::Turn(turn) => vec![*turn],
                        _ => unreachable!(),
                    };
                    for turn in turns {
                        let before = self.lobby.game.clone();
                        if let Some(hits) = self.lobby.game.take_move(turn.0, turn.1) {
                            self.button_undo.cancel_confirmation();
                            if let Some(mage) = self
                                .active_mage
                                .and_then(|index| before.get_mage(index))
                                .filter(|mage| mage.team == before.turn_for())
                            {
                                self.selection_memory.remember(mage);
                            }
                            self.ai_pending = None;
                            self.ai_revision = self.ai_revision.wrapping_add(1);
                            self.drag = None;
                            self.drag_return = None;
                            let landing = self.local_landing.take().and_then(|(expected, pose)| {
                                (expected.0 == turn.0 && expected.1 == turn.1).then_some(pose)
                            });
                            self.presentation.enqueue_landing(
                                before,
                                &self.lobby.game,
                                turn,
                                hits,
                                frame,
                                landing,
                            );
                            self.active_mage = None;
                            self.destination = None;
                            self.mobile_press = None;
                            self.last_move_frame = frame;
                        }
                    }
                }
                Message::Lobby(lobby) => {
                    // Readiness-only snapshots must not cancel playback. Replacements do.
                    if !crate::app::presentation::same_history(&self.lobby.game, &lobby.game) {
                        self.ai_pending = None;
                        self.ai_revision = self.ai_revision.wrapping_add(1);
                        self.drag = None;
                        self.drag_return = None;
                        self.local_landing = None;
                        self.selection_memory = SelectionMemory::default();
                        self.presentation = Presentation::new(&lobby.game);
                        self.deadlock_banner = deadlock_banner::DeadlockBanner::default();
                        self.particle_system = ParticleSystem::default();
                        self.active_mage = None;
                        self.destination = None;
                        self.mobile_press = None;
                        self.shake_frame = (0, 0);
                        self.last_hits.clear();
                        self.recorded_result = false;
                        self.newly_won = false;
                        self.result_menu_at = None;
                        self.button_menu.set_selected(false);
                    }
                    if let Some(team) = lobby.player_team(session_id.as_ref()) {
                        self.view_team = team;
                    }
                    self.lobby = *lobby.clone();
                    self.board_dirty = true;

                    if let Ok(lobby_id) = self.lobby_id() {
                        if !lobby.all_ready() && !lobby.has_session_id(session_id.as_ref()) {
                            send_ready(lobby_id, session_id.clone().unwrap());
                        }
                    }
                }
                _ => (),
            }
        }

        message_pool.clear();

        // Capture swept trails before consuming transitions, including their impact frame.
        for position in self
            .presentation
            .missile_trail(self.last_visual_frame, frame)
        {
            let angle = js_sys::Math::random() * std::f64::consts::TAU;
            self.particle_system.add(Particle::new(
                (
                    position.0 + (js_sys::Math::random() - 0.5) * 0.04,
                    position.1 + (js_sys::Math::random() - 0.5) * 0.04,
                ),
                (angle.cos() * 0.012, angle.sin() * 0.012),
                4 + (js_sys::Math::random() * 6.0) as u64,
                ParticleSort::MissileTrail((js_sys::Math::random() * 4.0) as u8),
            ));
        }
        let mut target_positions = Vec::new();
        for signal in self.presentation.advance(frame) {
            match signal {
                Signal::Move => app_context.audio_system.play_clip(ClipId::MageMove),
                Signal::Pickup(powerup) => app_context.audio_system.play_powerup(powerup),
                Signal::Impact(hits) => target_positions.extend(hits),
            }
        }
        self.last_visual_frame = frame;
        self.deadlock_banner
            .observe(self.presentation.game().overcharge_at(), frame);
        for tile in &target_positions {
            for _ in 0..40 {
                let d = js_sys::Math::random() * std::f64::consts::TAU;
                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                self.particle_system.add(Particle::new(
                    (tile.0 as f64, tile.1 as f64),
                    (d.cos() * v, d.sin() * v),
                    (js_sys::Math::random() * 50.0) as u64,
                    ParticleSort::Missile,
                ));
            }
        }

        if !target_positions.is_empty() {
            self.last_impact_frame = frame;
            self.last_hits = target_positions.clone();
            self.shake_frame = (frame, target_positions.len());

            app_context
                .audio_system
                .play_random_zap(target_positions.len());
        }
    }

    fn configure_result_buttons(&mut self) {
        let campaign_win = self.lobby.settings.lobby_sort == LobbySort::LocalAI
            && (self.tutorial
                || matches!(
                    self.lobby.settings.loadout_method,
                    LoadoutMethod::Arena(..) | LoadoutMethod::ArenaChaos(..)
                ))
            && !self.presentation.busy()
            && self.presentation.game().result() == Some(GameResult::Win(Team::Red));
        self.button_leave
            .set_text(if campaign_win { "Continue" } else { "Leave" });
        self.button_leave.set_confirmation_required(!campaign_win);
    }

    pub fn is_interface_active(&self) -> bool {
        self.button_menu.selected()
    }
}

impl State for Game {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        self.configure_result_buttons();
        self.draw_game(
            context,
            interface_context,
            atlas,
            frame,
            pointer,
            Self::mobile_enabled(app_context),
        )?;
        self.draw_mobile(interface_context, atlas, app_context)?;

        {
            let interface_pointer =
                pointer.teleport(app_context.canvas_settings.inverse_interface_center());

            interface_context.save();
            interface_context.translate(
                (app_context.canvas_settings.interface_width / 2) as f64,
                (app_context.canvas_settings.interface_height / 2) as f64,
            )?;

            self.button_menu
                .draw(interface_context, atlas, &interface_pointer, frame)?;

            if self.lobby.is_local() {
                self.button_undo
                    .draw(interface_context, atlas, &interface_pointer, frame)?;
            }

            if self.is_interface_active() {
                if !self.presentation.busy() {
                    if let Some(result) = self.presentation.game().result() {
                        let (title, color) = match result {
                            GameResult::Win(Team::Red) => ("House Ruby wins!", "#a52f55"),
                            GameResult::Win(Team::Blue) => ("Azur Clan wins!", "#256f9c"),
                            GameResult::Stalemate => ("Stalemate!", "#70783e"),
                        };
                        draw_label(
                            interface_context,
                            atlas,
                            (-88, -64),
                            (176, 24),
                            color,
                            &crate::app::ContentElement::Text(title.into(), Alignment::Center),
                            &interface_pointer,
                            frame,
                            &LabelTrim::Glorious,
                            false,
                        )?;
                    }
                }
                self.button_rematch
                    .draw(interface_context, atlas, &interface_pointer, frame)?;
                self.button_leave
                    .draw(interface_context, atlas, &interface_pointer, frame)?;

                for player in self
                    .lobby
                    .players()
                    .values()
                    .filter(|player| player.rematch)
                {
                    let first_mage = self
                        .presentation
                        .game()
                        .iter_mages()
                        .find(|mage| mage.team == player.team);

                    if let Some(first_mage) = first_mage {
                        interface_context.save();

                        match player.team {
                            Team::Red => {
                                interface_context.translate(-40.0, -8.0)?;
                            }
                            Team::Blue => {
                                interface_context.translate(40.0, -8.0)?;
                            }
                        }

                        draw_mage(
                            interface_context,
                            atlas,
                            first_mage,
                            frame,
                            player.team,
                            true,
                            None,
                        )?;

                        interface_context.restore();
                    }
                }
            }

            let session_id = app_context.session_id.as_ref();

            if !self.presentation.busy() && self.lobby.is_active_player(session_id) {
                interface_context.translate(
                    28.0 - self.board_offset().0 as f64 + 128.0,
                    0.0 - text_length("Your turn") as f64 / 2.0,
                )?;
                interface_context.rotate(PI / 2.0)?;
                draw_text(interface_context, atlas, 0.0, 0.0, "Your turn")?;
            }

            interface_context.restore();
        }

        let width = app_context.canvas_settings.interface_width as f64;
        let travel = app_context.canvas_settings.padding_x() as f64 + width / 2.0 + 96.0;
        if let Some(offset) = self.deadlock_banner.offset(frame, travel) {
            draw_label(
                interface_context,
                atlas,
                (
                    (width / 2.0 - 88.0 + offset).round() as i32,
                    app_context.canvas_settings.interface_height as i32 / 2 - 12,
                ),
                (176, 24),
                "#70783e",
                &crate::app::ContentElement::Text("Deadlock!".into(), Alignment::Center),
                pointer,
                frame,
                &LabelTrim::Glorious,
                false,
            )?;
        }

        // draw_text(
        //     interface_context,
        //     atlas,
        //     16.0,
        //     16.0,
        //     &format!("{:?}", self.lobby().game.evaluate()),
        // )?;

        Ok(())
    }

    fn tick(
        &mut self,
        _text_input: &HtmlInputElement,
        app_context: &AppContext,
    ) -> Option<StateSort> {
        if self
            .drag_return
            .as_ref()
            .is_some_and(|r| app_context.frame.saturating_sub(r.started) >= 12)
        {
            self.drag_return = None;
        }
        let consumed_drag = self.drag_input(app_context);
        let filtered_pointer = app_context.pointer.without_clicks();
        let mut released_pointer = app_context.pointer.clone();
        if app_context.pointer.is_touch()
            && app_context
                .pointer
                .gestures
                .iter()
                .any(|e| matches!(e, crate::app::pointer::GestureEvent::Release(_)))
        {
            released_pointer.pending_click = true;
        }
        if consumed_drag && released_pointer.clicked() {
            self.button_undo.cancel_confirmation();
        }
        let board_offset = self.board_offset();
        let frame = app_context.frame;
        let pointer = if consumed_drag {
            &filtered_pointer
        } else {
            &released_pointer
        };
        let session_id = &app_context.session_id;

        if self.result_menu_at.is_some_and(|at| frame >= at) {
            self.button_menu.set_selected(true);
            self.result_menu_at = None;
        }

        if !self.presentation.busy() && self.lobby.finished() {
            if !self.recorded_result {
                self.result_menu_at = Some(frame + 120);
            }
            if let Some(GameResult::Win(team)) = self.lobby.game.result() {
                // Did not record the result in the KV-store yet...
                if !self.recorded_result {
                    let code = self
                        .lobby
                        .settings
                        .loadout_method
                        .campaign_progress_code()
                        .unwrap_or_else(|| self.lobby.game.prototype_code());
                    let already_won = App::kv_get(&code) == "win";
                    self.newly_won = team == self.view_team && !already_won;
                    // Completion is permanent, including when replaying the tutorial.
                    if !already_won {
                        App::kv_set(
                            &code,
                            if team == self.view_team {
                                "win"
                            } else {
                                "loss"
                            },
                        );
                    }

                    app_context
                        .audio_system
                        .play_clip(if team == self.view_team {
                            ClipId::LevelSuccess
                        } else {
                            ClipId::LevelFailure
                        });

                    self.recorded_result = true;
                }

                let board_size = self.lobby().game.board_size();

                let particle_sort = match team {
                    Team::Red => ParticleSort::RedWin,
                    Team::Blue => ParticleSort::BlueWin,
                };

                for _ in 0..(board_size.0 + board_size.1) / 5 {
                    let d = js_sys::Math::random() * std::f64::consts::TAU;
                    let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;

                    self.particle_system.add(Particle::new(
                        (js_sys::Math::random() * board_size.0 as f64 - 0.5, -0.5),
                        (d.sin() * v * 0.5, -v),
                        (js_sys::Math::random() * 40.0) as u64,
                        particle_sort,
                    ));

                    self.particle_system().add(Particle::new(
                        (
                            js_sys::Math::random() * board_size.0 as f64 - 0.5,
                            board_size.1 as f64 - 0.5,
                        ),
                        (d.sin() * v * 0.5, v),
                        (js_sys::Math::random() * 40.0) as u64,
                        particle_sort,
                    ));

                    self.particle_system().add(Particle::new(
                        (-0.5, js_sys::Math::random() * board_size.1 as f64 - 0.5),
                        (-v, d.sin() * v * 0.5),
                        (js_sys::Math::random() * 40.0) as u64,
                        particle_sort,
                    ));

                    self.particle_system().add(Particle::new(
                        (
                            board_size.0 as f64 - 0.5,
                            js_sys::Math::random() * board_size.1 as f64 - 0.5,
                        ),
                        (v, d.sin() * v * 0.5),
                        (js_sys::Math::random() * 40.0) as u64,
                        particle_sort,
                    ));
                }
            }
        }

        if !self.presentation.busy() && self.lobby.finished() {
            self.recorded_result = true;
        }

        let interface_pointer =
            pointer.teleport(app_context.canvas_settings.inverse_interface_center());

        self.button_menu.tick(&interface_pointer);

        if self.lobby.is_local() && self.button_undo.tick(&interface_pointer).is_some() {
            self.drag = None;
            self.drag_return = None;
            self.local_landing = None;
            self.ai_pending = None;
            self.ai_revision = self.ai_revision.wrapping_add(1);
            self.message_pool.borrow_mut().clear();
            self.presentation.rewind(&self.lobby.game, 2, frame);
            self.lobby.rewind(2);
            self.selection_memory.suppress_turn(self.lobby.game.turns());
            self.shake_frame = (0, 0);
            self.active_mage = None;
            self.destination = None;
            self.mobile_press = None;
            self.recorded_result = false;
            self.newly_won = false;
            self.result_menu_at = None;

            self.last_move_frame = frame;
            self.last_hits = Vec::new();

            self.button_menu.set_selected(false);
        }

        self.configure_result_buttons();
        if self.is_interface_active() {
            self.drag = None;
            self.drag_return = None;
            self.destination = None;
            self.mobile_press = None;
            let rematch_event = self.button_rematch.tick(&interface_pointer);
            if let Some(UIEvent::ButtonClick(value, clip_id)) =
                self.button_leave.tick(&interface_pointer).or(rematch_event)
            {
                app_context.audio_system.play_clip_option(clip_id);

                match value {
                    BUTTON_REMATCH => {
                        if self.lobby.is_local() {
                            return Some(StateSort::Game(Game::new(self.lobby.settings.clone())));
                        } else if let Ok(lobby_id) = self.lobby_id() {
                            let session_id = app_context.session_id.clone().unwrap();
                            let _ = send_rematch(lobby_id, session_id)
                                .unwrap()
                                .then(&self.message_closure);
                        }
                    }
                    BUTTON_LEAVE => match &self.lobby.settings {
                        LobbySettings {
                            loadout_method: LoadoutMethod::EditorPrefab(level),
                            ..
                        } => {
                            return Some(StateSort::Editor(Editor::new(level.clone())));
                        }
                        LobbySettings {
                            loadout_method:
                                LoadoutMethod::Arena(_, position)
                                | LoadoutMethod::ArenaChaos(_, position),
                            ..
                        } => {
                            return Some(StateSort::ArenaMenu(ArenaMenu::at_position(
                                *position,
                                self.newly_won && !self.presentation.busy(),
                            )));
                        }
                        settings if self.lobby.is_local() => {
                            return Some(StateSort::SkirmishMenu(SkirmishMenu::new(
                                settings.clone(),
                            )));
                        }
                        _ => return Some(StateSort::SkirmishMenu(SkirmishMenu::default())),
                    },
                    _ => (),
                }
            }
        } else {
            self.button_rematch.cancel_confirmation();
            self.button_leave.cancel_confirmation();
            if pointer.alt_clicked() {
                self.deselect_mage();
            }

            if !self.presentation.busy() && pointer.clicked() {
                if let Some(selected_tile) =
                    self.location_as_position(pointer.location, board_offset, BOARD_SCALE)
                {
                    if let Some(active_mage) = self.get_movable_mage() {
                        let from = active_mage.position;

                        if self.lobby.game.try_move(from, selected_tile) {
                            self.submit_turn(Turn(from, selected_tile), None, app_context);
                        } else {
                            self.select_mage_at(session_id.as_ref(), &selected_tile);
                            self.play_mage_selection_sound(app_context);
                        }
                    } else {
                        self.select_mage_at(session_id.as_ref(), &selected_tile);
                        self.play_mage_selection_sound(app_context);
                    }
                }
            }
        }

        self.tick_game(frame, app_context);
        if !self.presentation.busy()
            && !self.lobby.finished()
            && !self.is_interface_active()
            && !self.pending_game_change(session_id.as_ref())
            && self.lobby.is_active_player(session_id.as_ref())
        {
            if let Some(index) = self.selection_memory.restore(&self.lobby.game) {
                if self.active_mage.is_none() {
                    self.active_mage = Some(index);
                    self.destination = None;
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod drag_tests {
    use super::*;

    #[test]
    fn blue_drag_moves_in_world_coordinates() {
        let mut drag = MageDrag {
            index: 0,
            origin: Position(2, 3),
            press: (77, 110),
            ground: (2.0, 3.0),
            started: None,
        };
        drag.update((93, 142), 12, Team::Blue);
        assert_eq!(drag.ground, (2.5, 2.0));
        assert_eq!(drag.landing(12).offset_y, -12.0);
    }

    #[test]
    fn threshold_grab_offset_and_float_are_stable() {
        let mut drag = MageDrag {
            index: 0,
            origin: Position(2, 3),
            press: (77, 110),
            ground: (2.0, 3.0),
            started: None,
        };
        drag.update((81, 110), 10, Team::Red);
        assert_eq!(drag.started, None);
        drag.update((81, 111), 11, Team::Red);
        assert_eq!(drag.started, Some(11));
        drag.update((109, 142), 12, Team::Red);
        assert_eq!(drag.ground, (3.0, 4.0));
        drag.update((77, 110), 13, Team::Red);
        assert_eq!(drag.started, Some(11)); // Crossing back never becomes a tap.
        assert_eq!(drag.ground, (2.0, 3.0));
        assert_eq!(drag.landing(11).offset_y, -12.0);
        assert_eq!(drag.landing(26).offset_y, -11.0);
        assert_eq!(drag.landing(56).offset_y, -13.0);
    }
}
