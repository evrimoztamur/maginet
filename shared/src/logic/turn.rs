use serde::{Deserialize, Serialize};

use crate::Position;

/// A turn is a pair of [`Position`]s, referring to the tile a mage is moving from and to.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Turn(pub Position, pub Position);

impl Turn {
    /// An invalid turn for the purposes of [alpha-beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning).
    pub fn sentinel() -> Turn {
        Turn(Position(0, 0), Position(0, 0))
    }
}
