use serde::{Deserialize, Serialize};

mod position;
pub use position::*;

mod game;
pub use game::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Move(Position),
}
