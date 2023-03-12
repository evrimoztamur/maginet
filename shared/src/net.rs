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
