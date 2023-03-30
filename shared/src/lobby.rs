use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::{Game, Message, Turn};

const DEFAULT_BOARD_SIZE: (usize, usize) = (8, 8);
const DEFAULT_MAGE_COUNT: usize = 4;
const DEFAULT_PLAYER_COUNT: usize = 2;

/// Errors concerning the [`Lobby`].
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyError(pub String);

impl From<Result<(), LobbyError>> for Message {
    fn from(result: Result<(), LobbyError>) -> Self {
        match result {
            Ok(_) => Message::Ok,
            Err(err) => Message::LobbyError(err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Player {
    index: usize,
}

impl Player {
    fn new(index: usize) -> Player {
        Player { index }
    }
}

/// Sort of the lobby. Currently used to modify game logic and executing networking and AI code.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum LobbySort {
    /// `Lobby` is for on-the-board play involving two local players on the same board.
    Local,
    /// `LocalAI` is the same as `Local`, but executes AI code following player turns.
    LocalAI,
    /// `Online` excutes netcode to synchronise the game state and communicate player turns with the server.
    Online,
}

/// [`Lobby`] is a `struct` which contains all the information necessary for executing a game.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lobby {
    /// The active [`Game`] of the lobby.
    pub game: Game,
    players: HashMap<String, Player>,
    player_slots: VecDeque<usize>,
    ticks: usize,
    /// The [`Lobby`]s sort.
    pub sort: LobbySort,
}

impl Lobby {
    /// Instantiates the [`Lobby`] `struct` with a given [`LobbySort`].
    pub fn new(sort: LobbySort) -> Lobby {
        Lobby {
            game: Game::new(
                DEFAULT_BOARD_SIZE.0,
                DEFAULT_BOARD_SIZE.1,
                DEFAULT_MAGE_COUNT,
            )
            .unwrap(),
            players: HashMap::new(),
            player_slots: (0..DEFAULT_PLAYER_COUNT).collect(),
            ticks: 0,
            sort,
        }
    }

    /// Number of ticks since the lobby's creation.
    /// Used to synchronise lobby-related events.
    pub fn tick(&mut self) {
        self.ticks += 1;
    }

    /// Determines if all players slots are taken.
    pub fn all_ready(&self) -> bool {
        self.player_slots.is_empty()
    }

    /// Includes a new session ID into the lobby, and assigns a player index to it.
    pub fn join_player(&mut self, session_id: String) -> Result<(), LobbyError> {
        if self.all_ready() {
            Err(LobbyError("cannot join an active game".to_string()))
        } else if self.players.contains_key(&session_id) {
            Err(LobbyError("already in lobby".to_string()))
        } else {
            if let Some(index) = self.player_slots.pop_front() {
                self.players.insert(session_id.clone(), Player::new(index));

                self.tick();

                Ok(())
            } else {
                Err(LobbyError("no available slots in lobby".to_string()))
            }
        }
    }

    // pub fn leave_player(&mut self, session_id: String) -> Result<String, LobbyError> {
    //     if self.state == LobbyState::Finished {
    //         Err(LobbyError("cannot leave a finished game".to_string()))
    //     } else {
    //         match self.players.remove(&session_id) {
    //             Some(player) => {
    //                 self.player_slots.push_back(player.index);

    //                 self.players.remove(&session_id);

    //                 self.tick();

    //                 Ok(session_id)
    //             }
    //             None => Err(LobbyError("player not in lobby".to_string())),
    //         }
    //     }
    // }

    /// Executes a certain [`Message`] for the player.
    pub fn act_player(&mut self, session_id: String, message: Message) -> Result<(), LobbyError> {
        if !self.all_ready() {
            Err(LobbyError("game not yet started".to_string()))
        } else {
            match self.players.get(&session_id) {
                Some(player) => {
                    if self.game.turn_index() == player.index {
                        match message {
                            Message::Move(Turn(from, to)) => {
                                self.game.take_move(from, to);
                            }
                            _ => (),
                        }

                        Ok(())
                    } else {
                        Err(LobbyError("not your turn".to_string()))
                    }
                }
                None => Err(LobbyError("player not in lobby".to_string())),
            }
        }
    }

    /// Determines if the lobby is *not* an online one.
    pub fn is_local(&self) -> bool {
        match self.sort {
            LobbySort::Local => true,
            LobbySort::LocalAI => true,
            LobbySort::Online => false,
        }
    }

    /// Determines if the game is finished.
    pub fn finished(&self) -> bool {
        self.game
            .all_available_turns(self.game.turn_for())
            .is_empty()
    }

    /// Returns `true` for [`LobbySort::LocalAI`].
    pub fn has_ai(&self) -> bool {
        self.sort == LobbySort::LocalAI
    }

    /// Determines if the given session ID is the one taking its turn.
    pub fn is_active_player(&self, session_id: Option<&String>) -> bool {
        if self.is_local() {
            true
        } else if !self.all_ready() {
            false
        } else {
            match session_id {
                Some(session_id) => match self.players.get(session_id) {
                    Some(player) => self.game.turn_index() == player.index,
                    None => false,
                },
                None => false,
            }
        }
    }

    /// Detemines whether or not the given session ID is in this lobby.
    pub fn has_session_id(&self, session_id: Option<&String>) -> bool {
        match session_id {
            Some(session_id) => self.players.contains_key(session_id),
            None => false,
        }
    }
}
