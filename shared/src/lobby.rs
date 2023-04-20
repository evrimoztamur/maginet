use std::collections::{HashMap, VecDeque};

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use serde::{Deserialize, Serialize};

use crate::{Game, MageSort, Message, Team, Turn};

const DEFAULT_BOARD_SIZE: (usize, usize) = (8, 8);

/// A identifier for a lobby, shared by the client and the server.
pub type LobbyID = u16;

/// Errors concerning the [`Lobby`].
#[derive(Debug, Serialize, Deserialize)]
pub struct LobbyError(pub String);

impl<T> From<Result<T, LobbyError>> for Message {
    fn from(result: Result<T, LobbyError>) -> Self {
        match result {
            Ok(_) => Message::Ok,
            Err(err) => Message::LobbyError(err),
        }
    }
}

/// A player in a lobby, used in online lobbies only.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    /// The player's team.
    pub team: Team,
    /// Whether the player wants to rematch or not.
    pub rematch: bool,
}

impl Player {
    fn new(team: Team) -> Player {
        Player {
            team,
            rematch: false,
        }
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.team == other.team
    }
}

/// [`Lobby`] is a `struct` which contains all the information necessary for executing a game.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lobby {
    /// The active [`Game`] of this lobby.
    pub game: Game,
    players: HashMap<String, Player>,
    player_slots: VecDeque<Player>,
    ticks: usize,
    /// The [`Lobby`]s sort.
    pub settings: LobbySettings,
}

impl Lobby {
    /// Instantiates the [`Lobby`] `struct` with a given [`LobbySort`].
    pub fn new(settings: LobbySettings) -> Lobby {
        let mut rng = ChaCha8Rng::seed_from_u64(settings.seed as u64);
        let (red_mage_sorts, blue_mage_sorts) = Lobby::select_loadouts(&settings, &mut rng);

        Lobby {
            game: Game::new(
                DEFAULT_BOARD_SIZE.0,
                DEFAULT_BOARD_SIZE.1,
                red_mage_sorts,
                blue_mage_sorts,
                Lobby::random_team(&mut rng),
            )
            .expect("game should be instantiable with default values"),
            players: HashMap::new(),
            player_slots: VecDeque::from([Player::new(Team::Red), Player::new(Team::Blue)]),
            ticks: 0,
            settings,
        }
    }

    fn select_loadouts(
        lobby_settings: &LobbySettings,
        rng: &mut ChaCha8Rng,
    ) -> (Vec<MageSort>, Vec<MageSort>) {
        match lobby_settings.loadout_method {
            LoadoutMethod::Default => (Lobby::default_loadout(), Lobby::default_loadout()),
            LoadoutMethod::Manual => todo!(),
            LoadoutMethod::Random { symmetric } => {
                if symmetric {
                    let loadout = Lobby::random_loadout(rng);

                    (loadout.clone(), loadout)
                } else {
                    (Lobby::random_loadout(rng), Lobby::random_loadout(rng))
                }
            }
        }
    }

    fn default_loadout() -> Vec<MageSort> {
        vec![
            MageSort::Diamond,
            MageSort::Spike,
            MageSort::Knight,
            MageSort::Cross,
        ]
    }

    fn random_loadout(rng: &mut ChaCha8Rng) -> Vec<MageSort> {
        (0..4)
            .into_iter()
            .map(|_| ((rng.next_u64() % 4) as usize).into())
            .collect::<Vec<MageSort>>()
    }

