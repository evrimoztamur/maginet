use std::collections::HashMap;

use data_encoding::Encoding;
use data_encoding_macro::new_encoding;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{vecmap, Board, Game, Mage, Position, PowerUp, PowerUpEntry, Team, Turn, TurnLeaf};

/// Base 32 (Crockford) encoding for levels.
pub const BASE32: Encoding = new_encoding! {
    symbols: "0123456789abcdefghjkmnpqrstvwxyz",
};

/// [`Level`] is the builder for a [`Game`] instance.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Level {
    /// Level's [`Board`].
    pub board: Board,
    /// Level's mages as a [`Vec<Board>`].
    pub mages: Vec<Mage>,
    /// Number of mages.
    pub mage_index: usize,
    /// Level's power-ups as as [`HashMap<Position, PowerUp>`].
    #[serde(with = "vecmap")]
    pub powerups: HashMap<Position, PowerUp>,
    /// Level's starting [`Team`].
    pub starting_team: Team,
}

impl Level {
    /// Instantiates a new [`Level`].
    pub fn new(
        board: Board,
        mut mages: Vec<Mage>,
        powerups: HashMap<Position, PowerUp>,
        starting_team: Team,
    ) -> Level {
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
            powerups,
            starting_team,
        }
    }

    /// Instantiates a new [`Level`] with default parameters but provided mages.
    pub fn default_with_mages(mages: Vec<Mage>) -> Level {
        Level::new(Board::default(), mages, HashMap::default(), Team::default())
    }

    /// Simulated `n` games and yields results.
    pub fn simulate(level: &Level, n: usize, seed: u64) -> Vec<Game> {
        (0..n)
            .map(|m| {
                let mut game = Game::new(level, true).unwrap();

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

impl From<&Level> for Vec<u8> {
    fn from(level: &Level) -> Self {
        let board_width = level.board.width as u8 - 1;
        let board_height = level.board.height as u8 - 1;

        let starting_team = level.starting_team as u8;

        let mut result = Vec::new();

        let board_byte =
            ((board_width & 0b111) << 5) | ((board_height & 0b111) << 2) | (starting_team & 0b11);

        result.push(board_byte);

        result.push(level.mages.len() as u8);

        for mage in &level.mages {
            let mut mage_bytes: Vec<u8> = mage.into();
            result.append(&mut mage_bytes);
        }

        result.push(level.powerups.len() as u8);

        for (position, powerup) in &level.powerups {
            let mut prop_bytes: Vec<u8> = (&PowerUpEntry(*position, *powerup)).into();
            result.append(&mut prop_bytes);
        }

        result
    }
}

impl From<Vec<u8>> for Level {
    fn from(value: Vec<u8>) -> Self {
        let board_byte = value[0];

        let board_width = ((board_byte >> 5) & 0b111) + 1;
        let board_height = ((board_byte >> 2) & 0b111) + 1;

        let board = Board::new(board_width.into(), board_height.into()).unwrap();

        let starting_team = Team::from_index((board_byte & 0b11) as usize);

        let num_mages = value[1];
        let mages: Vec<Mage> = value
            .iter()
            .skip(2)
            .take(num_mages as usize * 3)
            .chunks(3)
            .into_iter()
            .map(|chunk| chunk.cloned().collect::<Vec<u8>>().into())
            .collect();

        let num_props = value[2 + num_mages as usize * 3];
        let powerup_entries: Vec<PowerUpEntry> = value
            .iter()
            .skip(3 + num_mages as usize * 3)
            .take(num_props as usize * 2)
            .chunks(2)
            .into_iter()
            .map(|chunk| chunk.cloned().collect::<Vec<u8>>().into())
            .collect();

        let powerups: HashMap<Position, PowerUp> = powerup_entries.iter().cloned().collect();

        Level::new(board, mages, powerups, starting_team)
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

impl Clone for Level {
    fn clone(&self) -> Self {
        let mages = self
            .mages
            .iter()
            .enumerate()
            .map(|(i, mage)| {
                let mut mage = mage.clone();
                mage.index = i;
                mage
            })
            .collect();

        Self {
            board: self.board.clone(),
            mages,
            mage_index: self.mage_index,
            powerups: self.powerups.clone(),
            starting_team: self.starting_team,
        }
    }
}
