use std::{cell::RefCell, f64::consts::PI, rc::Rc};

use shared::{
    Board, BoardStyle, Difficulty, GameResult, LoadoutMethod, Lobby, LobbyError, LobbyID,
    LobbySettings, LobbySort, Mage, Mages, Message, Position, Team, Turn,
};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{ArenaMenu, Editor, SkirmishMenu, State};
use crate::{
    app::{
        presentation::{Presentation, Signal},
        Alignment, App, AppContext, ButtonElement, ClipId, ConfirmButtonElement, LabelTheme,
        LabelTrim, Particle, ParticleSort, ParticleSystem, Pointer, StateSort, ToggleButtonElement,
        UIElement, UIEvent, BOARD_SCALE,
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

pub struct Game {
    ai_pending: Option<crate::ai::Pending>,
    ai_revision: u32,
    ai_request: u32,
    difficulty: Difficulty,
    button_rematch: ButtonElement,
    button_leave: ConfirmButtonElement,
    button_menu: ToggleButtonElement,
    button_undo: ButtonElement,
    lobby: Lobby,
    presentation: Presentation,
    last_visual_frame: u64,
    last_impact_frame: u64,
    last_move_frame: u64,
    last_hits: Vec<Position>,
    active_mage: Option<usize>,
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
                let message: Message = serde_wasm_bindgen::from_value(value).unwrap();
                message_pool.push(message);
            })
        };

        if let shared::LobbySort::Online(0) = lobby_settings.lobby_sort {
            let _ = create_new_lobby(lobby_settings.clone())
                .unwrap()
                .then(&message_closure);
        }

        let button_menu = ToggleButtonElement::new(
            (-128 - 18 - 8, -9 - 12),
            (20, 20),
            BUTTON_MENU,
            LabelTrim::Round,
            LabelTheme::Bright,
            crate::app::ContentElement::Sprite((112, 32), (16, 16)),
        );

        let button_undo = ButtonElement::new(
            (-128 - 18 - 8, -9 + 12),
            (20, 20),
            BUTTON_UNDO,
            LabelTrim::Round,
            LabelTheme::Action,
            crate::app::ContentElement::Sprite((144, 16), (16, 16)),
        );

        let button_rematch = ButtonElement::new(
            (-44, -24),
            (88, 24),
            BUTTON_REMATCH,
            LabelTrim::Glorious,
            LabelTheme::Action,
            crate::app::ContentElement::Text("Rematch".to_string(), Alignment::Center),
        );

        let button_leave = ConfirmButtonElement::new(
            (-36, 8),
            (72, 16),
            BUTTON_LEAVE,
            LabelTrim::Return,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Leave".to_string(), Alignment::Center),
        );

        let lobby = Lobby::new(lobby_settings, client_timestamp());
        Game {
            ai_pending: None,
            ai_revision: 0,
            ai_request: 0,
            difficulty: Difficulty::from_preference(&App::kv_get("difficulty")),
            button_rematch,
            button_leave,
            button_menu,
            button_undo,
            presentation: Presentation::new(&lobby.game),
            last_visual_frame: 0,
            last_impact_frame: 0,
            lobby,
            last_move_frame: 0,
            last_hits: Vec::new(),
            active_mage: None,
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

    pub fn particle_system(&mut self) -> &mut ParticleSystem {
        &mut self.particle_system
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
        let position = Position(
            ((location.0 - offset.0) / scale.0) as i8,
            ((location.1 - offset.1) / scale.1) as i8,
        );

        let (board_width, board_height) = self.lobby.game.board_size();

        if (location.0 - offset.0) >= 0
            && position.0 < board_width as i8
            && (location.1 - offset.1) >= 0
            && position.1 < board_height as i8
        {
            Some(position)
        } else {
            None
        }
    }

    pub fn live_occupied(&self, position: Position) -> bool {
        self.lobby.game.live_occupied(&position)
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

    pub fn select_mage_at(&mut self, session_id: Option<&String>, selected_tile: &Position) {
        if !self.presentation.busy() && self.lobby.is_active_player(session_id) {
            self.active_mage = if let Some(occupant) = self.lobby.game.live_occupant(selected_tile)
            {
                if occupant.team == self.lobby.game.turn_for() {
                    Some(occupant.index)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }

    pub fn deselect_mage(&mut self) {
        self.active_mage = None;
    }

    pub fn play_mage_selection_sound(&self, app_context: &AppContext) {
        match self.active_mage {
            Some(_) => app_context.audio_system.play_clip(ClipId::MageSelect),
            None => app_context.audio_system.play_clip(ClipId::MageDeselect),
        }
    }

    pub fn with_easy_ai(mut self) -> Self {
        self.difficulty = Difficulty::Easy;
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

    pub fn draw_game(
        &mut self,
        context: &CanvasRenderingContext2d,
        _interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        frame: u64,
        pointer: &Pointer,
    ) -> Result<(), JsValue> {
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

            draw_sprite(context, atlas, 256.0, 0.0, 256.0, 256.0, 0.0, 0.0)?;

            context.translate(board_offset.0, board_offset.1)?;

            // DRAW particles

            self.particle_system()
                .tick_and_draw(context, atlas, frame)?;

            // DRAW powerups
            for (position, powerup) in self.presentation.powerups(frame) {
                context.save();

                context.translate(
                    16.0 + position.0 as f64 * board_scale.0,
                    16.0 + position.1 as f64 * board_scale.1,
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

            let visual_mages = self.presentation.mages(frame);
            {
                let board_offset = self.board_offset();

                // DRAW markers
                context.save();

                for (mage, _) in &visual_mages {
                    if mage.is_alive() && mage.is_defensive() {
                        for (_, position) in self.presentation.game().targets(mage, mage.position) {
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

                if let Some(mage) = self.get_active_mage() {
                    let available_moves = self.presentation.game().available_moves(mage);
                    for (position, dir, _) in &available_moves {
                        let ri = rotation_from_position(*dir);
                        let is_diagonal = ri % 2 == 1;
                        context.save();
                        context.translate(
                            (position.0 as f64 + 0.5) * board_scale.0,
                            (position.1 as f64 + 0.5) * board_scale.1,
                        )?;
                        context.rotate((ri / 2) as f64 * std::f64::consts::PI / 2.0)?;
                        let bop = (frame / 10 % 3) as f64;
                        context.translate(bop - 4.0, if is_diagonal { bop - 4.0 } else { 0.0 })?;
                        draw_sprite(
                            context,
                            atlas,
                            if is_diagonal { 16.0 } else { 0.0 },
                            32.0,
                            16.0,
                            16.0,
                            -8.0,
                            -8.0,
                        )?;
                        context.restore();
                    }

                    if let Some(selected_tile) = self.presentation.game().location_as_position(
                        pointer.location,
                        board_offset,
                        BOARD_SCALE,
                    ) {
                        if available_moves
                            .iter()
                            .any(|(position, _, _)| position == &selected_tile)
                        {
                            for (enemy_occupied, position) in
                                &self.presentation.game().targets(mage, selected_tile)
                            {
                                if *enemy_occupied {
                                    draw_sprite(
                                        context,
                                        atlas,
                                        32.0,
                                        256.0,
                                        32.0,
                                        32.0,
                                        position.0 as f64 * 32.0,
                                        position.1 as f64 * 32.0,
                                    )?;
                                    draw_crosshair(context, atlas, position, (64.0, 32.0), frame)?;
                                } else {
                                    draw_sprite(
                                        context,
                                        atlas,
                                        64.0,
                                        256.0,
                                        32.0,
                                        32.0,
                                        position.0 as f64 * 32.0,
                                        position.1 as f64 * 32.0,
                                    )?;
                                    // draw_crosshair(context, atlas, position, (48.0, 32.0), 0)?;
                                }
                            }
                        }
                    }
                }

                if let Some(selected_tile) = self.presentation.game().location_as_position(
                    pointer.location,
                    board_offset,
                    BOARD_SCALE,
                ) {
                    if let Some(occupant) = self.presentation.game().live_occupant(&selected_tile) {
                        if let Some(selected_tile) = self.presentation.game().location_as_position(
                            pointer.location,
                            board_offset,
                            BOARD_SCALE,
                        ) {
                            for (_, position) in
                                &self.presentation.game().targets(occupant, selected_tile)
                            {
                                draw_sprite(
                                    context,
                                    atlas,
                                    80.0,
                                    32.0,
                                    16.0,
                                    16.0,
                                    position.0 as f64 * board_scale.0 + 8.0,
                                    position.1 as f64 * board_scale.1 + 8.0,
                                )?;
                            }
                        }
                    }
                    draw_crosshair(context, atlas, &selected_tile, (32.0, 32.0), frame)?;
                }

                context.restore();
            }

            {
                let game_started = self.lobby.all_ready() | self.lobby.is_local();

                // DRAW mages
                for (mage, position) in &visual_mages {
                    let pose = self.presentation.pose(mage.index, frame);
                    context.save();

                    context.translate(
                        16.0 + position.0 * board_scale.0,
                        16.0 + position.1 * board_scale.1,
                    )?;

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
                        game_started,
                        self.presentation.game().result(),
                        !self.presentation.busy(),
                        pose.offset_y,
                        pose.flip_x,
                    )?;

                    context.restore();

                    if mage.is_alive() {
                        if mage.has_diagonals() {
                            for _ in 0..(frame / 3 % 2) {
                                let d = js_sys::Math::random() * -std::f64::consts::PI * 0.9;
                                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                                self.particle_system.add(Particle::new(
                                    (
                                        position.0 + d.cos() * 0.4,
                                        position.1 + pose.offset_y / board_scale.1 - 0.15
                                            + d.sin() * 0.4,
                                    ),
                                    (d.cos() * v, d.sin() * v),
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
                                        position.0 + d.cos() * 0.4,
                                        position.1 + pose.offset_y / board_scale.1 - 0.15
                                            + d.sin() * 0.4,
                                    ),
                                    (d.cos() * v, d.sin() * v),
                                    (js_sys::Math::random() * 30.0) as u64,
                                    ParticleSort::Shield,
                                ));
                            }
                        }
                    }

                    if self.is_mage_active(mage) {
                        draw_sprite(
                            context,
                            atlas,
                            72.0,
                            0.0,
                            8.0,
                            5.0,
                            -3.0,
                            -17.0 - (frame / 6 % 6) as f64,
                        )?;
                    }

                    context.restore();
                }

                // DRAW mana bars for all mages
                for (mage, position) in &visual_mages {
                    let pose = self.presentation.pose(mage.index, frame);
                    context.save();

                    context.translate(
                        16.0 + position.0 * board_scale.0,
                        16.0 + position.1 * board_scale.1,
                    )?;
                    context.translate(0.0, pose.offset_y)?;
                    draw_mana(context, atlas, mage)?;

                    context.restore();
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
                    &pointer,
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
            let (_, gap) = self.presentation.game().stalemate();
            for i in 1..9 {
                if gap > i {
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
            && self.lobby.game.turn_for() == Team::Blue
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
                            self.ai_pending = None;
                            self.ai_revision = self.ai_revision.wrapping_add(1);
                            self.presentation
                                .enqueue(before, &self.lobby.game, turn, hits, frame);
                            self.active_mage = None;
                            self.last_move_frame = frame;
                        }
                    }
                }
                Message::Lobby(lobby) => {
                    // Readiness-only snapshots must not cancel playback. Replacements do.
                    if !crate::app::presentation::same_history(&self.lobby.game, &lobby.game) {
                        self.ai_pending = None;
                        self.ai_revision = self.ai_revision.wrapping_add(1);
                        self.presentation = Presentation::new(&lobby.game);
                        self.particle_system = ParticleSystem::default();
                        self.active_mage = None;
                        self.shake_frame = (0, 0);
                        self.last_hits.clear();
                        self.recorded_result = false;
                        self.newly_won = false;
                        self.result_menu_at = None;
                        self.button_menu.set_selected(false);
                    }
                    self.lobby = *lobby.clone();
                    self.board_dirty = true;

                    if let Ok(lobby_id) = self.lobby_id() {
                        if !lobby.all_ready() {
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

        self.draw_game(context, interface_context, atlas, frame, pointer)?;

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
                let campaign_win = self.lobby.settings.lobby_sort == LobbySort::LocalAI
                    && matches!(
                        self.lobby.settings.loadout_method,
                        LoadoutMethod::Arena(..) | LoadoutMethod::ArenaChaos(..)
                    )
                    && !self.presentation.busy()
                    && self.presentation.game().result() == Some(GameResult::Win(Team::Red));
                self.button_leave
                    .set_text(if campaign_win { "Continue" } else { "Leave" });
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
        let board_offset = self.board_offset();
        let frame = app_context.frame;
        let pointer = &app_context.pointer;
        let session_id = &app_context.session_id;

        let message_pool = self.message_pool.clone();

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
                    self.newly_won = team == Team::Red && !already_won;
                    // Completion is permanent, including when replaying the tutorial.
                    if !already_won {
                        App::kv_set(&code, if team == Team::Red { "win" } else { "loss" });
                    }

                    match team {
                        Team::Red => app_context.audio_system.play_clip(ClipId::LevelSuccess),
                        Team::Blue => app_context.audio_system.play_clip(ClipId::LevelFailure),
                    }

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
            self.ai_pending = None;
            self.ai_revision = self.ai_revision.wrapping_add(1);
            self.message_pool.borrow_mut().clear();
            self.presentation.rewind(&self.lobby.game, 2, frame);
            self.lobby.rewind(2);
            self.shake_frame = (0, 0);
            self.active_mage = None;
            self.recorded_result = false;
            self.newly_won = false;
            self.result_menu_at = None;

            self.last_move_frame = frame;
            self.last_hits = Vec::new();

            self.button_menu.set_selected(false);
        }

        if self.is_interface_active() {
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
                        _ => return Some(StateSort::SkirmishMenu(SkirmishMenu::default())),
                    },
                    _ => (),
                }
            }
        } else {
            if pointer.alt_clicked() {
                self.deselect_mage();
            }

            if !self.presentation.busy() && pointer.clicked() {
                if let Some(selected_tile) = self.lobby.game.location_as_position(
                    pointer.location,
                    board_offset,
                    BOARD_SCALE,
                ) {
                    if let Some(active_mage) = self.get_active_mage() {
                        let from = active_mage.position;

                        if self.lobby.game.try_move(from, selected_tile) {
                            if !self.lobby.is_local() && session_id.is_some() {
                                send_message(
                                    self.lobby_id().unwrap(),
                                    session_id.clone().unwrap(),
                                    Message::Turn(Turn(from, selected_tile)),
                                );
                            }

                            let mut message_pool = message_pool.borrow_mut();

                            message_pool
                                .messages
                                .push(Message::Turn(Turn(from, selected_tile)));

                            self.active_mage = None;
                            self.last_move_frame = frame;
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

        None
    }
}
