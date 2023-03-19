use std::collections::{HashMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::{Game, Message, Turn};

const DEFAULT_BOARD_SIZE: (usize, usize) = (8, 8);
const DEFAULT_MAGE_COUNT: usize = 4;
const DEFAULT_PLAYER_COUNT: usize = 2;

#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyError(pub String);

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    index: usize,
}

impl Player {
    fn new(index: usize) -> Player {
        Player { index }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum LobbySort {
    Local,
    LocalAI,
    Online,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lobby {
    pub game: Game,
    players: HashMap<String, Player>,
    player_slots: VecDeque<usize>,
    ticks: usize,
    pub sort: LobbySort,
}

impl Lobby {
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

    pub fn tick(&mut self) {
        self.ticks += 1;
    }

    pub fn all_ready(&self) -> bool {
        self.player_slots.is_empty()
    }

    pub fn join_player(&mut self, session_id: String) -> Result<String, LobbyError> {
        if self.all_ready() {
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

    pub fn is_local(&self) -> bool {
        match self.sort {
            LobbySort::Local => true,
            LobbySort::LocalAI => true,
            LobbySort::Online => false,
        }
    }

    pub fn finished(&self) -> bool {
        self.game
            .all_available_turns(self.game.turn_for())
            .is_empty()
    }

    pub fn has_ai(&self) -> bool {
        self.sort == LobbySort::LocalAI
    }

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

    pub fn has_session_id(&self, session_id: Option<&String>) -> bool {
        match session_id {
            Some(session_id) => self.players.contains_key(session_id),
            None => false,
        }
    }
}
