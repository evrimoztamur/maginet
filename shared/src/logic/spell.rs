use serde::{Deserialize, Serialize};

use crate::{MageSort, Position};

/// A [`Spell`] is currently simply a container for a specific [`crate::Mage`]'s pattern.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Spell {
    /// A [`Vec`] of *relative* [`crate::Position`]s that the mage will attack upon move.
    pub pattern: Vec<Position>,
}

impl Spell {
    /// Instantiates the [`Spell`] `struct`.
    pub fn new(pattern: Vec<Position>) -> Spell {
        Spell { pattern }
    }

    /// Maps a spell index to a [`Spell`]. Exact choices may vary.
    pub fn select(mage_sort: MageSort) -> Spell {
        match mage_sort {
            MageSort::Diamond => Self::diamond_missile(),
            MageSort::Cross => Self::cross_missile(),
            MageSort::Knight => Self::knight_missile(),
            MageSort::Spike => Self::spike_missile(),
            MageSort::Plus => Self::plus_missile(),
        }
    }

    fn plus_missile() -> Spell {
        Self::new(vec![
            Position(-2, 0),
            Position(-1, 0),
            Position(1, 0),
            Position(2, 0),
            Position(0, -2),
            Position(0, -1),
            Position(0, 1),
            Position(0, 2),
        ])
    }

    fn diamond_missile() -> Spell {
        Self::new(vec![
            Position(-2, 0),
            Position(-1, -1),
            Position(0, -2),
            Position(1, -1),
            Position(2, 0),
            Position(1, 1),
            Position(0, 2),
            Position(-1, 1),
        ])
    }

    fn cross_missile() -> Spell {
        Self::new(vec![
            Position(-2, -2),
            Position(-2, 2),
            Position(2, -2),
            Position(2, 2),
            Position(-1, -1),
            Position(-1, 1),
            Position(1, -1),
            Position(1, 1),
        ])
    }

    fn knight_missile() -> Spell {
        Self::new(vec![
            Position(-2, -1),
            Position(-1, -2),
            Position(1, 2),
            Position(2, 1),
            Position(1, -2),
            Position(2, -1),
            Position(-2, 1),
            Position(-1, 2),
        ])
    }
    fn spike_missile() -> Spell {
        Self::new(vec![
            Position(-2, -2),
            Position(-2, 2),
            Position(2, -2),
            Position(2, 2),
            Position(-1, 0),
            Position(0, -1),
            Position(1, 0),
            Position(0, 1),
        ])
    }
}
