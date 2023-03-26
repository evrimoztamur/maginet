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
        let mut wrapped = self.clone();

        while wrapped.0 < 0 {
            wrapped.0 += xmax;
        }

        while wrapped.0 >= xmax {
            wrapped.0 -= xmax;
        }

        while wrapped.1 < 0 {
            wrapped.1 += ymax;
        }

        while wrapped.1 >= ymax {
            wrapped.1 -= ymax;
        }

        wrapped
    }

    pub fn length(&self) -> isize {
        (self.0.pow(2) + self.1.pow(2)) as isize
    }
}
