use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use serde::{Deserialize, Serialize};

use crate::{Board, Level, Mage, Mages, Position, Team, Turn};

/// Result of a game.
pub enum GameResult {
    /// Win for a [`Team`]
    Win(Team),
    /// Stalemate
    Stalemate,
}

/// A [`Game`] contains all the information to represent a deterministically replicable state of the game.
/// From a given [`Board`] and list of [`Turn`], the exact same [`Game`] must be reached.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    board: Board,
    mages: Vec<Mage>,
    turns: Vec<Turn>,
    starting_team: Team,
    last_nominal: usize,
}

impl Game {
    /// Instantiates the [`Game`] `struct` with a given board size (always 8-by-8) and number of mages (always 4)]
    pub fn new(level: &Level) -> Result<Game, &'static str> {
        let board = Board::new(level.board.width, level.board.height)?;
        let mages = level.mages.clone();
        let starting_team = level.starting_team;
        let turns = Vec::new();
        let last_nominal = 0;

        let game = Game {
            board,
            mages,
            turns,
            starting_team,
            last_nominal,
        };

        Ok(game)
    }

    /// Determines if the game is stalemated.
    fn stalemate(&self) -> bool {
        self.last_nominal + 8 < self.turns() && self.turns() > 24
    }

    /// Determines if the game is finished.
    pub fn result(&self) -> Option<GameResult> {
        if self.all_available_turns(self.turn_for()).is_empty() || self.stalemate() {
            let mana_diff: isize = self
                .mages
                .iter()
                .map(|mage| match mage.team {
                    Team::Red => mage.mana.0 as isize,
                    Team::Blue => -(mage.mana.0 as isize),
                })
                .sum();

            if mana_diff == 0 {
                Some(GameResult::Stalemate)
            } else if mana_diff > 0 {
                Some(GameResult::Win(Team::Red))
            } else {
                Some(GameResult::Win(Team::Blue))
            }
        } else {
            None
        }
    }

    /// Returns a list of [`Turn`]s skipping the first `since` turns.
    pub fn turns_since(&self, since: usize) -> Vec<&Turn> {
        self.turns.iter().skip(since).collect()
    }

    /// Returns the latest [`Turn`].
    pub fn last_turn(&self) -> Option<Turn> {
        self.turns.last().copied()
    }

    /// Returns an iterator over all [`Mage`]s.
    pub fn iter_mages(&self) -> std::slice::Iter<Mage> {
        self.mages.iter()
    }

    /// Returns a reference to a [`Mage`] of a certain index.
    pub fn get_mage(&self, index: usize) -> Option<&Mage> {
        self.mages.get(index)
    }

    /// Returns a mutable reference to a [`Mage`] of a certain index.
    pub fn get_mage_mut(&mut self, index: usize) -> Option<&mut Mage> {
        self.mages.get_mut(index)
    }

    /// Returns the number of turns since the start of the game, starting from 0.
    pub fn turns(&self) -> usize {
        self.turns.len()
    }

    /// Returns the index for the team which will be making the next move.
    fn turn_index(&self) -> usize {
        self.turns() % 2
    }

    /// Returns the [`Team`] which will be taking their turn.
    pub fn turn_for(&self) -> Team {
        let team = if self.turn_index() == 0 {
            Team::Red
        } else {
            Team::Blue
        };

        if self.starting_team == Team::Red {
            team
        } else {
            team.enemy()
        }
    }

    /// Returns the [`Team`] which makes the first move.
    pub fn starting_team(&self) -> Team {
        self.starting_team
    }

    /// Returns a list of all available [`Position`]s a [`Mage`] can move to, including metadata on the direction and whether or not it's a diagonal.
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

        for (dir, diagonal) in DIRS {
            let position = &mage.position + &dir;
            // let position = position.wrap(self.board.width as i8, self.board.height as i8);

            if let Some(position) = self.board.validate_position(position) {
                if !self.mages.occupied(&position) && !(diagonal && !mage.has_diagonals()) {
                    moves.push((position, dir, diagonal));
                }
            }
        }

        return moves;
    }

    /// Returns a list of all available [`Turn`]s for the active team.
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

    /// Evaluates the viability of the board on a signed basis, where a positive evaluation is in favour of the red team.
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

        mana_diff.pow(2) * mana_diff.signum() * 10 + pos_adv
    }

    /// Returns the best [`Turn`] available and its evaluation.
    pub fn best_turn(&self, seed: u64) -> Option<(Turn, isize)> {
        if self.result().is_none() {
            let alive_mages = self.mages.iter().filter(|mage| mage.is_alive()).count();

            Some(self.alphabeta(
                4 + (2usize.saturating_sub(alive_mages) / 3),
                isize::MIN,
                isize::MAX,
                &mut ChaCha8Rng::seed_from_u64(seed),
            ))
        } else {
            None
        }
    }

    /// Returns the best turn based on the evaluation function and [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning).
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
                self.evaluate() + (rng.next_u64() % 8) as isize,
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

    /// Executes a [`Turn`], modifying the game state.
    pub fn take_move(&mut self, from: Position, to: Position) -> Option<Vec<Position>> {
        if self.result().is_none() {
            if let Some(mage) = self.mages.live_occupant(&from) {
                if mage.team == self.turn_for() {
                    let available_moves = self.available_moves(mage);
                    let potential_move = available_moves
                        .iter()
                        .find(|(position, _, _)| *position == to);

                    let mage = self.mages.live_occupant_mut(&from).unwrap();

                    if let Some((to, _, _)) = potential_move {
                        mage.position = *to;

                        let attacks = self.attack(*to);

                        if !attacks.is_empty() {
                            self.last_nominal = self.turns();
                        }

                        self.turns.push(Turn(from, *to));

                        return Some(attacks);
                    }
                }
            }
        }

        None
    }

    /// Executes an attack on the given [`Position`].
    /// A mage must already be present on the tile.
    pub fn attack(&mut self, at: Position) -> Vec<Position> {
        let mut hits = Vec::new();

        if let Some(active_mage) = self.mages.live_occupant(&at) {
            let targets = self.targets(active_mage, at);

            for (is_enemy, tile) in targets {
                if is_enemy {
                    self.mages.live_occupant_mut(&tile).unwrap().mana -= 1;
                    hits.push(tile);
                }
            }
        }

        hits
    }

    /// Returns the list of targets a [`Mage`] can attack to on a certain [`Position`].
    pub fn targets(&self, mage: &Mage, at: Position) -> Vec<(bool, Position)> {
        mage.targets(&self.board, &at)
            .iter()
            .map(|position| {
                (
                    self.mages.live_occupied_by(&position, mage.team.enemy()),
                    *position,
                )
            })
            .collect()
    }

    /// Returns the [`Board`]'s size as an `usize` tuple.
    pub fn board_size(&self) -> (usize, usize) {
        (self.board.width, self.board.height)
    }

    /// Converts a canvas location to a board [`Position`].
    pub fn location_as_position(
        &self,
        location: (i32, i32),
        offset: (i32, i32),
        scale: (i32, i32),
    ) -> Option<Position> {
        self.board.location_as_position(location, offset, scale)
    }
}

impl Mages for Game {
    fn occupant(&self, position: &Position) -> Option<&Mage> {
        self.mages.occupant(position)
    }

    fn occupant_mut(&mut self, position: &Position) -> Option<&mut Mage> {
        self.mages.occupant_mut(position)
    }

    fn occupied(&self, position: &Position) -> bool {
        self.mages.occupied(position)
    }

    fn live_occupant(&self, position: &Position) -> Option<&Mage> {
        self.mages.live_occupant(position)
    }

    fn live_occupant_mut(&mut self, position: &Position) -> Option<&mut Mage> {
        self.mages.live_occupant_mut(position)
    }

    fn live_occupied(&self, position: &Position) -> bool {
        self.mages.live_occupied(position)
    }

    fn live_occupied_by(&self, position: &Position, team: Team) -> bool {
        self.mages.live_occupied_by(position, team)
    }
}
