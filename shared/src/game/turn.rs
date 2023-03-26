use serde::{Deserialize, Serialize};

use crate::Position;
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Turn(pub Position, pub Position);

impl Turn {
    pub fn sentinel() -> Turn {
        Turn(Position(0, 0), Position(0, 0))
    }
}
