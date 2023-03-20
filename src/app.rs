use shared::{Lobby, LobbySort, Mage, Message, Position, Team, Turn};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::{
    draw::{draw_crosshair, draw_digits, draw_mage, draw_sprite, draw_sprite_scaled, draw_tooltip},
    net::{get_session_id, pathname, send_message},
};

pub const BOARD_OFFSET: (i32, i32) = (8, 8);
pub const BOARD_OFFSET_F64: (f64, f64) = (BOARD_OFFSET.0 as f64, BOARD_OFFSET.1 as f64);
pub const BOARD_SCALE: (i32, i32) = (30, 30);
pub const BOARD_SCALE_F64: (f64, f64) = (BOARD_SCALE.0 as f64, BOARD_SCALE.1 as f64);

enum ParticleSort {
    Missile,
    Overdrive,
}

struct Particle(f64, f64, f64, f64, u64, ParticleSort);

impl Particle {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        frame: u64,
    ) -> Result<(), JsValue> {
        context.save();
        context.translate(
            ((self.0 + 0.5) * BOARD_SCALE_F64.0).floor(),
            ((self.1 + 0.5) * BOARD_SCALE_F64.1).floor(),
        )?;

        let spin = self.4;
        let cycle = frame + (self.0 * 16.0) as u64 + (self.1 * 16.0) as u64 + spin;

        context.rotate((spin / 5) as f64 * std::f64::consts::PI / 2.0)?;
        // context.rotate(frame as f64 * 0.1)?;
        draw_sprite(
            context,
            atlas,
            64.0 + {
                let t = cycle % 24;
                if t > 16 {
                    16.0
                } else if t > 8 {
                    8.0
                } else {
                    0.0
                }
            } + {
                match self.5 {
                    ParticleSort::Missile => 0.0,
                    ParticleSort::Overdrive => 24.0,
                }
            },
            56.0,
            8.0,
            8.0,
            -4.0,
            -4.0,
        )?;
        context.restore();
        self.0 += self.2;
        self.1 += self.3;
        self.2 -= self.2 * 0.1;
        self.3 -= self.3 * 0.1;
        self.4 = self.4.saturating_sub(1);
        Ok(())
    }

    fn is_alive(&self) -> bool {
        self.4 > 1
        // true
    }
}

#[derive(Clone, Default)]
pub struct Pointer {
    previous: Option<Box<Pointer>>,
    pub location: (i32, i32),
    pub button: bool,
    pub alt_button: bool,
}

impl Pointer {
    fn new() -> Pointer {
        Pointer {
            ..Default::default()
        }
    }

    fn clicked(&self) -> bool {
        match &self.previous {
            Some(pointer) => self.button && !pointer.button,
            None => self.button,
        }
    }

    fn alt_clicked(&self) -> bool {
        match &self.previous {
            Some(pointer) => self.alt_button && !pointer.alt_button,
            None => self.alt_button,
        }
    }

    pub fn swap(&mut self) {
        self.previous.take(); // Must explicitly drop old Pointer from heap
        self.previous = Some(Box::new(self.clone()));
    }
}

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
        context.clear_rect(0.0, 0.0, 512.0, 544.0);
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
                particle.draw(context, atlas, self.frame)?;
            }

            self.particles.drain_filter(|particle| !particle.is_alive());

            {
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
                        self.lobby.all_ready() | self.lobby.is_local(),
                        self.lobby.finished(),
                    )?;

                    if mage.is_overdriven() && mage.is_alive() {
                        for _ in 0..(self.frame / 2 % 2) {
                            let d = js_sys::Math::random() * -std::f64::consts::PI * 0.9;
                            let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                            self.particles.push(Particle(
                                mage.position.0 as f64 + d.cos() * 0.4,
                                mage.position.1 as f64 - 0.15 + d.sin() * 0.4,
                                d.cos() * v,
                                d.sin() * v,
                                (js_sys::Math::random() * 30.0) as u64,
                                ParticleSort::Overdrive,
                            ));
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
                    for (position, overdrive) in &available_moves {
                        draw_sprite(
                            context,
                            atlas,
                            if *overdrive { 16.0 } else { 0.0 },
                            32.0,
                            16.0,
                            16.0,
                            position.0 as f64 * BOARD_SCALE_F64.0 + BOARD_OFFSET_F64.0 - 1.0,
                            position.1 as f64 * BOARD_SCALE_F64.1 + BOARD_OFFSET_F64.1 - 1.0,
                        )?;
                    }

                    if let Some(selected_tile) = self.lobby.game.location_as_position(
                        self.pointer.location,
                        BOARD_OFFSET,
                        BOARD_SCALE,
                    ) {
                        if available_moves
                            .iter()
                            .find(|(position, _)| position == &selected_tile)
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

        // DRAW UI block
        {
            context.save();

            {
                // DRAW active mage
                if let Some(mage) = self.get_active_mage() {
                    for i in 0..mage.mana.max {
                        if i < *mage.mana {
                            draw_sprite(
                                context,
                                atlas,
                                80.0,
                                0.0,
                                8.0,
                                8.0,
                                129.0 - (mage.mana.max * 5) as f64 + i as f64 * 10.0,
                                256.0,
                            )?;
                        } else {
                            draw_sprite(
                                context,
                                atlas,
                                88.0,
                                0.0,
                                8.0,
                                8.0,
                                129.0 - (mage.mana.max * 5) as f64 + i as f64 * 10.0,
                                256.0,
                            )?;
                        }
                    }
                }
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

        if let Some(selected_tile) =
            self.lobby
                .game
                .location_as_position(self.pointer.location, BOARD_OFFSET, BOARD_SCALE)
        {
            if let Some(occupant) = self.lobby.game.live_occupant(&selected_tile) {
                draw_tooltip(
                    context,
                    atlas,
                    (self.pointer.location.0, self.pointer.location.1 + 16),
                    24,
                )?;

                draw_digits(
                    context,
                    atlas,
                    (self.pointer.location.0 + 2, self.pointer.location.1 + 16),
                    *occupant.mana as usize,
                )?;

                draw_sprite(
                    context,
                    atlas,
                    80.0,
                    12.0,
                    4.0,
                    4.0,
                    self.pointer.location.0 as f64 + 11.0,
                    self.pointer.location.1 as f64 + 18.0,
                )?;

                draw_digits(
                    context,
                    atlas,
                    (self.pointer.location.0 + 17, self.pointer.location.1 + 16),
                    occupant.mana.max as usize,
                )?;
            }
        }

        context.restore();

        self.frame += 1;

        Ok(())
    }

    pub fn preprocess(&mut self, messages: &mut Vec<Message>) {
        if self.lobby.has_ai()
            && self.lobby.game.turn_for() == Team::Blue
            && self.frame - self.last_move_frame > 60
            && !self.lobby.finished()
        {
            let turn = self.lobby.game.best_turn();
            messages.append(&mut vec![Message::Move(turn.0)]);
        }
    }

    pub fn update(&mut self, messages: &Vec<Message>) {
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
                self.particles.push(Particle(
                    tile.0 as f64,
                    tile.1 as f64,
                    d.cos() * v,
                    d.sin() * v,
                    (js_sys::Math::random() * 50.0) as u64,
                    ParticleSort::Missile,
                ));
            }
        }
    }
}
