#![feature(drain_filter)]

use futures::TryFutureExt;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub, SubAssign};
use std::rc::Rc;
use std::{cell::RefCell, ops::AddAssign};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, Request, RequestInit, Response};

const BOARD_OFFSET: (i32, i32) = (8, 8);
const BOARD_OFFSET_F64: (f64, f64) = (BOARD_OFFSET.0 as f64, BOARD_OFFSET.1 as f64);
const BOARD_SCALE: (i32, i32) = (30, 30);
const BOARD_SCALE_F64: (f64, f64) = (BOARD_SCALE.0 as f64, BOARD_SCALE.1 as f64);

#[derive(PartialEq, Clone, Copy)]
struct Position(i8, i8);

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Position {
    fn within_bounds(&self, xmin: i8, xmax: i8, ymin: i8, ymax: i8) -> bool {
        self.0 >= xmin && self.0 < xmax && self.1 >= ymin && self.1 < ymax
    }
}

#[derive(PartialEq)]
enum Team {
    Red,
    Blue,
}

impl Team {
    fn enemy(&self) -> Team {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }
}

struct Mana {
    value: u8,
    max: u8,
}

impl Mana {
    fn with_max(max_mana: u8) -> Mana {
        Mana {
            value: max_mana,
            max: max_mana,
        }
    }

    fn is_overdriven(&self) -> bool {
        self.value <= 2
    }
}

impl Add<u8> for Mana {
    type Output = Mana;

    fn add(self, rhs: u8) -> Self::Output {
        Mana {
            max: self.max,
            value: self.value.saturating_add(rhs).min(self.max),
        }
    }
}

impl AddAssign<u8> for Mana {
    fn add_assign(&mut self, rhs: u8) {
        self.value = self.value.saturating_add(rhs).min(self.max)
    }
}

impl Sub<u8> for Mana {
    type Output = Mana;

    fn sub(self, rhs: u8) -> Self::Output {
        Mana {
            max: self.max,
            value: self.value.saturating_sub(rhs),
        }
    }
}

impl SubAssign<u8> for Mana {
    fn sub_assign(&mut self, rhs: u8) {
        self.value = self.value.saturating_sub(rhs);
    }
}

// enum SpellSort {
//     Damage(u8),
// }

struct Spell {
    // cost: u8,
    // sort: SpellSort,
    pattern: Vec<Position>,
}

impl Spell {
    fn new(
        // cost: u8,
        //  sort: SpellSort,
        pattern: Vec<Position>,
    ) -> Spell {
        Spell {
            // cost,
            // sort,
            pattern,
        }
    }

    fn select_missile(index: usize) -> Spell {
        let index = index % 4;

        match index {
            0 => Self::diamond_missile(),
            1 => Self::spike_missile(),
            2 => Self::knight_missile(),
            3 => Self::cross_missile(),
            _ => Self::default_missile(),
        }
    }

    fn default_missile() -> Spell {
        Self::new(
            // 1,
            // SpellSort::Damage(1),
            vec![
                Position(-2, 0),
                Position(-1, 0),
                Position(1, 0),
                Position(2, 0),
                Position(0, -2),
                Position(0, -1),
                Position(0, 1),
                Position(0, 2),
            ],
        )
    }

    fn diamond_missile() -> Spell {
        Self::new(
            // 1,
            // SpellSort::Damage(1),
            vec![
                Position(-2, 0),
                Position(-1, -1),
                Position(0, -2),
                Position(1, -1),
                Position(2, 0),
                Position(1, 1),
                Position(0, 2),
                Position(-1, 1),
            ],
        )
    }

    fn cross_missile() -> Spell {
        Self::new(
            // 1,
            // SpellSort::Damage(1),
            vec![
                Position(-2, -2),
                Position(-2, 2),
                Position(2, -2),
                Position(2, 2),
                Position(-1, -1),
                Position(-1, 1),
                Position(1, -1),
                Position(1, 1),
            ],
        )
    }

    fn knight_missile() -> Spell {
        Self::new(
            // 1,
            // SpellSort::Damage(1),
            vec![
                Position(-2, -1),
                Position(-1, -2),
                Position(1, 2),
                Position(2, 1),
                Position(1, -2),
                Position(2, -1),
                Position(-2, 1),
                Position(-1, 2),
            ],
        )
    }
    fn spike_missile() -> Spell {
        Self::new(
            // 1,
            // SpellSort::Damage(1),
            vec![
                Position(-2, -2),
                Position(-2, 2),
                Position(2, -2),
                Position(2, 2),
                Position(-1, 0),
                Position(0, -1),
                Position(1, 0),
                Position(0, 1),
            ],
        )
    }
}

