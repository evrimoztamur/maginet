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
    pub fn wrap(&self, xmax: i8, ymax: i8) -> Position {
        Position(self.0.rem_zero(xmax), self.1.rem_zero(ymax))
    }

    pub fn length(&self) -> isize {
        (self.0.pow(2) + self.1.pow(2)) as isize
    }
}

pub trait RemZero<Rhs = Self> {
    type Output;

    fn rem_zero(self, rhs: Rhs) -> Self::Output;
}

impl RemZero for i8 {
    type Output = i8;

    fn rem_zero(self, rhs: Self) -> Self::Output {
        ((self % rhs) + rhs) % rhs
    }
}
