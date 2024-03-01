use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json_any_key::*;

use crate::{Lobby, LobbyError, LobbySettings, Turn};

/// A network message.
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    /// Everything's zappin'!
    Ok,
    /// A single [`Turn`].
    Turn(Turn),
    /// A list of [`Turn`]s for synchronising observers who may be multiple turns behind.
    Turns(Vec<Turn>),
    /// An entire [`Lobby`] state for complete synchronisation.
    Lobby(Box<Lobby>),
    /// List of lobbies
    Lobbies(#[serde(with = "any_key_map")] HashMap<u16, Lobby>),
    /// A [`LobbyError`].
    LobbyError(LobbyError),
}

/// An HTTP request made with a certain session ID.
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionRequest {
    /// The session ID for this request.
    pub session_id: String,
}

/// An HTTP request made with a session ID, containing a [`Message`] payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionMessage {
    /// The session ID for this request.
    pub session_id: String,
    /// A [`Message`] payload.
    pub message: Message,
}

/// An HTTP request made with a session ID, containing a [`Message`] payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionNewLobby {
    /// A [`Message`] payload.
    pub lobby_settings: LobbySettings,
}

/// Generates a [`Duration`] for client/server synchronisation and expiry checks.
/// The result is a UNIX timestamp.
#[cfg(feature = "server")]
pub fn timestamp() -> Duration {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");

    Duration::from_secs_f64(since_the_epoch.as_secs_f64())
}