use shared::{Lobby, LobbySort, Mage, Message, Position, Team, Turn};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::{
    Particle, ParticleSort, Pointer, BOARD_OFFSET, BOARD_OFFSET_F64, BOARD_SCALE, BOARD_SCALE_F64,
};
use crate::{
    draw::{
        draw_crosshair, draw_mage, draw_particle, draw_sprite, draw_sprite_scaled,
        rotation_from_position,
    },
    net::{get_session_id, pathname, send_message},
    window,
};

pub struct App {
    pub lobby: Lobby,
    particles: Vec<Particle>,
    pub frame: u64,
    last_move_frame: u64,
    active_mage: Option<usize>,
    pub session_id: Option<String>,
    pub pointer: Pointer,
}

impl App {
    pub fn new() -> App {
        let pathname = pathname();
        let lobby_sort = match pathname.as_str() {
            "/local" => LobbySort::Local,
            "/local/ai" => LobbySort::LocalAI,
            _ => LobbySort::Online,
        };

        App {
            lobby: Lobby::new(lobby_sort),
            particles: Vec::new(),
            frame: 0,
            last_move_frame: 0,
            active_mage: None,
            session_id: get_session_id(),
            pointer: Pointer::new(),
        }
    }

    pub fn in_lobby(&self) -> bool {
        self.lobby.has_session_id(self.session_id.as_ref())
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

    pub fn select_mage_at(&mut self, selected_tile: &Position) {
        if self.lobby.is_active_player(self.session_id.as_ref()) {
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

    pub fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
    ) -> Result<(), JsValue> {
        context.clear_rect(0.0, 0.0, 512.0, 512.0);
        context.save();

        context.scale(2.0, 2.0)?;

        // DRAW background layer (board + UI block)

        // DRAW board

        {
            context.save();

            context.translate(BOARD_OFFSET_F64.0, BOARD_OFFSET_F64.1)?;

            draw_sprite_scaled(context, atlas, 64.0, 0.0, 8.0, 8.0, 0.0, 0.0, 240.0, 240.0)?;

            // DRAW particles

            for particle in self.particles.iter_mut() {
                particle.tick();
                draw_particle(context, atlas, &particle, self.frame)?;
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
                        self.frame,
                        self.lobby.game.turn_for(),
                        game_started,
                        game_finished,
                    )?;

                    if mage.is_alive() {
                        if mage.has_diagonals() {
                            for _ in 0..(self.frame / 3 % 2) {
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
                            -4.0,
                            -17.0 - (self.frame / 6 % 6) as f64,
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
                            position.0 as f64 * BOARD_SCALE_F64.0 + BOARD_OFFSET_F64.0 + 7.0,
                            position.1 as f64 * BOARD_SCALE_F64.1 + BOARD_OFFSET_F64.1 + 7.0,
                        )?;
                        context.rotate((ri / 2) as f64 * std::f64::consts::PI / 2.0)?;
                        let bop = (self.frame / 10 % 3) as f64;
                        context.translate(bop, if is_diagonal { bop } else { 0.0 })?;
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
                        self.pointer.location,
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
                                    draw_crosshair(
                                        context,
                                        atlas,
                                        position,
                                        (64.0, 32.0),
                                        self.frame,
                                    )?;
                                } else {
                                    draw_crosshair(context, atlas, position, (48.0, 32.0), 0)?;
                                }
                            }
                        }
                    }
                }

                if let Some(selected_tile) = self.lobby.game.location_as_position(
                    self.pointer.location,
                    BOARD_OFFSET,
                    BOARD_SCALE,
                ) {
                    if let Some(occupant) = self.lobby.game.live_occupant(&selected_tile) {
                        if let Some(selected_tile) = self.lobby.game.location_as_position(
                            self.pointer.location,
                            BOARD_OFFSET,
                            BOARD_SCALE,
                        ) {
                            for (_, position) in &self.lobby.game.targets(occupant, selected_tile) {
                                draw_crosshair(context, atlas, position, (80.0, 32.0), 0)?;
                            }
                        }
                    }
                    draw_crosshair(context, atlas, &selected_tile, (32.0, 32.0), self.frame)?;
                }

                context.restore();
            }

            context.restore();
        }

        // DRAW cursor
        draw_sprite(
            context,
            atlas,
            64.0,
            8.0,
            16.0,
            16.0,
            self.pointer.location.0 as f64 - 5.0,
            self.pointer.location.1 as f64 - 1.0,
        )?;

        context.restore();

        self.frame += 1;

        Ok(())
    }

    pub fn preprocess(&mut self, messages: &mut Vec<Message>) {
        if self.lobby.has_ai()
            && self.lobby.game.turn_for() == Team::Blue
            && self.frame - self.last_move_frame > 45
            && !self.lobby.finished()
        {
            let turn = self
                .lobby
                .game
                .best_turn(window().performance().unwrap().now().to_bits());
            messages.append(&mut vec![Message::Move(turn.0)]);
        }
    }

    pub fn tick(&mut self, messages: &Vec<Message>) {
        let mut target_positions = Vec::new();

        for message in messages {
            match message {
                Message::Moves(turns) => {
                    for Turn(from, to) in turns {
                        if let Some(mut move_targets) = self.lobby.game.take_move(*from, *to) {
                            target_positions.append(&mut move_targets);

                            self.last_move_frame = self.frame;
                        }
                    }
                }
                Message::Move(Turn(from, to)) => {
                    if let Some(mut move_targets) = self.lobby.game.take_move(*from, *to) {
                        target_positions.append(&mut move_targets);

                        self.last_move_frame = self.frame;
                    }
                }
                _ => (),
            }
        }

        if self.pointer.alt_clicked() {
            self.active_mage = None;
        }

        if self.pointer.clicked() {
            if let Some(selected_tile) = self.lobby.game.location_as_position(
                self.pointer.location,
                BOARD_OFFSET,
                BOARD_SCALE,
            ) {
                if let Some(active_mage) = self.get_active_mage() {
                    let from = active_mage.position;

                    if let Some(mut move_targets) = self.lobby.game.take_move(from, selected_tile) {
                        if !self.lobby.is_local() && self.session_id.is_some() {
                            send_message(
                                self.session_id.clone().unwrap(),
                                Message::Move(Turn(from, selected_tile)),
                            );
                        }

                        target_positions.append(&mut move_targets);

                        self.active_mage = None;
                        self.last_move_frame = self.frame;
                    } else {
                        self.select_mage_at(&selected_tile);
                    }
                } else {
                    self.select_mage_at(&selected_tile);
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
    }
}
