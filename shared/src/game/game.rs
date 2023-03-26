use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use serde::{Deserialize, Serialize};

use crate::{Board, Mage, Position, Spell, Team, Turn};

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

            let game = Game {
                board,
                mages,
                turns,
            };

            Ok(game)
        }
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

    pub fn available_moves(&self, mage: &Mage) -> Vec<(Position, Position, bool)> {
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
            let position = position.wrap(self.board.width as i8, self.board.height as i8);

            if !self.occupied(&position) && !(overdrive && !mage.has_diagonals()) {
                moves.push((position, dir, overdrive));
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
                    .map(|(to, _, _)| Turn(mage.position, *to))
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
                Team::Red => mage.mana.0 as isize,
                Team::Blue => -(mage.mana.0 as isize),
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

        mana_diff.pow(2) * mana_diff.signum() * 10 + pos_adv + self.turns() as isize
    }

    pub fn best_turn(&self, seed: u64) -> (Turn, isize) {
        let alive_mages = self.mages.iter().filter(|mage| mage.is_alive()).count();
        self.alphabeta(
            4 + (2 - alive_mages / 3),
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

    pub fn take_move(&mut self, from: Position, to: Position) -> Option<Vec<Position>> {
        if let Some(mage) = self.live_occupant(&from) {
            if mage.team == self.turn_for() {
                let available_moves = self.available_moves(mage);
                let potential_move = available_moves
                    .iter()
                    .find(|(position, _, _)| *position == to);

                let mage = self.live_occupant_mut(&from).unwrap();

                if let Some((to, _, _)) = potential_move {
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
            let position = position.wrap(self.board.width as i8, self.board.height as i8);

            moves.push((
                self.live_occupied_by(&position, mage.team.enemy()),
                position,
            ));
        }

        return moves;
    }
}
