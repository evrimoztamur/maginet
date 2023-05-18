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

impl MageSort {
    /// Rotates the [`MageSort`] left.
    pub fn previous(&self) -> MageSort {
        match self {
            MageSort::Diamond => MageSort::Plus,
            MageSort::Cross => MageSort::Diamond,
            MageSort::Knight => MageSort::Cross,
            MageSort::Spike => MageSort::Knight,
            MageSort::Plus => MageSort::Spike,
        }
    }

    /// Rotates the [`MageSort`] right.
    pub fn next(&self) -> MageSort {
        match self {
            MageSort::Diamond => MageSort::Cross,
            MageSort::Cross => MageSort::Knight,
            MageSort::Knight => MageSort::Spike,
            MageSort::Spike => MageSort::Plus,
            MageSort::Plus => MageSort::Diamond,
        }
    }
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

impl PartialEq for Mage {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
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

/// Trait for helper functions that interface with `Vec<Mages>`.
pub trait Mages {
    /// Returns a reference to a [`Mage`] at a certain [`Position`].
    fn occupant(&self, position: &Position) -> Option<&Mage>;
    /// Returns a mutable reference to a [`Mage`] at a certain [`Position`].
    fn occupant_mut(&mut self, position: &Position) -> Option<&mut Mage>;
    /// Determines whether or not a move can be made to the [`Position`].
    fn occupied(&self, position: &Position) -> bool;
    /// Returns a reference to a [`Mage`] at a certain [`Position`] if the mage is alive.
    fn live_occupant(&self, position: &Position) -> Option<&Mage>;
    /// Returns a mutable reference to a [`Mage`] at a certain [`Position`] if the mage is alive.
    fn live_occupant_mut(&mut self, position: &Position) -> Option<&mut Mage>;
    /// Determines whether or not a [`Position`] is occupied by a live [`Mage`].
    fn live_occupied(&self, position: &Position) -> bool;
    /// Determines whether or not a [`Position`] is occupied by a live [`Mage`] of a certain [`Team`].
    fn live_occupied_by(&self, position: &Position, team: Team) -> bool;
}

impl Mages for Vec<Mage> {
    fn occupant(&self, position: &Position) -> Option<&Mage> {
        for mage in self.iter() {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    fn occupant_mut(&mut self, position: &Position) -> Option<&mut Mage> {
        for mage in self.iter_mut() {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    fn occupied(&self, position: &Position) -> bool {
        self.occupant(position).is_some()
    }

    fn live_occupant(&self, position: &Position) -> Option<&Mage> {
        for mage in self.iter().filter(|mage| mage.is_alive()) {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    fn live_occupant_mut(&mut self, position: &Position) -> Option<&mut Mage> {
        for mage in self.iter_mut().filter(|mage| mage.is_alive()) {
            if mage.position == *position {
                return Some(mage);
            }
        }

        None
    }

    fn live_occupied(&self, position: &Position) -> bool {
        self.live_occupant(position).is_some()
    }

    fn live_occupied_by(&self, position: &Position, team: Team) -> bool {
        if let Some(occupant) = self.live_occupant(position) {
            occupant.team == team
        } else {
            false
        }
    }
}
