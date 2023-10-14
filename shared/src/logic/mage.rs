use serde::{Deserialize, Serialize};

use crate::{Board, Mana, Position, PowerUp, Spell, Team};

/// A [`MageSort`] is the distinct type of the mage, determining its visual appearance and spell.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MageSort {
    /// A simple mage who attacks with a diamond pattern.
    Diamond,
    /// A simple mage who attacks with an X pattern.
    Cross,
    /// A simple mage who attacks with a chess knight pattern.
    Knight,
    /// A simple mage who attacks with a funky pattern.
    Spike,
    /// A simple mage who attacks with a + pattern.
    Plus,
}

impl MageSort {
    /// Rotates the [`MageSort`] left.
    pub fn previous(&self) -> MageSort {
        match self {
            MageSort::Diamond => MageSort::Plus,
            MageSort::Cross => MageSort::Diamond,
            MageSort::Knight => MageSort::Cross,
            MageSort::Spike => MageSort::Knight,
            MageSort::Plus => MageSort::Spike,
        }
    }

    /// Rotates the [`MageSort`] right.
    pub fn next(&self) -> MageSort {
        match self {
            MageSort::Diamond => MageSort::Cross,
            MageSort::Cross => MageSort::Knight,
            MageSort::Knight => MageSort::Spike,
            MageSort::Spike => MageSort::Plus,
            MageSort::Plus => MageSort::Diamond,
        }
    }
}

impl From<usize> for MageSort {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Diamond,
            1 => Self::Cross,
            2 => Self::Knight,
            3 => Self::Spike,
            _ => Self::Plus,
        }
    }
}

impl Default for MageSort {
    fn default() -> Self {
        Self::Cross
    }
}

/// A [`Mage`] is the playable unit on the [`crate::Board`].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mage {
    /// A unique identifier to determine the selected [`Mage`] by the front-end and also its visual appearence.
    pub index: usize,
    /// [`Position`] of the mage on the board.
    pub position: Position,
    /// [`Sort`] of the mage.
    pub sort: MageSort,
    /// [`Mana`] of the mage.
    pub mana: Mana,
    /// [`Team`] of the mage.
    pub team: Team,
    /// [`Spell`] of the mage.
    pub spell: Spell,
    /// [`PowerUp`] of the mage.
    pub powerup: Option<PowerUp>,
}

impl PartialEq for Mage {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Mage {
    /// Instantiates the [`Mage`] `struct` using [`crate::Mana::with_max()`]
    pub fn new(index: usize, team: Team, sort: MageSort, position: Position) -> Mage {
        Mage {
            index,
            position,
            sort,
            team,
            mana: Mana::select(sort),
            spell: Spell::select(sort),
            powerup: None,
        }
    }

    /// Instantiates a [`Mage`] for use within the editor.
    pub fn editor_new(
        index: usize,
        team: Team,
        sort: MageSort,
        mana: Mana,
        position: Position,
    ) -> Mage {
        Mage {
            index,
            position,
            sort,
            team,
            mana,
            spell: Spell::select(sort),
            powerup: None,
        }
    }

    /// Determines if the [`Mage`] is alive. A *sleeping* mage has no mana left (`== 0`).
    pub fn is_alive(&self) -> bool {
        self.mana > 0
    }

    /// Determines if the [`Mage`] has access to the diagonal moves.
    pub fn has_diagonals(&self) -> bool {
        self.powerup == Some(PowerUp::Diagonal)
    }

    /// Determines if the [`Mage`] is in defensive mode.
    pub fn is_defensive(&self) -> bool {
        self.powerup == Some(PowerUp::Shield)
    }

