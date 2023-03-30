use serde::{Deserialize, Serialize};

use crate::{Mana, Position, Spell, Team};

/// A [`Mage`] is the playable unit on the [`crate::Board`].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mage {
    /// A unique identifier to determine the selected [`Mage`] by the front-end and also its visual appearence.
    pub index: usize,
    /// [`Position`] of the mage on the board.
    pub position: Position,
    /// [`Mana`] of the mage.
    pub mana: Mana,
    /// [`Team`] of the mage.
    pub team: Team,
    /// [`Spell`] of the mage.
    pub spell: Spell,
}

impl Mage {
    /// Instantiates the [`Mage`] `struct` using [`crate::Mana::with_max()`]
    pub fn new(index: usize, position: Position, max_mana: u8, team: Team, spell: Spell) -> Mage {
        Mage {
            index,
            position,
            team,
            mana: Mana::with_max(max_mana),
            spell,
        }
    }

    /// Determines if the [`Mage`] is alive. A *sleeping* mage has no mana left (`== 0`).
    pub fn is_alive(&self) -> bool {
        self.mana > 0
    }

    /// Determines if the [`Mage`] has access to the diagonal moves.
    pub fn has_diagonals(&self) -> bool {
        self.mana <= 2
    }
}
