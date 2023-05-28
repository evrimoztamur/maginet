use std::{cell::RefCell, rc::Rc};

use shared::{
    LoadoutMethod, Lobby, LobbyError, LobbyID, LobbySettings, LobbySort, Mage, Mages, Message,
    Position, Team, Turn,
};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{EditorState, MenuState, State};
use crate::{
    app::{
        Alignment, AppContext, ButtonElement, ConfirmButtonElement, Interface, LabelTheme,
        LabelTrim, Particle, ParticleSort, StateSort, ToggleButtonElement, UIElement, UIEvent,
        BOARD_SCALE,
    },
    draw::{draw_crosshair, draw_mage, draw_particle, draw_sprite, rotation_from_position, draw_board},
    net::{
        create_new_lobby, fetch, request_state, request_turns_since, send_message, send_ready,
        send_rematch, MessagePool,
    },
    tuple_as, window,
};

const BUTTON_REMATCH: usize = 1;
const BUTTON_LEAVE: usize = 2;
const BUTTON_MENU: usize = 10;

pub struct LobbyState {
    interface: Interface,
    button_menu: ToggleButtonElement,
    lobby: Lobby,
    last_move_frame: u64,
    active_mage: Option<usize>,
    particles: Vec<Particle>,
    message_pool: Rc<RefCell<MessagePool>>,
    message_closure: Closure<dyn FnMut(JsValue)>,
    board_dirty: bool,
}

impl LobbyState {
    pub fn new(lobby_settings: LobbySettings) -> LobbyState {
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
            (-128 - 18 - 8, -9),
            (20, 20),
            BUTTON_MENU,
            LabelTrim::Round,
            LabelTheme::Bright,
            crate::app::ContentElement::Sprite((112, 32), (16, 16)),
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
            LabelTrim::Glorious,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Leave".to_string(), Alignment::Center),
        );

        let root_element = Interface::new(vec![Box::new(button_rematch), Box::new(button_leave)]);

