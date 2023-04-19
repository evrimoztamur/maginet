use serde::{Deserialize, Serialize};

use crate::{Mana, Position, Spell, Team};

const DIAGONALS_THRESHOLD: u8 = 1;

/// A [`MageSort`] is the distinct type of the mage, determining its visual appearance and spell.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MageSort {
    /// A simple mage who attacks with a diamond pattern.
    Diamond,
    /// A simple mage who attacks with an X pattern.
    Cross,
    /// A simple mage who attacks with a chess knight pattern.
    Knight,
    /// A simple mage who attacks with a funky pattern.
    Spike,
    /// A simple mage who attacks with a + pattern.
    Plus,
}

impl From<usize> for MageSort {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Diamond,
            1 => Self::Cross,
            2 => Self::Knight,
            3 => Self::Spike,
            _ => Self::Plus,
        }
    }
}

/// A [`Mage`] is the playable unit on the [`crate::Board`].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mage {
    /// A unique identifier to determine the selected [`Mage`] by the front-end and also its visual appearence.
    pub index: usize,
    /// [`Position`] of the mage on the board.
    pub position: Position,
    /// [`Sort`] of the mage.
    pub sort: MageSort,
    /// [`Mana`] of the mage.
    pub mana: Mana,
    /// [`Team`] of the mage.
    pub team: Team,
    /// [`Spell`] of the mage.
    pub spell: Spell,
}

impl Mage {
    /// Instantiates the [`Mage`] `struct` using [`crate::Mana::with_max()`]
    pub fn new(index: usize, team: Team, sort: MageSort, position: Position) -> Mage {
        Mage {
            index,
            position,
            sort,
            team,
            mana: Mana::select(sort),
            spell: Spell::select(sort),
        }
    }

    /// Determines if the [`Mage`] is alive. A *sleeping* mage has no mana left (`== 0`).
    pub fn is_alive(&self) -> bool {
        self.mana > 0
    }

    /// Determines if the [`Mage`] has access to the diagonal moves.
    pub fn has_diagonals(&self) -> bool {
        self.mana <= DIAGONALS_THRESHOLD
    }
}
