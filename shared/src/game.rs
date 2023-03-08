use serde::{Deserialize, Serialize};
use std::{
    ops::{Add, AddAssign, Deref, Sub, SubAssign},
    slice::Iter,
};

use crate::Position;

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum Team {
    Red,
    Blue,
}

impl Team {
    pub fn enemy(&self) -> Team {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mana {
    value: u8,
    pub max: u8,
}

impl Mana {
    fn with_max(max_mana: u8) -> Mana {
        Mana {
            value: max_mana,
            max: max_mana,
        }
    }

    pub fn is_overdriven(&self) -> bool {
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

impl Deref for Mana {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spell {
    // cost: u8,
    pattern: Vec<Position>,
}

impl Spell {
    pub fn new(pattern: Vec<Position>) -> Spell {
        Spell { pattern }
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
        Self::new(vec![
            Position(-2, 0),
            Position(-1, 0),
            Position(1, 0),
            Position(2, 0),
            Position(0, -2),
            Position(0, -1),
            Position(0, 1),
            Position(0, 2),
        ])
    }

    fn diamond_missile() -> Spell {
        Self::new(vec![
            Position(-2, 0),
            Position(-1, -1),
            Position(0, -2),
            Position(1, -1),
            Position(2, 0),
            Position(1, 1),
            Position(0, 2),
            Position(-1, 1),
        ])
    }

    fn cross_missile() -> Spell {
        Self::new(vec![
            Position(-2, -2),
            Position(-2, 2),
            Position(2, -2),
            Position(2, 2),
            Position(-1, -1),
            Position(-1, 1),
            Position(1, -1),
            Position(1, 1),
        ])
    }

    fn knight_missile() -> Spell {
        Self::new(vec![
            Position(-2, -1),
            Position(-1, -2),
            Position(1, 2),
            Position(2, 1),
            Position(1, -2),
            Position(2, -1),
            Position(-2, 1),
            Position(-1, 2),
        ])
    }
    fn spike_missile() -> Spell {
        Self::new(vec![
            Position(-2, -2),
            Position(-2, 2),
            Position(2, -2),
            Position(2, 2),
            Position(-1, 0),
            Position(0, -1),
            Position(1, 0),
            Position(0, 1),
        ])
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mage {
    pub index: usize,
    pub position: Position,
    pub mana: Mana,
    pub team: Team,
    pub spell: Spell,
}

impl Mage {
    pub fn new(index: usize, position: Position, max_mana: u8, team: Team, spell: Spell) -> Mage {
        Mage {
            index,
            position,
            team,
            mana: Mana::with_max(max_mana),
            spell,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.mana.value > 0
    }

    pub fn available_moves(&self, game: &Game) -> Vec<(Position, bool)> {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Result<Board, &'static str> {
        match (width, height) {
            (width, height) if width >= 4 && width <= 8 && height >= 4 && height <= 8 => {
                Ok(Board { width, height })
            }
            _ => Err("board size does not conform to limits"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    board: Board,
    mages: Vec<Mage>,
    active_mage: Option<usize>,
    turn: usize,
}

impl Game {
    pub fn new(
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
                    Spell::select_missile(i),
                ));

                mages.push(Mage::new(
                    i * 2 + 1,
                    Position(board_width as i8 - x - 1, 1),
                    8,
                    Team::Blue,
                    Spell::select_missile(i),
                ));
            }

            let active_mage = None;
            let turn = 0;
            // let particles = Vec::new();

            Ok(Game {
                board,
                mages,
                // particles,
                active_mage,
                turn,
            })
        }
    }

    pub fn iter_mages(&self) -> Iter<Mage> {
        self.mages.iter()
    }

    pub fn get_active_mage(&self) -> Option<&Mage> {
        // Index guaranteed to be within bounds
        if let Some(active_mage) = self.active_mage {
            self.mages.get(active_mage)
        } else {
            None
        }
    }

    pub fn is_mage_active(&self, mage: &Mage) -> bool {
        if let Some(active_mage) = self.get_active_mage() {
            active_mage.index == mage.index
        } else {
            false
        }
    }

    pub fn get_active_mage_mut(&mut self) -> Option<&mut Mage> {
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

    pub fn live_occupant(&self, position: &Position) -> Option<&Mage> {
        for mage in self.mages.iter().filter(|mage| mage.is_alive()) {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    pub fn live_occupant_mut(&mut self, position: &Position) -> Option<&mut Mage> {
        for mage in self.mages.iter_mut().filter(|mage| mage.is_alive()) {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    fn live_occupied_by(&self, position: &Position, team: Team) -> bool {
        if let Some(occupant) = self.live_occupant(position) {
            occupant.team == team
        } else {
            false
        }
    }

    pub fn turn_for(&self) -> Team {
        if self.turn % 2 == 0 {
            Team::Red
        } else {
            Team::Blue
        }
    }

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

    pub fn select_mage_at(&mut self, selected_tile: &Position) {
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

    pub fn end_turn(&mut self) {
        self.turn += 1;
        self.active_mage = None;
    }

    pub fn take_move(&mut self, from: Position, to: Position) -> Option<Vec<Position>> {
        self.select_mage_at(&from);

        if let Some(active_mage) = self.get_active_mage() {
            let available_moves = active_mage.available_moves(self);
            let potential_move = available_moves.iter().find(|(position, _)| *position == to);

            if let Some((position, _)) = potential_move {
                self.get_active_mage_mut().unwrap().position = *position;
                let tiles = self.attack();
                self.end_turn();

                return Some(tiles);
            }
        }

        None
    }

    pub fn attack(&mut self) -> Vec<Position> {
        let mut hits = Vec::new();

        if let Some(active_mage) = self.get_active_mage() {
            let targets = self.targets(active_mage, active_mage.position);

            for (is_enemy, tile) in targets {
                if is_enemy {
                    self.live_occupant_mut(&tile).unwrap().mana -= 1;
                    hits.push(tile);
                }
            }
        }

        hits
    }

    pub fn targets(&self, mage: &Mage, tile: Position) -> Vec<(bool, Position)> {
        let mut moves = Vec::with_capacity(mage.spell.pattern.len());

        for dir in &mage.spell.pattern {
            let position = &tile + dir;

            if position.within_bounds(0, self.board.width as i8, 0, self.board.height as i8) {
                moves.push((
                    self.live_occupied_by(&position, mage.team.enemy()),
                    position,
                ));
            }
        }

        return moves;
    }
}
