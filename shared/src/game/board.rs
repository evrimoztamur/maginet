use serde::{Deserialize, Serialize};

use crate::{Mage, MageSort, Position, Team};

/// [`Board`] is a struct which currently only contains the size of the playing field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    /// Width of the board.
    pub width: usize,
    /// Height of the board.
    pub height: usize,
}

impl Board {
    /// Instantiates the [`Board`] `struct` with a certain size.
    /// Restricted to `4..=8` on both axes. Currently always 8-by-8.
    pub fn new(width: usize, height: usize) -> Result<Board, &'static str> {
        match (width, height) {
            (width, height) if width >= 4 && width <= 8 && height >= 4 && height <= 8 => {
                Ok(Board { width, height })
            }
            _ => Err("board size does not conform to limits"),
        }
    }

    /// Returns a list of [`Mage`]s already indexed, positioned on the board, and instantiated.
    pub fn place_mages(&self, team: Team, mage_sorts: Vec<MageSort>, offset: usize) -> Vec<Mage> {
        let x_offset = ((self.width - mage_sorts.len()) / 2) as i8;

        let mut mages = Vec::with_capacity(mage_sorts.len());

        for (index, mage_sort) in mage_sorts.iter().enumerate() {
            mages.push(Mage::new(
                offset + index,
                team,
                *mage_sort,
                Position(x_offset + index as i8, self.height as i8 - 2).align(self, team),
            ));
        }

        mages
    }

    /// Validates a [`Position`] and, if necessary, converts it to a valid one.
    /// Currently only confirms that the position resides on the board (albeit very inefficiently)
    pub fn validate_position(&self, position: Position) -> Option<Position> {
        // TODO restore bounds check
        if &position == &position.wrap(self.width as i8, self.height as i8) {
            Some(position)
        } else {
            None
        }
    }
}
