use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::Position;

/// Style for a [`PowerUp::Boulder`]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum BoulderStyle {
    /// Default rock block.
    Rock,
    /// Pedestal block.
    Pedestal,
    /// Tentacle lump.
    Tentacle,
}

/// A [`PowerUp`] is a the distinct type of the powerup.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PowerUp {
    /// Turns the mage into defensive mode.
    Shield,
    /// Zaps out a plus-shaped beam, damaging all in its way.
    Beam,
    /// Gives a mage the ability to move diagonals.
    Diagonal,
    /// Blocks off the mages' movement.
    Boulder(BoulderStyle),
}

impl PowerUp {
    /// Returns next [`PowerUp`] in order.
    pub fn next(&self) -> PowerUp {
        match self {
            PowerUp::Shield => PowerUp::Beam,
            PowerUp::Beam => PowerUp::Diagonal,
            PowerUp::Diagonal => PowerUp::Boulder(BoulderStyle::Rock),
            PowerUp::Boulder(BoulderStyle::Rock) => PowerUp::Boulder(BoulderStyle::Pedestal),
            PowerUp::Boulder(BoulderStyle::Pedestal) => PowerUp::Boulder(BoulderStyle::Tentacle),
            PowerUp::Boulder(BoulderStyle::Tentacle) => PowerUp::Shield,
        }
    }
    /// Returns next [`PowerUp`] in order.
    pub fn previous(&self) -> PowerUp {
        match self {
            PowerUp::Shield => PowerUp::Boulder(BoulderStyle::Tentacle),
            PowerUp::Beam => PowerUp::Shield,
            PowerUp::Diagonal => PowerUp::Beam,
            PowerUp::Boulder(BoulderStyle::Rock) => PowerUp::Diagonal,
            PowerUp::Boulder(BoulderStyle::Pedestal) => PowerUp::Boulder(BoulderStyle::Rock),
            PowerUp::Boulder(BoulderStyle::Tentacle) => PowerUp::Boulder(BoulderStyle::Pedestal),
        }
    }
}

impl From<u8> for PowerUp {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Shield,
            1 => Self::Beam,
            2 => Self::Diagonal,
            3 => Self::Boulder(BoulderStyle::Rock),
            4 => Self::Boulder(BoulderStyle::Pedestal),
            5 => Self::Boulder(BoulderStyle::Tentacle),
            _ => Self::Boulder(BoulderStyle::Rock),
        }
    }
}

impl From<PowerUp> for u8 {
    fn from(value: PowerUp) -> Self {
        match value {
            PowerUp::Shield => 0,
            PowerUp::Beam => 1,
            PowerUp::Diagonal => 2,
            PowerUp::Boulder(BoulderStyle::Rock) => 3,
            PowerUp::Boulder(BoulderStyle::Pedestal) => 4,
            PowerUp::Boulder(BoulderStyle::Tentacle) => 5,
        }
    }
}

impl Default for PowerUp {
    fn default() -> Self {
        Self::Diagonal
    }
}

#[derive(Clone)]
/// Cloneablepower
pub struct PowerUpEntry(pub Position, pub PowerUp);

impl From<&PowerUpEntry> for Vec<u8> {
    fn from(PowerUpEntry(position, powerup): &PowerUpEntry) -> Self {
        let position_x = position.0 as u8;
        let position_y = position.1 as u8;
        let sort = u8::from(*powerup);

        vec![(position_x & 0b111) << 5 | (position_y & 0b111) << 2, sort]
    }
}

impl From<Vec<u8>> for PowerUpEntry {
    fn from(value: Vec<u8>) -> Self {
        if value.len() == 2 {
            let pos_team_byte = value[0];
            let position_x = (pos_team_byte >> 5) & 0b111;
            let position_y = (pos_team_byte >> 2) & 0b111;
            let position = Position(position_x as i8, position_y as i8);

            let powerup = value[1].into();

            PowerUpEntry(position, powerup)
        } else {
            PowerUpEntry(Position(0, 0), PowerUp::default())
        }
    }
}

impl std::iter::FromIterator<PowerUpEntry> for BTreeMap<Position, PowerUp> {
    fn from_iter<T: IntoIterator<Item = PowerUpEntry>>(iter: T) -> Self {
        iter.into_iter()
            .map(|PowerUpEntry(position, powerup)| (position, powerup))
            .collect()
    }
}
