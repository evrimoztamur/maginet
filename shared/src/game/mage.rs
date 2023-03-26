use serde::{Deserialize, Serialize};

use crate::{Mana, Position, Spell, Team};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mage {
    pub index: usize,
    pub position: Position,
    pub mana: Mana,
    pub team: Team,
    pub spell: Spell,
}

impl Mage {
    pub fn new(index: usize, position: Position, max_mana: u8, team: Team, spell: Spell) -> Mage {
        Mage {
            index,
            position,
            team,
            mana: Mana::with_max(max_mana),
            spell,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.mana > 0
    }

    pub fn has_diagonals(&self) -> bool {
        self.mana <= 2
    }
}