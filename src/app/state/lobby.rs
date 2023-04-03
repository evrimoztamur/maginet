use std::{cell::RefCell, rc::Rc};

use shared::{Lobby, LobbyError, LobbyID, LobbySettings, Mage, Message, Position, Team, Turn};
use wasm_bindgen::{prelude::Closure, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::State;
use crate::{
    app::{
        AppContext, Particle, ParticleSort, BOARD_OFFSET, BOARD_OFFSET_F64, BOARD_SCALE,
        BOARD_SCALE_F64, StateSort,
    },
    draw::{draw_crosshair, draw_mage, draw_particle, draw_sprite, rotation_from_position},
    net::{fetch, request_state, request_turns_since, send_message, send_ready, MessagePool},
    window,
};

pub struct LobbyState {
    lobby: Lobby,
    last_move_frame: u64,
    active_mage: Option<usize>,
    particles: Vec<Particle>,
    message_pool: Rc<RefCell<MessagePool>>,
    message_closure: Closure<dyn FnMut(JsValue)>,
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

        LobbyState {
            lobby: Lobby::new(lobby_settings),
            last_move_frame: 0,
            active_mage: None,
            particles: Vec::new(),
            message_pool,
            message_closure,
        }
    }

    pub fn lobby_id(&self) -> Result<LobbyID, LobbyError> {
        self.lobby
            .id
            .clone()
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
}

impl State for LobbyState {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;
        // DRAW background layer (board + UI block)

        // DRAW board

        {
            context.save();

            context.translate(BOARD_OFFSET_F64.0, BOARD_OFFSET_F64.1)?;

            draw_sprite(context, atlas, 256.0, 0.0, 256.0, 256.0, 0.0, 0.0)?;

            // DRAW particles

            for particle in self.particles.iter_mut() {
                particle.tick();
                draw_particle(context, atlas, &particle, frame)?;
            }

            self.particles.drain_filter(|particle| !particle.is_alive());

            {
                let game_started = self.lobby.all_ready() | self.lobby.is_local();
                let game_finished = self.lobby.finished();

                // DRAW mages
                for mage in self.lobby.game.iter_mages() {
                    context.save();

                    context.translate(
                        15.0 + mage.position.0 as f64 * BOARD_SCALE_F64.0,
                        15.0 + mage.position.1 as f64 * BOARD_SCALE_F64.1,
                    )?;

                    draw_mage(
                        context,
                        atlas,
                        mage,
                        frame,
                        self.lobby.game.turn_for(),
                        game_started,
                        game_finished,
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
                            (position.0 as f64 + 0.5) * BOARD_SCALE_F64.0,
                            (position.1 as f64 + 0.5) as f64 * BOARD_SCALE_F64.1,
                        )?;
                        context.rotate((ri / 2) as f64 * std::f64::consts::PI / 2.0)?;
                        let bop = (frame / 10 % 3) as f64;
                        context.translate(bop - 4.0, if is_diagonal { bop } else { 0.0 })?;
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
                        pointer.location(),
                        BOARD_OFFSET,
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
                    pointer.location(),
                    BOARD_OFFSET,
                    BOARD_SCALE,
                ) {
                    if let Some(occupant) = self.lobby.game.live_occupant(&selected_tile) {
                        if let Some(selected_tile) = self.lobby.game.location_as_position(
                            pointer.location(),
                            BOARD_OFFSET,
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

    fn tick(&mut self, app_context: &AppContext) -> Option<StateSort> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;
        let session_id = &app_context.session_id;

        let mut target_positions = Vec::new();

        {
            let mut message_pool = self.message_pool.borrow_mut();

            if let Ok(lobby_id) = self.lobby_id() {
                if message_pool.available(frame) {
                    if self.lobby.all_ready() {
                        let _ = fetch(&request_turns_since(lobby_id, self.lobby.game.turns()))
                            .then(&self.message_closure);
                    } else {
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
                message_pool
                    .messages
                    .append(&mut vec![Message::Move(turn.0)]);
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
                            send_ready(lobby_id, session_id.clone().unwrap());
                        }
                    }
                    _ => (),
                }
            }

            message_pool.clear();
        }

        if pointer.alt_clicked() {
            self.active_mage = None;
        }

        if pointer.clicked() {
            if let Some(selected_tile) =
                self.lobby
                    .game
                    .location_as_position(pointer.location(), BOARD_OFFSET, BOARD_SCALE)
            {
                if let Some(active_mage) = self.get_active_mage() {
                    let from = active_mage.position;

                    if let Some(mut move_targets) = self.lobby.game.take_move(from, selected_tile) {
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
