use serde::{Deserialize, Serialize};

use crate::{Mage, MageSort, Position, Team};

/// Style for board.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum BoardStyle {
    /// Default grassy style.
    #[default]
    Grass,
    /// Teleport style used in teleport rune menu.
    Teleport,
    /// Desert style.
    Desert,
    /// Fleshy style.
    Flesh,
    /// Crusty style.
    Crust,
    /// Eldritch style.
    Eldritch,
}

impl BoardStyle {
    /// Returns the sprite offet for the board.
    pub fn sprite_offset(&self) -> (usize, usize) {
        match self {
            BoardStyle::Grass => (0, 0),
            BoardStyle::Teleport => (0, 64),
            BoardStyle::Desert => (0, 128),
            BoardStyle::Flesh => (0, 256),
            BoardStyle::Crust => (0, 320),
            BoardStyle::Eldritch => (0, 384),
        }
    }

    /// Next style
    pub fn next(&self) -> BoardStyle {
        match self {
            BoardStyle::Grass => BoardStyle::Desert,
            BoardStyle::Desert => BoardStyle::Flesh,
            BoardStyle::Flesh => BoardStyle::Crust,
            BoardStyle::Crust => BoardStyle::Eldritch,
            BoardStyle::Eldritch => BoardStyle::Grass,
            BoardStyle::Teleport => BoardStyle::Grass,
        }
    }

    /// Previous style
    pub fn previous(&self) -> BoardStyle {
        match self {
            BoardStyle::Grass => BoardStyle::Eldritch,
            BoardStyle::Desert => BoardStyle::Grass,
            BoardStyle::Flesh => BoardStyle::Desert,
            BoardStyle::Crust => BoardStyle::Flesh,
            BoardStyle::Eldritch => BoardStyle::Crust,
            BoardStyle::Teleport => BoardStyle::Grass,
        }
    }
}

/// Default size of the game board.
pub const DEFAULT_BOARD_SIZE: (usize, usize) = (6, 6);

/// [`Board`] is a struct which currently only contains the size of the playing field.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    /// Width of the board.
    pub width: usize,
    /// Height of the board.
    pub height: usize,
    /// Style of the board
    pub style: BoardStyle,
}

impl Board {
    /// Instantiates the [`Board`] `struct` with a certain size.
    /// Restricted to `4..=8` on both axes. Currently always 8-by-8.
    pub fn new(width: usize, height: usize) -> Result<Board, &'static str> {
        match (width, height) {
            (width, height) if (3..=8).contains(&width) && (3..=8).contains(&height) => Ok(Board {
                width,
                height,
                ..Default::default()
            }),
            _ => Err("board size does not conform to limits"),
        }
    }

    /// Instantiates the [`Board`] `struct` with a certain size.
    /// Restricted to `4..=8` on both axes. Currently always 8-by-8.
    pub fn with_style(
        width: usize,
        height: usize,
        style: BoardStyle,
    ) -> Result<Board, &'static str> {
        match (width, height) {
            (width, height) if (3..=8).contains(&width) && (3..=8).contains(&height) => Ok(Board {
                width,
                height,
                style,
            }),
            _ => Err("board size does not conform to limits"),
        }
    }

    /// Unrestricted for draw calls
    pub fn unchecked(width: usize, height: usize, style: BoardStyle) -> Board {
        Board {
            width,
            height,
            style,
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
                Position(x_offset + index as i8, if self.height >= 7 { 1 } else { 0 })
                    .align(self, team),
            ));
        }

        mages
    }

    /// Validates a [`Position`] and, if necessary, converts it to a valid one.
    /// Currently only confirms that the position resides on the board (albeit very inefficiently)
    pub fn validate_position(&self, position: Position) -> Option<Position> {
        // TODO restore bounds check
        if position == position.wrap(self.width as i8, self.height as i8) {
            Some(position)
        } else {
            None
        }
    }

    /// Clamps a [`Position`] to rest within this board.
    pub fn clamp_position(&self, position: Position) -> Position {
        Position(
            position.0.clamp(0, self.width as i8 - 1),
            position.1.clamp(0, self.height as i8 - 1),
        )
    }

    /// Converts a canvas location to a board [`Position`].
    pub fn location_as_position(
        &self,
        location: (i32, i32),
        offset: (i32, i32),
        scale: (i32, i32),
    ) -> Option<Position> {
        let position = Position(
            ((location.0 - offset.0) / scale.0) as i8,
            ((location.1 - offset.1) / scale.1) as i8,
        );

        if (location.0 - offset.0) >= 0
            && position.0 < self.width as i8
            && (location.1 - offset.1) >= 0
            && position.1 < self.height as i8
        {
            Some(position)
        } else {
            None
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            width: DEFAULT_BOARD_SIZE.0,
            height: DEFAULT_BOARD_SIZE.1,
            style: Default::default(),
        }
    }
}
