use std::collections::BTreeMap;

use data_encoding::Encoding;
use data_encoding_macro::new_encoding;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    vecmap, Board, Game, Mage, Mages, Position, PowerUp, PowerUpEntry, Team, Turn, TurnLeaf,
};

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
    /// Level's power-ups as as [`BTreeMap<Position, PowerUp>`].
    #[serde(with = "vecmap")]
    pub powerups: BTreeMap<Position, PowerUp>,
    /// Level's starting [`Team`].
    pub starting_team: Team,
}

impl Level {
    /// Strict, fallible parser for external level codes. Legacy decoding remains unchanged.
    pub fn parse_code(code: &str) -> Result<Self, String> {
        let bytes = BASE32
            .decode(code.as_bytes())
            .map_err(|e| format!("invalid lowercase Crockford base32 level code: {e}"))?;
        if bytes.len() < 3 {
            return Err("level code is truncated (missing header)".into());
        }
        let width = (bytes[0] >> 5) + 1;
        let height = ((bytes[0] >> 2) & 7) + 1;
        if width < 3 || height < 3 || bytes[0] & 3 > 1 {
            return Err(
                "board must be 3–8 tiles wide/high and starting team must be Red or Blue".into(),
            );
        }
        let props = 2 + bytes[1] as usize * 3;
        if bytes.len() <= props || bytes.len() != props + 1 + bytes[props] as usize * 2 {
            return Err("level code has truncated records or trailing data".into());
        }
        let mut occupied = std::collections::HashSet::new();
        for mage in bytes[2..props].chunks_exact(3) {
            let pos = (mage[0] >> 5, (mage[0] >> 2) & 7);
            if pos.0 >= width
                || pos.1 >= height
                || mage[0] & 3 > 1
                || mage[1] > 4
                || mage[2] >> 4 > mage[2] & 15
                || !occupied.insert(pos)
            {
                return Err(
                    "invalid mage position, team, type, mana, or duplicate position".into(),
                );
            }
        }
        let mut positions = std::collections::HashSet::new();
        for prop in bytes[props + 1..].chunks_exact(2) {
            let pos = (prop[0] >> 5, (prop[0] >> 2) & 7);
            if pos.0 >= width
                || pos.1 >= height
                || prop[0] & 3 != 0
                || prop[1] > 5
                || !positions.insert(pos)
                || occupied.contains(&pos)
            {
                return Err("invalid or overlapping powerup position/type".into());
            }
        }
        let level: Self = bytes.into();
        Ok(level)
    }

    /// Instantiates a new [`Level`].
    pub fn new(
        board: Board,
        mut mages: Vec<Mage>,
        powerups: BTreeMap<Position, PowerUp>,
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
        Level::new(
            Board::default(),
            mages,
            BTreeMap::default(),
            Team::default(),
        )
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

    /// Checks if a tile is blocked.
    pub fn is_blocked(&self, position: &Position) -> bool {
        if self.mages.occupied(position) {
            true
        } else {
            matches!(self.powerups.get(position), Some(PowerUp::Boulder(_)))
        }
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

        let powerups: BTreeMap<Position, PowerUp> = powerup_entries.iter().cloned().collect();

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
