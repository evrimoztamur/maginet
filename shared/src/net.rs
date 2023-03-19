use serde::{Deserialize, Serialize};

use crate::{Lobby, LobbyError, Turn};

#[derive(Serialize)]
pub enum OutMessage<'a> {
    Move(Turn),
    Lobby(&'a Lobby),
    LobbyError(LobbyError),
}

#[derive(Debug, Deserialize)]
pub enum Message {
    Move(Turn),
    Lobby(Lobby),
    LobbyError(LobbyError),
}

#[derive(Serialize, Deserialize)]
pub struct SessionRequest {
    pub session_id: String,
}

#[derive(Deserialize)]
pub struct SessionMessage {
    pub session_id: String,
    pub message: Message,
}

#[derive(Serialize)]
pub struct OutSessionMessage<'a> {
    pub session_id: String,
    pub message: OutMessage<'a>,
}
