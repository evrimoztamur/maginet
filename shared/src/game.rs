use std::{
    ops::{Add, AddAssign, Deref, Sub, SubAssign},
    slice::Iter,
};

use serde::{Deserialize, Serialize};

use crate::Position;

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn is_overdriven(&self) -> bool {
        self.mana.value <= 2
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Turn(pub Position, pub Position);

impl Turn {
    pub fn sentinel() -> Turn {
        Turn(Position(0, 0), Position(0, 0))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    board: Board,
    mages: Vec<Mage>,
    turns: Vec<Turn>,
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

            let turns = Vec::new();

            Ok(Game {
                board,
                mages,
                turns,
            })
        }
    }

    pub fn turns_since(&self, since: usize) -> Vec<&Turn> {
        self.turns.iter().skip(since).collect()
    }

    pub fn last_turn(&self) -> Option<Turn> {
        self.turns.last().copied()
    }

    pub fn iter_mages(&self) -> Iter<Mage> {
        self.mages.iter()
    }

    pub fn get_mage(&self, index: usize) -> Option<&Mage> {
        self.mages.get(index)
    }

    pub fn get_mage_mut(&mut self, index: usize) -> Option<&mut Mage> {
        self.mages.get_mut(index)
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

    pub fn turns(&self) -> usize {
        self.turns.len()
    }

    pub fn turn_index(&self) -> usize {
        self.turns() % 2
    }

    pub fn turn_for(&self) -> Team {
        if self.turn_index() == 0 {
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
            && position.0 < self.board.width as i8
            && (location.1 - offset.1) >= 0
            && position.1 < self.board.height as i8
        {
            Some(position)
        } else {
            None
        }
    }

    pub fn available_moves(&self, mage: &Mage) -> Vec<(Position, bool)> {
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
            let position = &mage.position + &dir;

            if position.within_bounds(0, self.board.width as i8, 0, self.board.height as i8)
                && !self.occupied(&position)
                && !(overdrive && !mage.is_overdriven())
            {
                moves.push((position, overdrive));
            }
        }

        return moves;
    }

    pub fn all_available_turns(&self) -> Vec<Turn> {
        self.mages
            .iter()
            .map(|mage| (mage, self.available_moves(mage)))
            .map(|(mage, moves)| {
                moves
                    .iter()
                    .map(|(to, _)| Turn(mage.position, *to))
                    .collect::<Vec<Turn>>()
            })
            .flatten()
            .collect::<Vec<Turn>>()
    }

    pub fn evaluate(&self) -> isize {
        let mana_diff: isize = self
            .mages
            .iter()
            .map(|mage| match mage.team {
                Team::Red => mage.mana.value as isize,
                Team::Blue => -(mage.mana.value as isize),
            })
            .sum();

        let pos_adv: isize = self
            .mages
            .iter()
            .map(|mage| {
                let centre_dist = &Position(mage.position.0 * 2, mage.position.1 * 2)
                    - &Position((self.board.width - 1) as i8, (self.board.height - 1) as i8);
                match mage.team {
                    Team::Red => -centre_dist.length() as isize,
                    Team::Blue => centre_dist.length() as isize,
                }
            })
            .sum();

        mana_diff * 32 + pos_adv
    }

    // function alphabeta(node, depth, α, β, maximizingPlayer) is
    // if depth = 0 or node is a terminal node then
    //     return the heuristic value of node
    // if maximizingPlayer then
    //     value := −∞
    //     for each child of node do
    //         value := max(value, alphabeta(child, depth − 1, α, β, FALSE))
    //         α := max(α, value)
    //         if value ≥ β then
    //             break (* β cutoff *)
    //     return value
    // else
    //     value := +∞
    //     for each child of node do
    //         value := min(value, alphabeta(child, depth − 1, α, β, TRUE))
    //         β := min(β, value)
    //         if value ≤ α then
    //             break (* α cutoff *)
    //     return value

    pub fn take_move(&mut self, from: Position, to: Position) -> Option<Vec<Position>> {
        if let Some(mage) = self.live_occupant(&from) {
            if mage.team == self.turn_for() {
                let available_moves = self.available_moves(mage);
                let potential_move = available_moves.iter().find(|(position, _)| *position == to);

                let mage = self.live_occupant_mut(&from).unwrap();

                if let Some((to, _)) = potential_move {
                    mage.position = *to;
                    self.turns.push(Turn(from, *to));
                    return Some(self.attack(*to));
                }
            }
        }

        None
    }

    pub fn attack(&mut self, at: Position) -> Vec<Position> {
        let mut hits = Vec::new();

        if let Some(active_mage) = self.live_occupant(&at) {
            let targets = self.targets(active_mage, at);

            for (is_enemy, tile) in targets {
                if is_enemy {
                    self.live_occupant_mut(&tile).unwrap().mana -= 1;
                    hits.push(tile);
                }
            }
        }

        hits
    }

    pub fn targets(&self, mage: &Mage, at: Position) -> Vec<(bool, Position)> {
        let mut moves = Vec::with_capacity(mage.spell.pattern.len());

        for dir in &mage.spell.pattern {
            let position = &at + dir;

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
