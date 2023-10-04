use serde::{Deserialize, Serialize};

/// A [`PowerUp`] is a the distinct type of the powerup.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PowerUp {
    /// Turns the mage into defensive mode.
    Shield,
    /// Zaps out a plus-shaped beam, damaging all in its way.
    Beam,
    /// Gives a mage the ability to move diagonals.
    Diagonal,
}
impl PowerUp {
    /// Returns next [`PowerUp`] in order.
    pub fn next(&self) -> PowerUp {
        match self {
            PowerUp::Shield => PowerUp::Beam,
            PowerUp::Beam => PowerUp::Diagonal,
            PowerUp::Diagonal => PowerUp::Shield,
        }
    }
    /// Returns next [`PowerUp`] in order.
    pub fn previous(&self) -> PowerUp {
        match self {
            PowerUp::Shield => PowerUp::Diagonal,
            PowerUp::Beam => PowerUp::Shield,
            PowerUp::Diagonal => PowerUp::Beam,
        }
    }
}

impl From<usize> for PowerUp {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Shield,
            1 => Self::Beam,
            2 => Self::Diagonal,
            _ => Self::Diagonal,
        }
    }
}

impl Default for PowerUp {
    fn default() -> Self {
        Self::Diagonal
    }
}
