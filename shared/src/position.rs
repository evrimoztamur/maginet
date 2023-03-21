use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize, Hash)]
pub struct Position(pub i8, pub i8);

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Position {
    pub fn within_bounds(&self, xmin: i8, xmax: i8, ymin: i8, ymax: i8) -> bool {
        self.0 >= xmin && self.0 < xmax && self.1 >= ymin && self.1 < ymax
    }

    pub fn length(&self) -> isize {
        (self.0.pow(2) + self.1.pow(2)) as isize
    }
}
