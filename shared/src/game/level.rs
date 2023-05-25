use serde::{Deserialize, Serialize};

use crate::{Board, Game, Mage, Team, Turn};

/// [`Level`] is the builder for a [`Game`] instance.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Level {
    /// Level's [`Board`].
    pub board: Board,
    /// Level's mages as a [`Vec<Board>`].
    pub mages: Vec<Mage>,
    /// Level's starting [`Team`].
    pub starting_team: Team,
}

impl Level {
    /// Instantiates a new [`Level`].
    pub fn new(board: Board, mages: Vec<Mage>, starting_team: Team) -> Level {
        Level {
            board,
            mages,
            starting_team,
        }
    }

    /// Simulated `n` games and yields results.
    pub fn simulate(level: Level, n: usize, seed: u64) -> Vec<Game> {
        (0..n)
            .into_iter()
            .map(|m| {
                let mut game = Game::new(&level).unwrap();

                for i in 0..50 {
                    if let Some((Turn(from, to), _)) = game.best_turn(seed + m as u64 + i as u64) {
                        game.take_move(from, to);

                        if game.result().is_some() {
                            break;
                        }
                    }
                }

                game
            })
            .collect()
    }
}