        LobbyState {
            interface: root_element,
            button_menu,
            lobby: Lobby::new(lobby_settings),
            last_move_frame: 0,
            active_mage: None,
            particles: Vec::new(),
            message_pool,
            message_closure,
            board_dirty: true
        }
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
        if self.lobby.is_active_player(session_id) {
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

    pub fn take_best_turn(&mut self) {
        let turn = self
            .lobby
            .game
            .best_turn(window().performance().unwrap().now().to_bits());

        if let Some((turn, _)) = turn {
            self.message_pool.borrow_mut().push(Message::Move(turn));
        }
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

    fn draw_game(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let board_scale = tuple_as!(BOARD_SCALE, f64);
        let board_offset = tuple_as!(self.board_offset(), f64);

        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        // DRAW background layer (board + UI block)

        // DRAW board

        {
            context.save();

            draw_sprite(context, atlas, 256.0, 0.0, 256.0, 256.0, 0.0, 0.0)?;

            context.translate(board_offset.0, board_offset.1)?;

            if !self.lobby.all_ready() {
                let mut lid = self.lobby_id().unwrap_or(0);

                while lid != 0 {
                    let tz = lid.trailing_zeros();
                    let x = tz % 4;
                    let y = tz / 4;

                    lid ^= 1 << tz;

                    draw_sprite(
                        context,
                        atlas,
                        96.0,
                        32.0,
                        16.0,
                        16.0,
                        x as f64 * board_scale.0 + 72.0,
                        y as f64 * board_scale.1 + 72.0,
                    )?;
                }
            }

            // DRAW particles

            for particle in self.particles.iter_mut() {
                particle.tick();
                draw_particle(context, atlas, &particle, frame)?;
            }

            self.particles.drain_filter(|particle| !particle.is_alive());

            {
                let game_started = self.lobby.all_ready() | self.lobby.is_local();

                let mut mage_heap: Vec<&Mage> = self.lobby.game.iter_mages().collect();
                mage_heap.sort_by(|a, b| a.position.1.cmp(&b.position.1));

                // DRAW mages
                for mage in mage_heap {
                    context.save();

                    context.translate(
                        16.0 + mage.position.0 as f64 * board_scale.0,
                        16.0 + mage.position.1 as f64 * board_scale.1,
                    )?;

                    draw_mage(
                        context,
                        atlas,
                        mage,
                        frame,
                        self.lobby.game.turn_for(),
                        game_started,
                        self.lobby.game.result(),
                        true,
                    )?;

                    if mage.is_alive() {
                        if mage.has_diagonals() {
                            for _ in 0..(frame / 3 % 2) {
                                let d = js_sys::Math::random() * -std::f64::consts::PI * 0.9;
                                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                                self.particles.push(Particle::new(
                                    (
                                        mage.position.0 as f64 + d.cos() * 0.4,
                                        mage.position.1 as f64 - 0.15 + d.sin() * 0.4,
                                    ),
                                    (d.cos() * v, d.sin() * v),
                                    (js_sys::Math::random() * 30.0) as u64,
                                    ParticleSort::Diagonals,
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
            }

            let board_offset = self.board_offset();

            {
                // DRAW markers
                context.save();

                if let Some(mage) = self.get_active_mage() {
                    let available_moves = self.lobby.game.available_moves(mage);
                    for (position, dir, _) in &available_moves {
                        let ri = rotation_from_position(*dir);
                        let is_diagonal = ri % 2 == 1;
                        context.save();
                        context.translate(
                            (position.0 as f64 + 0.5) * board_scale.0,
                            (position.1 as f64 + 0.5) as f64 * board_scale.1,
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

                    if let Some(selected_tile) = self.lobby.game.location_as_position(
                        pointer.location,
                        board_offset,
                        BOARD_SCALE,
                    ) {
                        if available_moves
                            .iter()
                            .find(|(position, _, _)| position == &selected_tile)
                            .is_some()
                        {
                            for (enemy_occupied, position) in
                                &self.lobby.game.targets(mage, selected_tile)
                            {
                                if *enemy_occupied {
                                    draw_crosshair(context, atlas, position, (64.0, 32.0), frame)?;
                                } else {
                                    draw_crosshair(context, atlas, position, (48.0, 32.0), 0)?;
                                }
                            }
                        }
                    }
                }

                if let Some(selected_tile) = self.lobby.game.location_as_position(
                    pointer.location,
                    board_offset,
                    BOARD_SCALE,
                ) {
                    if let Some(occupant) = self.lobby.game.live_occupant(&selected_tile) {
                        if let Some(selected_tile) = self.lobby.game.location_as_position(
                            pointer.location,
                            board_offset,
                            BOARD_SCALE,
                        ) {
                            for (_, position) in &self.lobby.game.targets(occupant, selected_tile) {
                                draw_crosshair(context, atlas, position, (80.0, 32.0), 0)?;
                            }
                        }
                    }
                    draw_crosshair(context, atlas, &selected_tile, (32.0, 32.0), frame)?;
                }

                context.restore();
            }

            context.restore();
        }

        Ok(())
    }

    pub fn is_interface_active(&self) -> bool {
        self.button_menu.selected()
    }
}

impl State for LobbyState {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        let (board_width, board_height) = self.lobby.game.board_size();

        if self.board_dirty {
            self.board_dirty = false;
            draw_board(atlas, 256.0, 0.0, board_width, board_height, 8, 8).unwrap();
        }

        self.draw_game(context, atlas, app_context)?;

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

            if self.is_interface_active() {
                self.interface
                    .draw(interface_context, atlas, &interface_pointer, frame)?;

                for player in self
                    .lobby
                    .players()
                    .values()
                    .filter(|player| player.rematch)
                {
                    let first_mage = self
                        .lobby
                        .game
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
                            false,
                        )?;

                        interface_context.restore();
                    }
                }
            }

            interface_context.restore();
        }

        Ok(())
    }

    fn tick(&mut self, text_input: &HtmlInputElement, app_context: &AppContext) -> Option<StateSort> {
        let board_offset = self.board_offset();
        let frame = app_context.frame;
        let pointer = &app_context.pointer;
        let session_id = &app_context.session_id;

        let mut target_positions = Vec::new();

        let all_ready = self.lobby.all_ready();

        {
            let mut message_pool = self.message_pool.borrow_mut();

            if let Some(lobby_id) = self.lobby.settings.lobby_sort.lobby_id() {
                if message_pool.available(frame) {
                    if all_ready {
                        if self.is_interface_active() {
                            let _ = fetch(&request_state(lobby_id)).then(&self.message_closure);
                        } else {
                            let _ = fetch(&request_turns_since(lobby_id, self.lobby.game.turns()))
                                .then(&self.message_closure);
                        }
                    } else if self.lobby.settings.lobby_sort != LobbySort::Online(0) {
                        let _ = fetch(&request_state(lobby_id)).then(&self.message_closure);
                    }

                    message_pool.block(frame);
                }
            }

            if self.lobby.has_ai()
                && self.lobby.game.turn_for() == Team::Blue
                && frame - self.last_move_frame > 45
                && !self.lobby.finished()
            {
                let turn = self
                    .lobby
                    .game
                    .best_turn(window().performance().unwrap().now().to_bits());

                if let Some((turn, _)) = turn {
                    message_pool.messages.append(&mut vec![Message::Move(turn)]);
                }
            }

            for message in &message_pool.messages {
                match message {
                    Message::Moves(turns) => {
                        for Turn(from, to) in turns {
                            if let Some(mut move_targets) = self.lobby.game.take_move(*from, *to) {
                                target_positions.append(&mut move_targets);

                                self.last_move_frame = frame;
                            }
                        }
                    }
                    Message::Move(Turn(from, to)) => {
                        if let Some(mut move_targets) = self.lobby.game.take_move(*from, *to) {
                            target_positions.append(&mut move_targets);

                            self.last_move_frame = frame;
                        }
                    }
                    Message::Lobby(lobby) => {
                        self.lobby = lobby.clone();

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
        }

        if self.lobby.finished() && self.frames_since_last_move(frame) == 120 {
            self.button_menu.set_selected(true);
        }

        let interface_pointer =
            pointer.teleport(app_context.canvas_settings.inverse_interface_center());

        self.button_menu.tick(&interface_pointer);

        if self.is_interface_active() {
            if let Some(UIEvent::ButtonClick(value)) = self.interface.tick(&interface_pointer) {
                match value {
                    BUTTON_REMATCH => {
                        if self.lobby.is_local() {
                            return Some(StateSort::Lobby(LobbyState::new(
                                self.lobby.settings.clone(),
                            )));
                        } else if let Ok(lobby_id) = self.lobby_id() {
                            let session_id = app_context.session_id.clone().unwrap();
                            let _ = send_rematch(lobby_id, session_id)
                                .unwrap()
                                .then(&self.message_closure);
                        }
                    }
                    BUTTON_LEAVE => match &self.lobby.settings {
                        LobbySettings {
                            loadout_method: LoadoutMethod::Prefab(mages),
                            board,
                            ..
                        } => {
                            return Some(StateSort::Editor(EditorState::new(
                                board.clone(),
                                mages.clone(),
                            )));
                        }
                        _ => return Some(StateSort::MenuMain(MenuState::new())),
                    },
                    _ => (),
                }
            }
        } else {
            if pointer.alt_clicked() {
                self.active_mage = None;
            }

            if pointer.clicked() {
                if let Some(selected_tile) = self.lobby.game.location_as_position(
                    pointer.location,
                    board_offset,
                    BOARD_SCALE,
                ) {
                    if let Some(active_mage) = self.get_active_mage() {
                        let from = active_mage.position;

                        if let Some(mut move_targets) =
                            self.lobby.game.take_move(from, selected_tile)
                        {
                            if !self.lobby.is_local() && session_id.is_some() {
                                send_message(
                                    self.lobby_id().unwrap(),
                                    session_id.clone().unwrap(),
                                    Message::Move(Turn(from, selected_tile)),
                                );
                            }

                            target_positions.append(&mut move_targets);

                            self.active_mage = None;
                            self.last_move_frame = frame;
                        } else {
                            self.select_mage_at(session_id.as_ref(), &selected_tile);
                        }
                    } else {
                        self.select_mage_at(session_id.as_ref(), &selected_tile);
                    }
                }
            }
        }

        for tile in target_positions {
            for _ in 0..40 {
                let d = js_sys::Math::random() * std::f64::consts::TAU;
                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                self.particles.push(Particle::new(
                    (tile.0 as f64, tile.1 as f64),
                    (d.cos() * v, d.sin() * v),
                    (js_sys::Math::random() * 50.0) as u64,
                    ParticleSort::Missile,
                ));
            }
        }

        None
    }
}