    fn random_team(rng: &mut ChaCha8Rng) -> Team {
        if true || rng.next_u32() % 2 == 0 {
            Team::Red
        } else {
            Team::Blue
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

    #[cfg(feature = "server")]
    /// Includes a new session ID into the lobby, and assigns a player index to it.
    pub fn join_player(&mut self, session_id: String) -> Result<(), LobbyError> {
        if self.all_ready() {
            Err(LobbyError("cannot join an active game".to_string()))
        } else if self.players.contains_key(&session_id) {
            Err(LobbyError("already in lobby".to_string()))
        } else {
            if let Some(player) = self.player_slots.pop_front() {
                self.players.insert(session_id.clone(), player);

                self.tick();

                Ok(())
            } else {
                Err(LobbyError("no available slots in lobby".to_string()))
            }
        }
    }

    // #[cfg(feature = "server")]
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

    #[cfg(feature = "server")]
    /// Executes a certain [`Message`] for the player.
    pub fn act_player(&mut self, session_id: String, message: Message) -> Result<(), LobbyError> {
        if !self.all_ready() {
            Err(LobbyError("game not yet started".to_string()))
        } else {
            match self.players.get(&session_id) {
                Some(player) => {
                    if self.game.turn_for() == player.team {
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

    #[cfg(feature = "server")]
    /// Requests a rematch for the active game.
    pub fn request_rematch(&mut self, session_id: String) -> Result<bool, LobbyError> {
        if !self.all_ready() {
            Err(LobbyError("game not yet started".to_string()))
        } else if !self.finished() {
            Err(LobbyError("game not yet finished".to_string()))
        } else {
            match self.players.get_mut(&session_id) {
                Some(player) => {
                    player.rematch = true;

                    Ok(self
                        .players
                        .values()
                        .fold(true, |acc, player| acc & player.rematch))
                }
                None => Err(LobbyError("player not in lobby".to_string())),
            }
        }
    }

    /// Makes a fully-reset clone of this [`Lobby`].
    pub fn remake(&mut self) {
        *self = Lobby::new(self.settings.clone());
    }

    /// Determines if the game is finished.
    pub fn finished(&self) -> bool {
        self.game
            .all_available_turns(self.game.turn_for())
            .is_empty()
    }

    /// Determines if the game is local (`true`) or online.
    pub fn is_local(&self) -> bool {
        match self.settings.lobby_sort {
            LobbySort::Online(_) => false,
            _ => true,
        }
    }

    /// Returns `true` for [`LobbySort::LocalAI`].
    pub fn has_ai(&self) -> bool {
        match self.settings.lobby_sort {
            LobbySort::LocalAI => true,
            _ => false,
        }
    }

    /// Determines if the given session ID is the one taking its turn.
    pub fn is_active_player(&self, session_id: Option<&String>) -> bool {
        if self.is_local() {
            !(self.has_ai() && self.game.turn_for() == Team::Blue)
        } else if !self.all_ready() {
            false
        } else {
            match session_id {
                Some(session_id) => match self.players.get(session_id) {
                    Some(player) => self.game.turn_for() == player.team,
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

    /// Returns the players.
    pub fn players(&self) -> &HashMap<String, Player> {
        &self.players
    }
}

/// Loadout methods.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Default)]
pub enum LoadoutMethod {
    /// Choose the default order.
    #[default]
    Default,
    /// Choose wizards manually.
    Manual,
    /// Choose wizards randomly.
    Random {
        /// Assign random sets to all players, or the same random set to all.
        symmetric: bool,
    },
}

/// Loadout methods.
#[derive(PartialEq, Debug, Serialize, Deserialize, Clone, Default)]
pub enum LobbySort {
    /// Choose the default order..
    #[default]
    Local,
    /// Versus AI.
    LocalAI,
    /// Online.
    Online(u16),
}

impl LobbySort {
    /// Returns the ID of the lobby, if Online.
    pub fn lobby_id(&self) -> Option<u16> {
        match self {
            LobbySort::Online(id) => Some(id.clone()),
            _ => None,
        }
    }
}

/// Settings for the lobby.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LobbySettings {
    /// Sort of the lobby.
    pub lobby_sort: LobbySort,
    /// [`LoadoutMethod`] for the lobby.
    pub loadout_method: LoadoutMethod,
    /// Seed for RNG.
    pub seed: u32,
}
