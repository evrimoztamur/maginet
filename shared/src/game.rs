use std::{
    collections::HashMap,
    ops::{Add, AddAssign, Deref, Sub, SubAssign},
};

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use serde::{Deserialize, Serialize};

use crate::{vecmap, Position};

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

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Prop {
    DoubleDamage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mage {
    pub index: usize,
    pub position: Position,
    pub mana: Mana,
    pub team: Team,
    pub spell: Spell,
    prop: Option<Prop>,
}

impl Mage {
    pub fn new(index: usize, position: Position, max_mana: u8, team: Team, spell: Spell) -> Mage {
        Mage {
            index,
            position,
            team,
            mana: Mana::with_max(max_mana),
            spell,
            prop: None,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.mana.value > 0
    }

    pub fn has_diagonals(&self) -> bool {
        self.mana.value <= 2
    }

    pub fn has_double_damage(&self) -> bool {
        self.prop.is_some_and(|prop| prop == Prop::DoubleDamage)
    }

    pub fn add_prop(&mut self, prop: Prop) {
        if !self.prop.is_some() {
            self.prop = Some(prop);
        }
    }

    pub fn remove_prop(&mut self) -> Option<Prop> {
        self.prop.take()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    #[serde(with = "vecmap")]
    pub props: HashMap<Position, Prop>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Result<Board, &'static str> {
        match (width, height) {
            (width, height) if width >= 4 && width <= 8 && height >= 4 && height <= 8 => {
                Ok(Board {
                    width,
                    height,
                    props: HashMap::new(),
                })
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
                    4,
                    Team::Red,
                    Spell::select_missile(i),
                ));

                mages.push(Mage::new(
                    i * 2 + 1,
                    Position(board_width as i8 - x - 1, 1),
                    4,
                    Team::Blue,
                    Spell::select_missile(i),
                ));
            }

            let turns = Vec::new();

            let mut game = Game {
                board,
                mages,
                turns,
            };

            // game.add_prop(Position(3, 3), Prop::DoubleDamage);
            // game.add_prop(Position(4, 4), Prop::DoubleDamage);

            Ok(game)
        }
    }

    pub fn add_prop(&mut self, position: Position, prop: Prop) -> bool {
        // A prop should not be overridden, ever

        if self.board.props.contains_key(&position) {
            false
        } else {
            self.board.props.insert(position, prop);
            true
        }
    }

    pub fn take_prop_at(&mut self, position: Position) -> Option<Prop> {
        self.board.props.remove(&position)
    }

    pub fn is_prop_at(&self, position: Position) -> bool {
        self.board.props.contains_key(&position)
    }

    pub fn iter_props(&self) -> std::collections::hash_map::Iter<Position, Prop> {
        self.board.props.iter()
    }

    pub fn turns_since(&self, since: usize) -> Vec<&Turn> {
        self.turns.iter().skip(since).collect()
    }

    pub fn last_turn(&self) -> Option<Turn> {
        self.turns.last().copied()
    }

    pub fn iter_mages(&self) -> std::slice::Iter<Mage> {
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
                && !(overdrive && !mage.has_diagonals())
            {
                moves.push((position, overdrive));
            }
        }

        return moves;
    }

    pub fn all_available_turns(&self, team: Team) -> Vec<Turn> {
        self.mages
            .iter()
            .filter(|mage| mage.is_alive() && mage.team == team)
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

        mana_diff * 36 + pos_adv + self.turns() as isize
    }

    pub fn best_turn(&self, seed: u64) -> (Turn, isize) {
        let alive_mages = self.mages.iter().filter(|mage| mage.is_alive()).count();
        self.alphabeta(
            4 + (1 - alive_mages / 5),
            isize::MIN,
            isize::MAX,
            &mut ChaCha8Rng::seed_from_u64(seed),
        )
    }

    pub fn alphabeta(
        &self,
        depth: usize,
        mut alpha: isize,
        mut beta: isize,
        rng: &mut ChaCha8Rng,
    ) -> (Turn, isize) {
        if depth == 0 {
            (
                Turn::sentinel(),
                self.evaluate() + (rng.next_u64() & 0b11) as isize,
            )
        } else {
            let mut best_turn = self
                .all_available_turns(self.turn_for())
                .first()
                .copied()
                .unwrap_or(Turn::sentinel());

            match self.turn_for() {
                Team::Red => {
                    // Maximizing
                    let mut value = isize::MIN;

                    for turn in self.all_available_turns(self.turn_for()) {
                        let mut next_game = self.clone();
                        next_game.take_move(turn.0, turn.1);

                        let (_, next_value) = next_game.alphabeta(depth - 1, alpha, beta, rng);

                        if next_value > value {
                            value = value.max(next_value);
                            alpha = alpha.max(value);

                            best_turn = turn;
                        }

                        if value >= beta {
                            break;
                        }
                    }

                    (best_turn, value)
                }
                Team::Blue => {
                    // Minimizing
                    let mut value = isize::MAX;

                    for turn in self.all_available_turns(self.turn_for()) {
                        let mut next_game = self.clone();
                        next_game.take_move(turn.0, turn.1);

                        let (_, next_value) = next_game.alphabeta(depth - 1, alpha, beta, rng);

                        if next_value < value {
                            value = value.min(next_value);
                            beta = beta.min(value);

                            best_turn = turn;
                        }

                        if value <= alpha {
                            break;
                        }
                    }

                    (best_turn, value)
                }
            }
        }
    }

    pub fn take_move(
        &mut self,
        from: Position,
        to: Position,
    ) -> Option<(Option<Prop>, Vec<Position>)> {
        if let Some(mage) = self.live_occupant(&from) {
            if mage.team == self.turn_for() {
                let available_moves = self.available_moves(mage);
                let potential_move = available_moves.iter().find(|(position, _)| *position == to);

                let mage = self.live_occupant_mut(&from).unwrap();

                if let Some((to, _)) = potential_move {
                    mage.position = *to;

                    self.turns.push(Turn(from, *to));

                    return Some((self.engage_prop(*to), self.attack(*to)));
                }
            }
        }

        None
    }

    pub fn engage_prop(&mut self, at: Position) -> Option<Prop> {
        if let Some(prop) = self.take_prop_at(at) {
            if let Some(active_mage) = self.live_occupant_mut(&at) {
                active_mage.add_prop(prop);
                Some(prop)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn attack(&mut self, at: Position) -> Vec<Position> {
        let mut hits = Vec::new();

        if let Some(active_mage) = self.live_occupant(&at) {
            let targets = self.targets(active_mage, at);
            let double_damage = active_mage.has_double_damage();

            for (is_enemy, tile) in targets {
                if is_enemy {
                    self.live_occupant_mut(&tile).unwrap().mana -=
                        if double_damage { 2 } else { 1 };
                    hits.push(tile);
                }
            }

            if !hits.is_empty() {
                self.live_occupant_mut(&at).unwrap().remove_prop();
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
