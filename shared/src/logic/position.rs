use std::ops::{Add, Sub};

use serde::{Deserialize, Serialize};

use crate::{Board, Team};

/// Reference to a position on the game board.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Serialize, Deserialize, Hash, Default)]
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
    /// Wraps the position from `(0, 0)` to `(xmax - 1, ymax - 1)` in order to ignore the edges of the board.
    /// Both negative and positive positions will be wrapped around as if it was on an infinite grid.
    pub fn wrap(&self, xmax: i8, ymax: i8) -> Position {
        Position(self.0.rem_zero(xmax), self.1.rem_zero(ymax))
    }

    /// Squared length of the [`Position`] as a vector.
    pub fn length(&self) -> isize {
        // (self.0.pow(2) + self.1.pow(2)) as isize
        (self.0.abs() + self.1.abs()) as isize
    }

    /// Rotates a [`Position`] 180 degrees.
    pub fn rotate(&self, board: &Board) -> Position {
        Position(
            board.width as i8 - self.0 - 1,
            board.height as i8 - self.1 - 1,
        )
    }

    /// Aligns a [`Position`] into the team's perspective.
    pub fn align(&self, board: &Board, team: Team) -> Position {
        match team {
            Team::Red => self.rotate(board),
            Team::Blue => *self,
        }
    }
}

/// This trait is for wrapping a sized integer such that it always lies between 0 and an another integer.
/// For example, 9 wrapped by 4 would be 1.
pub trait RemZero<Rhs = Self> {
    /// The exact formula is `((x % t) + t) % t)`. First remainder plus `t` yields a value `x'` such that it lies in `0..2t`. Second remainder brings it into `0..t`.
    fn rem_zero(self, rhs: Rhs) -> Self;
}

impl RemZero for i8 {
    fn rem_zero(self, rhs: Self) -> Self {
        ((self % rhs) + rhs) % rhs
    }
}
