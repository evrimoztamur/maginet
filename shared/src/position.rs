use std::ops::Add;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position(pub i8, pub i8);

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Position {
    pub fn within_bounds(&self, xmin: i8, xmax: i8, ymin: i8, ymax: i8) -> bool {
        self.0 >= xmin && self.0 < xmax && self.1 >= ymin && self.1 < ymax
    }
}
