use data_encoding::Encoding;
use data_encoding_macro::new_encoding;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{Board, Game, Mage, Team, Turn, TurnLeaf};

/// Base 32 (Crockford) encoding for levels.
pub const BASE32: Encoding = new_encoding! {
    symbols: "0123456789abcdefghjkmnpqrstvwxyz",
};

/// [`Level`] is the builder for a [`Game`] instance.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Level {
    /// Level's [`Board`].
    pub board: Board,
    /// Level's mages as a [`Vec<Board>`].
    pub mages: Vec<Mage>,
    /// Number of mages.
    pub mage_index: usize,
    /// Level's starting [`Team`].
    pub starting_team: Team,
}

impl Level {
    /// Instantiates a new [`Level`].
    pub fn new(board: Board, mut mages: Vec<Mage>, starting_team: Team) -> Level {
        mages = mages
            .iter_mut()
            .enumerate()
            .map(|(i, mage)| {
                mage.index = i;
                mage.clone()
            })
            .collect();
        Level {
            board,
            mage_index: mages.len(),
            mages,
            starting_team,
        }
    }

    /// Simulated `n` games and yields results.
    pub fn simulate(level: &Level, n: usize, seed: u64) -> Vec<Game> {
        (0..n)
            .into_iter()
            .map(|m| {
                let mut game = Game::new(&level).unwrap();

                for i in 0..50 {
                    if let Some(TurnLeaf(Turn(from, to), _)) =
                        game.best_turn(5, seed + m as u64 + i as u64)
                    {
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

    /// Converts the level to a Base 32 code string.
    pub fn as_code(&self) -> String {
        let encoded_level: Vec<u8> = self.into();
        BASE32.encode(encoded_level.as_slice())
    }
}

impl Into<Vec<u8>> for &Level {
    fn into(self) -> Vec<u8> {
        let board_width = self.board.width as u8 - 1;
        let board_height = self.board.height as u8 - 1;

        let starting_team = self.starting_team as u8;

        let mut result = Vec::new();

        let board_byte =
            ((board_width & 0b111) << 5) | ((board_height & 0b111) << 2) | (starting_team & 0b11);

        result.push(board_byte);

        for mage in &self.mages {
            let mut mage_bytes: Vec<u8> = mage.into();
            result.append(&mut mage_bytes);
        }

        result
    }
}

impl From<Vec<u8>> for Level {
    fn from(value: Vec<u8>) -> Self {
        if value.len() % 3 == 1 {
            let board_byte = value[0];

            let board_width = ((board_byte >> 5) & 0b111) + 1;
            let board_height = ((board_byte >> 2) & 0b111) + 1;

            let board = Board::new(board_width.into(), board_height.into()).unwrap();

            let starting_team = Team::from_index((board_byte & 0b11) as usize);

            let mages: Vec<Mage> = value
                .iter()
                .skip(1)
                .chunks(3)
                .into_iter()
                .map(|chunk| chunk.cloned().collect::<Vec<u8>>().into())
                .collect();

            Level::new(board, mages, starting_team)
        } else {
            Level::default()
        }
    }
}

impl From<&str> for Level {
    fn from(value: &str) -> Self {
        if let Ok(decoded) = BASE32.decode(value.as_bytes()) {
            decoded.into()
        } else {
            Level::default()
        }
    }
}
