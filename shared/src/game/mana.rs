use std::ops::{Add, AddAssign, Sub, SubAssign};

use serde::{Deserialize, Serialize};

use crate::MageSort;

const DEFAULT_MANA: u8 = 4;

/// Mana is a `struct` which contains the current mana level for a specific wizard. It stores the current and maximum values.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mana(pub u8, pub u8);

impl Mana {
    /// Helper function to instantiate a [`Mana`] where current value equals the maximum.
    pub fn with_max(max_mana: u8) -> Mana {
        Mana(max_mana, max_mana)
    }

    /// Selects the appropriate [`Mana`] for a given [`MageSort`].
    pub fn select(_mage_sort: MageSort) -> Mana {
        Mana::with_max(DEFAULT_MANA)
    }
}

impl Add<u8> for Mana {
    type Output = Mana;

    fn add(self, rhs: u8) -> Self::Output {
        Mana(self.0.saturating_add(rhs).min(self.1), self.1)
    }
}

impl AddAssign<u8> for Mana {
    fn add_assign(&mut self, rhs: u8) {
        self.0 = self.0.saturating_add(rhs).min(self.1)
    }
}

impl Sub<u8> for Mana {
    type Output = Mana;

    fn sub(self, rhs: u8) -> Self::Output {
        Mana(self.0.saturating_sub(rhs), self.1)
    }
}

impl SubAssign<u8> for Mana {
    fn sub_assign(&mut self, rhs: u8) {
        self.0 = self.0.saturating_sub(rhs);
    }
}

impl PartialEq<u8> for Mana {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for Mana {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl From<&Mana> for u8 {
    fn from(mana: &Mana) -> Self {
        (mana.0  & 0b1111) << 4 | mana.1 & 0b1111
    }
}

impl From<u8> for Mana {
    fn from(value: u8) -> Self {
        Mana((value >> 4) & 0b1111, value & 0b1111)
    }
}