    /// Returns the list of targets a [`Mage`] can attack to on a certain [`Position`].
    pub fn targets(&self, board: &Board, at: &Position) -> Vec<Position> {
        let mut moves = Vec::with_capacity(self.spell.pattern.len());

        for dir in &self.spell.pattern {
            let position = at + dir;
            // let position = position.wrap(self.board.width as i8, self.board.height as i8);

            if let Some(position) = board.validate_position(position) {
                moves.push(position);
            }
        }

        moves
    }
}

/// Trait for helper functions that interface with `Vec<Mages>`.
pub trait Mages {
    /// Returns a reference to a [`Mage`] at a certain [`Position`].
    fn occupant(&self, position: &Position) -> Option<&Mage>;
    /// Returns a mutable reference to a [`Mage`] at a certain [`Position`].
    fn occupant_mut(&mut self, position: &Position) -> Option<&mut Mage>;
    /// Determines whether or not a move can be made to the [`Position`].
    fn occupied(&self, position: &Position) -> bool;
    /// Returns a reference to a [`Mage`] at a certain [`Position`] if the mage is alive.
    fn live_occupant(&self, position: &Position) -> Option<&Mage>;
    /// Returns a mutable reference to a [`Mage`] at a certain [`Position`] if the mage is alive.
    fn live_occupant_mut(&mut self, position: &Position) -> Option<&mut Mage>;
    /// Determines whether or not a [`Position`] is occupied by a live [`Mage`].
    fn live_occupied(&self, position: &Position) -> bool;
    /// Determines whether or not a [`Position`] is occupied by a live [`Mage`] of a certain [`Team`].
    fn live_occupied_by(&self, position: &Position, team: Team) -> bool;
}

impl Mages for Vec<Mage> {
    fn occupant(&self, position: &Position) -> Option<&Mage> {
        self.iter().find(|&mage| mage.position == *position)
    }

    fn occupant_mut(&mut self, position: &Position) -> Option<&mut Mage> {
        self.iter_mut().find(|mage| mage.position == *position)
    }

    fn occupied(&self, position: &Position) -> bool {
        self.occupant(position).is_some()
    }

    fn live_occupant(&self, position: &Position) -> Option<&Mage> {
        self.iter()
            .filter(|mage| mage.is_alive())
            .find(|&mage| mage.position == *position)
    }

    fn live_occupant_mut(&mut self, position: &Position) -> Option<&mut Mage> {
        self.iter_mut()
            .filter(|mage| mage.is_alive())
            .find(|mage| mage.position == *position)
    }

    fn live_occupied(&self, position: &Position) -> bool {
        self.live_occupant(position).is_some()
    }

    fn live_occupied_by(&self, position: &Position, team: Team) -> bool {
        if let Some(occupant) = self.live_occupant(position) {
            occupant.team == team
        } else {
            false
        }
    }
}

impl Default for Mage {
    fn default() -> Self {
        Self::new(0, Team::default(), MageSort::default(), Position::default())
    }
}

impl From<&Mage> for Vec<u8> {
    fn from(mage: &Mage) -> Self {
        let position_x = mage.position.0 as u8;
        let position_y = mage.position.1 as u8;
        let team = mage.team as u8;

        let sort = mage.sort as u8;
        let mana: u8 = (&mage.mana).into();

        vec![
            (position_x & 0b111) << 5 | (position_y & 0b111) << 2 | team & 0b11,
            sort,
            mana,
        ]
    }
}

impl From<Vec<u8>> for Mage {
    fn from(value: Vec<u8>) -> Self {
        if value.len() == 3 {
            let pos_team_byte = value[0];
            let position_x = (pos_team_byte >> 5) & 0b111;
            let position_y = (pos_team_byte >> 2) & 0b111;
            let position = Position(position_x as i8, position_y as i8);
            let team = Team::from_index((pos_team_byte & 0b11) as usize);

            let sort_byte = value[1] as usize;
            let sort = sort_byte.into();

            let mut mage = Mage::new(0, team, sort, position);

            let mana_byte = value[2];
            let mana = mana_byte.into();

            mage.mana = mana;

            mage
        } else {
            Mage::default()
        }
    }
}
