use serde::{Deserialize, Serialize};
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Team {
    Red,
    Blue,
}

impl Team {
    pub fn enemy(&self) -> Team {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }
}
