use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::{Game, Message, Turn};

const DEFAULT_BOARD_SIZE: (usize, usize) = (8, 8);
const DEFAULT_MAGE_COUNT: usize = 4;
const DEFAULT_PLAYER_COUNT: usize = 2;

#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyError(pub String);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum LobbyState {
    Pending,
    Started,
    Finished,
}

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    index: usize,
    ready: bool,
}

impl Player {
    fn new(index: usize) -> Player {
        Player {
            index,
            ready: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lobby {
    state: LobbyState,
    pub game: Game,
    players: HashMap<String, Player>,
    player_slots: VecDeque<usize>,
    ticks: usize,
}

impl Lobby {
    pub fn new() -> Lobby {
        Lobby {
            state: LobbyState::Pending,
            game: Game::new(
                DEFAULT_BOARD_SIZE.0,
                DEFAULT_BOARD_SIZE.1,
                DEFAULT_MAGE_COUNT,
            )
            .unwrap(),
            players: HashMap::new(),
            player_slots: (0..DEFAULT_PLAYER_COUNT).collect(),
            ticks: 0,
        }
    }

    pub fn tick(&mut self) {
        self.ticks += 1;
    }

    pub fn all_ready(&self) -> bool {
        self.player_slots.is_empty() && self.players.iter().all(|(_, player)| player.ready)
    }

    pub fn join_player(&mut self, session_id: String) -> Result<String, LobbyError> {
        if self.state != LobbyState::Pending {
            Err(LobbyError("cannot join an active game".to_string()))
        } else if self.players.contains_key(&session_id) {
            Err(LobbyError("already in lobby".to_string()))
        } else {
            if let Some(index) = self.player_slots.pop_front() {
                self.players.insert(session_id.clone(), Player::new(index));

                self.tick();

                Ok(session_id)
            } else {
                Err(LobbyError("no available slots in lobby".to_string()))
            }
        }
    }

    pub fn ready_player(&mut self, session_id: String) -> Result<String, LobbyError> {
        if self.state != LobbyState::Pending {
            Err(LobbyError("cannot ready for an active game".to_string()))
        } else {
            match self.players.get_mut(&session_id) {
                Some(player) => {
                    player.ready = true;

                    if self.all_ready() {
                        self.state = LobbyState::Started;
                    }

                    self.tick();

                    Ok(session_id)
                }
                None => Err(LobbyError(
                    "cannot ready a player who is not in lobby".to_string(),
                )),
            }
        }
    }

    pub fn leave_player(&mut self, session_id: String) -> Result<String, LobbyError> {
        if self.state == LobbyState::Finished {
            Err(LobbyError("cannot leave a finished game".to_string()))
        } else {
            match self.players.remove(&session_id) {
                Some(player) => {
                    self.player_slots.push_back(player.index);

                    self.players.remove(&session_id);

                    self.tick();

                    Ok(session_id)
                }
                None => Err(LobbyError("player not in lobby".to_string())),
            }
        }
    }

    pub fn act_player(&mut self, session_id: String, message: Message) -> Result<(), LobbyError> {
        if self.state != LobbyState::Started {
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
}
