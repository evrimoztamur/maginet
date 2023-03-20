use serde::{Deserialize, Serialize};

use crate::{Lobby, LobbyError, Turn};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Move(Turn),
    Moves(Vec<Turn>),
    Lobby(Lobby),
    LobbyError(LobbyError),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRequest {
    pub session_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionMessage {
    pub session_id: String,
    pub message: Message,
}
