use serde::{Deserialize, Serialize};

/// A [`PowerUp`] is a the distinct type of the powerup.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PowerUp {
    /// Turns the mage into defensive mode.
    Shield,
    /// Zaps out a plus-shaped beam, damaging all in its way.
    Beam,
    /// Gives a mage the ability to move diagonals.
    Diagonal,
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
