use serde::{Deserialize, Serialize};

/// An `enum` for the teams. Currently there are only two teams, red and blue.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum Team {
    /// Red team.
    Red,
    /// Blue team.
    Blue,
}

impl Team {
    /// Returns the opposing team.
    pub fn enemy(&self) -> Team {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }
}