struct Mage {
    index: usize,
    position: Position,
    mana: Mana,
    team: Team,
    spells: Vec<Spell>,
}

impl Mage {
    fn new(index: usize, position: Position, max_mana: u8, team: Team, spells: Vec<Spell>) -> Mage {
        Mage {
            index,
            position,
            team,
            mana: Mana::with_max(max_mana),
            spells,
        }
    }

    fn is_alive(&self) -> bool {
        self.mana.value > 0
    }

    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        index: usize,
        frame: u64,
        team: Team,
    ) -> Result<(), JsValue> {
        let bounce = (if self.team == team && self.is_alive() {
            2 - ((frame as i64 / 6 + index as i64 / 2) % 4 - 2).abs()
        } else {
            0
        }) as f64;

        let sleeping_offset = if self.is_alive() { 0.0 } else { 64.0 };

        match self.team {
            Team::Red => context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    atlas,
                    32.0 * (index / 2) as f64,
                    64.0 + sleeping_offset,
                    32.0,
                    32.0,
                    0.0,
                    0.0 + bounce,
                    32.0,
                    32.0,
                )?,
            Team::Blue => context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    atlas,
                    32.0 * (index / 2) as f64,
                    96.0 + sleeping_offset,
                    32.0,
                    32.0,
                    0.0,
                    0.0 + bounce,
                    32.0,
                    32.0,
                )?,
        }

        Ok(())
    }

    fn available_moves(&self, game: &Game) -> Vec<(Position, bool)> {
        const DIRS: [(Position, bool); 8] = [
            (Position(0, -1), false),
            (Position(-1, 0), false),
            (Position(1, 0), false),
            (Position(0, 1), false),
            (Position(-1, -1), true),
            (Position(-1, 1), true),
            (Position(1, -1), true),
            (Position(1, 1), true),
        ];

        let mut moves = Vec::with_capacity(DIRS.len());

        for (dir, overdrive) in DIRS {
            let position = &self.position + &dir;

            if position.within_bounds(0, game.board.width as i8, 0, game.board.height as i8)
                && !game.occupied(&position)
                && !(overdrive && !self.mana.is_overdriven())
            {
                moves.push((position, overdrive));
            }
        }

        return moves;
    }
}

