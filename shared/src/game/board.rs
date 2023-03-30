use serde::{Deserialize, Serialize};

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
}