struct Board {
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Result<Board, &'static str> {
        match (width, height) {
            (width, height) if width >= 4 && width <= 8 && height >= 4 && height <= 8 => {
                Ok(Board { width, height })
            }
            _ => Err("board size does not conform to limits"),
        }
    }
}

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

        let cycle = frame + (self.0 * 16.0) as u64 + (self.1 * 16.0) as u64;

        context.rotate((cycle / 10) as f64 * std::f64::consts::PI / 2.0)?;
        // context.rotate(frame as f64 * 0.1)?;
        draw_sprite(
            context,
            atlas,
            64.0 + {
                let t = cycle % 20;
                if t > 15 {
                    16.0
                } else if t > 10 {
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

struct Game {
    board: Board,
    mages: Vec<Mage>,
    particles: Vec<Particle>,
    active_mage: Option<usize>,
    turn: usize,
}

impl Game {
    fn new(
        board_width: usize,
        board_height: usize,
        mage_count: usize,
    ) -> Result<Game, &'static str> {
        if mage_count >= (board_width - 1) * (board_height - 1) {
            Err("game contains too many mages for board")
        } else {
            let board = Board::new(board_width, board_height)?;
            let mut mages = Vec::with_capacity(mage_count * 2);

            let x_offset = ((board_width - mage_count) / 2) as i8;

            for i in 0..mage_count {
                let x = x_offset + i as i8;

                mages.push(Mage::new(
                    i * 2,
                    Position(x, board_height as i8 - 2),
                    8,
                    Team::Red,
                    vec![Spell::select_missile(i)],
                ));

                mages.push(Mage::new(
                    i * 2 + 1,
                    Position(board_width as i8 - x - 1, 1),
                    8,
                    Team::Blue,
                    vec![Spell::select_missile(i)],
                ));
            }

            let active_mage = None;
            let turn = 0;
            let particles = Vec::new();

            Ok(Game {
                board,
                mages,
                particles,
                active_mage,
                turn,
            })
        }
    }

    fn get_active_mage(&self) -> Option<&Mage> {
        // Index guaranteed to be within bounds
        if let Some(active_mage) = self.active_mage {
            self.mages.get(active_mage)
        } else {
            None
        }
    }

    fn get_active_mage_mut(&mut self) -> Option<&mut Mage> {
        // Index guaranteed to be within bounds
        if let Some(active_mage) = self.active_mage {
            self.mages.get_mut(active_mage)
        } else {
            None
        }
    }

    fn occupant(&self, position: &Position) -> Option<&Mage> {
        for mage in self.mages.iter() {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    fn occupied(&self, position: &Position) -> bool {
        self.occupant(position).is_some()
    }

    fn live_occupant(&self, position: &Position) -> Option<&Mage> {
        for mage in self.mages.iter().filter(|mage| mage.is_alive()) {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    fn live_occupant_mut(&mut self, position: &Position) -> Option<&mut Mage> {
        for mage in self.mages.iter_mut().filter(|mage| mage.is_alive()) {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    // fn live_occupied(&self, position: &Position) -> bool {
    //     self.live_occupant(position).is_some()
    // }

    fn live_occupied_by(&self, position: &Position, team: Team) -> bool {
        if let Some(occupant) = self.live_occupant(position) {
            occupant.team == team
        } else {
            false
        }
    }

    fn turn_for(&self) -> Team {
        if self.turn % 2 == 0 {
            Team::Red
        } else {
            Team::Blue
        }
    }

    fn location_as_position(
        &self,
        location: (i32, i32),
        offset: (i32, i32),
        scale: (i32, i32),
    ) -> Option<Position> {
        let position = Position(
            ((location.0 - offset.0) / scale.0) as i8,
            ((location.1 - offset.1) / scale.1) as i8,
        );

        if (location.0 - offset.0) >= 0
            && position.1 < self.board.width as i8
            && (location.1 - offset.1) >= 0
            && position.1 < self.board.height as i8
        {
            Some(position)
        } else {
            None
        }
    }

    fn select_mage_at(&mut self, selected_tile: &Position) {
        if let Some(occupant) = self.live_occupant(selected_tile) {
            if occupant.team == self.turn_for() {
                self.active_mage = Some(occupant.index);
            } else {
                self.active_mage = None;
            }
        } else {
            self.active_mage = None;
        }
    }

    fn end_turn(&mut self) {
        self.turn += 1;
        self.active_mage = None;
    }

    fn attack(&mut self) -> usize {
        let mut hits = 0;

        if let Some(active_mage) = self.get_active_mage() {
            let targets = self.targets(active_mage, active_mage.position, 0);

            for (is_enemy, tile) in targets {
                if is_enemy {
                    hits += 1;
                    self.live_occupant_mut(&tile).unwrap().mana -= 1;

                    for _ in 0..20 {
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

        hits
    }

    fn targets(&self, mage: &Mage, tile: Position, spell_index: usize) -> Vec<(bool, Position)> {
        if let Some(spell) = mage.spells.get(spell_index) {
            let mut moves = Vec::with_capacity(spell.pattern.len());

            for dir in &spell.pattern {
                let position = &tile + dir;

                if position.within_bounds(0, self.board.width as i8, 0, self.board.height as i8) {
                    moves.push((
                        self.live_occupied_by(&position, mage.team.enemy()),
                        position,
                    ));
                }
            }

            return moves;
        } else {
            return Vec::new();
        }
    }

    fn update(&mut self, pointer: &Pointer) {
        if pointer.clicked() {
            if let Some(selected_tile) =
                self.location_as_position(pointer.location, BOARD_OFFSET, BOARD_SCALE)
            {
                if let Some(active_mage) = self.get_active_mage() {
                    let available_moves = active_mage.available_moves(self);
                    let potential_move = available_moves
                        .iter()
                        .find(|(position, _)| *position == selected_tile);

                    if let Some((position, _)) = potential_move {
                        self.get_active_mage_mut().unwrap().position = *position;
                        self.attack();
                        self.end_turn();
                    } else {
                        self.select_mage_at(&selected_tile);
                    }
                } else {
                    self.select_mage_at(&selected_tile);
                }
            }
        } else if pointer.alt_clicked() {
            // if self.attack() > 0 {
            //     self.end_turn();
            // }
        }
    }
}

#[derive(Clone)]
struct Pointer {
    previous: Option<Box<Pointer>>,
    location: (i32, i32),
    lmb: bool,
    rmb: bool,
}

impl Pointer {
    fn clicked(&self) -> bool {
        match &self.previous {
            Some(pointer) => self.lmb && !pointer.lmb,
            None => self.lmb,
        }
    }

    fn alt_clicked(&self) -> bool {
        match &self.previous {
            Some(pointer) => self.rmb && !pointer.rmb,
            None => self.rmb,
        }
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn draw_sprite(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
    dx: f64,
    dy: f64,
) -> Result<(), JsValue> {
    context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        atlas, sx, sy, sw, sh, dx, dy, sw, sh,
    )?;

    Ok(())
}

fn draw_sprite_scaled(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
    dx: f64,
    dy: f64,
    dw: f64,
    dh: f64,
) -> Result<(), JsValue> {
    context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        atlas, sx, sy, sw, sh, dx, dy, dw, dh,
    )?;

    Ok(())
}

fn draw_crosshair(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    position: &Position,
    offset: (f64, f64),
    frame: u64,
) -> Result<(), JsValue> {
    draw_sprite(
        context,
        atlas,
        offset.0,
        offset.1,
        8.0,
        8.0,
        position.0 as f64 * BOARD_SCALE_F64.0 + (frame / 6 % 4) as f64,
        position.1 as f64 * BOARD_SCALE_F64.1 + (frame / 6 % 4) as f64,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0 + 8.0,
        offset.1,
        8.0,
        8.0,
        position.0 as f64 * BOARD_SCALE_F64.0 - (frame / 6 % 4) as f64 + BOARD_SCALE_F64.0 - 8.0,
        position.1 as f64 * BOARD_SCALE_F64.1 + (frame / 6 % 4) as f64,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0,
        offset.1 + 8.0,
        8.0,
        8.0,
        position.0 as f64 * BOARD_SCALE_F64.0 + (frame / 6 % 4) as f64,
        position.1 as f64 * BOARD_SCALE_F64.1 - (frame / 6 % 4) as f64 + BOARD_SCALE_F64.1 - 8.0,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0 + 8.0,
        offset.1 + 8.0,
        8.0,
        8.0,
        position.0 as f64 * BOARD_SCALE_F64.0 - (frame / 6 % 4) as f64 + BOARD_SCALE_F64.0 - 8.0,
        position.1 as f64 * BOARD_SCALE_F64.1 - (frame / 6 % 4) as f64 + BOARD_SCALE_F64.1 - 8.0,
    )?;

    Ok(())
}

fn draw_digits(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    location: (i32, i32),
    num: usize,
) -> Result<(), JsValue> {
    let digits = num.max(1).ilog10();

    for i in (0..=digits).rev() {
        let digit = num / 10usize.pow(i) % 10;

        draw_sprite(
            context,
            atlas,
            digit as f64 * 8.0,
            48.0,
            8.0,
            8.0,
            location.0 as f64 + (digits - i) as f64 * 8.0,
            location.1 as f64,
        )?;
    }

    Ok(())
}

fn draw_tooltip(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    location: (i32, i32),
    width: usize,
) -> Result<(), JsValue> {
    draw_sprite(
        context,
        atlas,
        96.0,
        0.0,
        1.0,
        12.0,
        location.0 as f64,
        location.1 as f64 - 2.0,
    )?;

    draw_sprite(
        context,
        atlas,
        96.0,
        0.0,
        1.0,
        12.0,
        (location.0 + width as i32 + 1) as f64,
        location.1 as f64 - 2.0,
    )?;

    draw_sprite_scaled(
        context,
        atlas,
        97.0,
        0.0,
        1.0,
        12.0,
        (location.0 + 1) as f64,
        location.1 as f64 - 2.0,
        width as f64,
        12.0,
    )?;

    Ok(())
}

fn draw(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    game: &mut Game,
    pointer: &Pointer,
    frame: u64,
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

        for particle in game.particles.iter_mut() {
            particle.draw(context, atlas, frame)?;
        }

        game.particles.drain_filter(|particle| !particle.is_alive());

        {
            // DRAW mages
            for mage in game.mages.iter() {
                context.save();

                context.translate(
                    -1.0 + mage.position.0 as f64 * BOARD_SCALE_F64.0,
                    -1.0 + mage.position.1 as f64 * BOARD_SCALE_F64.1,
                )?;

                mage.draw(context, atlas, mage.index, frame, game.turn_for())?;

                if mage.mana.is_overdriven() && mage.is_alive() {
                    for _ in 0..(frame / 2 % 2) {
                        let d = js_sys::Math::random() * -std::f64::consts::PI * 0.9;
                        let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                        game.particles.push(Particle(
                            mage.position.0 as f64 + d.cos() * 0.4,
                            mage.position.1 as f64 - 0.15 + d.sin() * 0.4,
                            d.cos() * v,
                            d.sin() * v,
                            (js_sys::Math::random() * 50.0) as u64,
                            ParticleSort::Overdrive,
                        ));
                    }
                }

                if game.active_mage == Some(mage.index) {
                    draw_sprite(
                        context,
                        atlas,
                        72.0,
                        0.0,
                        8.0,
                        5.0,
                        12.0,
                        -1.0 - (frame / 6 % 6) as f64,
                    )?;
                }

                context.restore();
            }
        }

        {
            // DRAW markers
            context.save();

            if let Some(mage) = game.get_active_mage() {
                let available_moves = mage.available_moves(game);
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

                if let Some(selected_tile) =
                    game.location_as_position(pointer.location, BOARD_OFFSET, BOARD_SCALE)
                {
                    if available_moves
                        .iter()
                        .find(|(position, _)| position == &selected_tile)
                        .is_some()
                    {
                        for (enemy_occupied, position) in &game.targets(mage, selected_tile, 0) {
                            if *enemy_occupied {
                                draw_crosshair(context, atlas, position, (64.0, 32.0), frame)?;
                            } else {
                                draw_crosshair(context, atlas, position, (48.0, 32.0), 0)?;
                            }
                        }
                    }
                }
            }

            context.restore();

            if let Some(selected_tile) =
                game.location_as_position(pointer.location, BOARD_OFFSET, BOARD_SCALE)
            {
                draw_crosshair(context, atlas, &selected_tile, (32.0, 32.0), frame)?;
            }
        }

        context.restore();
    }

    // DRAW UI block
    {
        context.save();

        context.translate(8.0, 8.0)?;

        context.translate(0.0, 248.0)?;
        {
            // DRAW active mage
            if let Some(active_mage) = game.get_active_mage() {
                for i in 0..active_mage.mana.max {
                    if i < active_mage.mana.value {
                        draw_sprite(context, atlas, 80.0, 0.0, 8.0, 8.0, i as f64 * 10.0, 0.0)?;
                    } else {
                        draw_sprite(context, atlas, 88.0, 0.0, 8.0, 8.0, i as f64 * 10.0, 0.0)?;
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
        pointer.location.0 as f64 - 5.0,
        pointer.location.1 as f64 - 1.0,
    )?;

    if let Some(selected_tile) =
        game.location_as_position(pointer.location, BOARD_OFFSET, BOARD_SCALE)
    {
        if let Some(occupant) = game.live_occupant(&selected_tile) {
            draw_tooltip(
                context,
                atlas,
                (pointer.location.0, pointer.location.1 + 16),
                24,
            )?;

            draw_digits(
                context,
                atlas,
                (pointer.location.0 + 2, pointer.location.1 + 16),
                occupant.mana.value as usize,
            )?;

            draw_sprite(
                context,
                atlas,
                80.0,
                12.0,
                4.0,
                4.0,
                pointer.location.0 as f64 + 11.0,
                pointer.location.1 as f64 + 18.0,
            )?;

            draw_digits(
                context,
                atlas,
                (pointer.location.0 + 17, pointer.location.1 + 16),
                occupant.mana.max as usize,
            )?;
        }
    }

    context.restore();

    Ok(())
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(512);
    canvas.set_height(544);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    context.set_image_smoothing_enabled(false);

    let atlas = document
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();

    atlas.set_src(&"img/atlas.png");

    let pointer = Rc::new(RefCell::new(Pointer {
        previous: None,
        location: (0, 0),
        lmb: false,
        rmb: false,
    }));

    let request = {
        let mut opts = RequestInit::new();
        opts.method("GET");

        let url = format!("test");

        Request::new_with_str_and_init(&url, &opts).unwrap()
    };

    let message_pool: Rc<RefCell<MessagePool>> = Rc::new(RefCell::new(MessagePool::new()));

    let message_closure = {
        let message_pool = message_pool.clone();

        Closure::<dyn FnMut(JsValue)>::new(move |value| {
            let mut message_pool = message_pool.borrow_mut();
            let message: Message = serde_wasm_bindgen::from_value(value).unwrap();
            message_pool.messages.clear();
            message_pool.messages.push(message);

            message_pool.polling = false;
        })
    };

    let game = Game::new(8, 8, 4).unwrap();
    let game = Rc::new(RefCell::new(game));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut frame = 0;

    {
        let pointer = pointer.clone();
        let game = game.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            let message_pool = message_pool.clone();
            let mut game = game.borrow_mut();

            {
                let pointer = pointer.borrow();

                draw(&context, &atlas, &mut game, &pointer, frame).unwrap();
                game.update(&pointer);
            }

            {
                let mut pointer = pointer.borrow_mut();
                pointer.previous.take();
                pointer.previous = Some(Box::new(pointer.clone()));
            }

            if !message_pool.borrow().polling {
                // let resp_value =
                //     JsFuture::from(web_sys::window().unwrap().fetch_with_request(&request))
                //         .and_then(|v| {
                //             assert!(v.is_instance_of::<Response>());
                //             let resp: Response = v.dyn_into().unwrap();
                //             JsFuture::from(resp.json().unwrap())
                //         });

                // let promise = future_to_promise(resp_value);
                // promise.then(&message_closure);
                // promise.catch(&Closure::<dyn FnMut(_)>::new(|value: JsValue| {
                //     web_sys::console::log_1(&value);
                // }));

                // message_pool.borrow_mut().polling = true;
            }

            frame += 1;

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    {
        let pointer = pointer.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut pointer = pointer.borrow_mut();

            match event.button() {
                0 => pointer.lmb = true,
                2 => pointer.rmb = true,
                _ => (),
            }
        });
        document.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut pointer = pointer.borrow_mut();

            match event.button() {
                0 => pointer.lmb = false,
                2 => pointer.rmb = false,
                _ => (),
            }
        });
        document.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let canvas = Rc::new(canvas);
    let bound: Rc<RefCell<Option<web_sys::DomRect>>> =
        Rc::new(RefCell::new(Some(canvas.get_bounding_client_rect())));

    {
        let canvas = canvas.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_: JsValue| {
            bound.replace(Some(canvas.get_bounding_client_rect()));
        });
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                let mut pointer = pointer.borrow_mut();

                let x = event.client_x() - bound.left() as i32;
                let y = event.client_y() - bound.top() as i32;
                let x = (x as f64 * (512.0 / bound.width())) as i32;
                let y = (y as f64 * (512.0 / bound.width())) as i32;

                pointer.location = (x / 2, y / 2);
            }

            event.prevent_default();
        });
        document.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::TouchEvent| {
            let mut pointer = pointer.borrow_mut();
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                if let Some(touch) = event.target_touches().item(0) {
                    let x = touch.client_x() - bound.left() as i32;
                    let y = touch.client_y() - bound.top() as i32;

                    let x = (x as f64 * (512.0 / bound.width())) as i32;
                    let y = (y as f64 * (512.0 / bound.width())) as i32;

                    pointer.location = (x as i32 / 2, y as i32 / 2);
                }
            }
        });
        document.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let bound = bound.clone();
        let game = game.clone();

        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::TouchEvent| {
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                if let Some(touch) = event.target_touches().item(0) {
                    let mut pointer = pointer.borrow_mut();
                    let game = game.borrow();

                    let x = touch.client_x() - bound.left() as i32;
                    let y = touch.client_y() - bound.top() as i32;

                    let x = (x as f64 * (512.0 / bound.width())) as i32;
                    let y = (y as f64 * (512.0 / bound.width())) as i32;

                    let pointer_location = (x as i32 / 2, y as i32 / 2);

                    if (pointer_location.0 - pointer.location.0).abs() < 16
                        && (pointer_location.1 - pointer.location.1).abs() < 16
                    {
                        pointer.lmb = true;
                    } else if let Some(selected_tile) =
                        game.location_as_position(pointer_location, BOARD_OFFSET, BOARD_SCALE)
                    {
                        if let Some(_) = game.live_occupant(&selected_tile) {
                            pointer.lmb = true;
                        }
                    }

                    pointer.location = (x as i32 / 2, y as i32 / 2);
                }
            }
        });
        document
            .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::TouchEvent| {
            let mut pointer = pointer.borrow_mut();

            pointer.lmb = false;
        });
        document.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
        });
        document
            .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

struct MessagePool {
    messages: Vec<Message>,
    polling: bool,
}

impl MessagePool {
    fn new() -> MessagePool {
        MessagePool {
            messages: Vec::new(),
            polling: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    value: String,
}
