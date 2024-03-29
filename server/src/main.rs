use std::{
    collections::HashMap,
    fs::{self, File},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Json, Path, State},
    routing::{get, post},
    Router,
};
use rand::Rng;
use shared::{
    timestamp, Lobby, LobbyError, LobbySettings, LobbySort, Message, SessionMessage,
    SessionNewLobby, SessionRequest, Turn,
};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {
    lobbies: Arc<Mutex<HashMap<u16, Lobby>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        lobbies: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route_service("/", ServeFile::new("html/game.html"))
        .route("/lobbies", get(get_lobbies))
        .route("/lobby/create", post(create_lobby))
        .route("/lobby/:id/turns/:since", post(get_turns_since))
        .route("/lobby/:id/act", post(process_inbound))
        .route("/lobby/:id/ready", post(post_ready))
        .route("/lobby/:id/rematch", post(post_rematch))
        .route("/lobby/:id/state", get(get_state))
        .route("/session", get(obtain_session))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_lobbies(State(state): State<AppState>) -> Json<Message> {
    let mut lobbies = state.lobbies.lock().unwrap();

    // let lobby_settings = LobbySettings {
    //     lobby_sort: shared::LobbySort::Local,
    //     ..Default::default()
    // };
    // let lobby = Lobby::new(lobby_settings, timestamp());

    // let len = lobbies.len() as u16;

    // if len < 5 {
    //     lobbies.insert(len, lobby.clone());
    // }

    lobbies.retain(|_, v| v.any_connected(timestamp()) && !v.finished());

    Json(Message::Lobbies(lobbies.clone()))
}

async fn create_lobby(
    State(state): State<AppState>,
    Json(mut session_message): Json<SessionNewLobby>,
) -> Json<Message> {
    let lobby_id = generate_lobby_id();
    let mut lobbies = state.lobbies.lock().unwrap();

    session_message.lobby_settings.lobby_sort = LobbySort::Online(lobby_id);
    let lobby = Lobby::new(session_message.lobby_settings, timestamp());

    lobbies.insert(lobby_id, lobby.clone());

    Json(Message::Lobby(Box::new(lobby)))
}

async fn get_turns_since(
    State(state): State<AppState>,
    Path((id, since)): Path<(u16, usize)>,
    Json(session_request): Json<SessionRequest>,
) -> Json<Message> {
    let mut lobbies = state.lobbies.lock().unwrap();

    if let Some(lobby) = lobbies.get_mut(&id) {
        lobby.beat_heart(session_request.session_id);

        if lobby.all_ready() {
            let turns_since: Vec<Turn> =
                lobby.game.turns_since(since).into_iter().cloned().collect();
            Json(Message::Turns(turns_since))
        } else {
            Json(Message::Lobby(Box::new(lobby.clone())))
        }
    } else {
        Json(Message::LobbyError(LobbyError(
            "lobby does not exist".to_string(),
        )))
    }
}

async fn get_state(State(state): State<AppState>, Path(id): Path<u16>) -> Json<Message> {
    let lobbies = state.lobbies.lock().unwrap();

    match lobbies.get(&id) {
        Some(lobby) => Json(Message::Lobby(Box::new(lobby.clone()))),
        None => Json(Message::LobbyError(LobbyError(
            "lobby does not exist".to_string(),
        ))),
    }
}

async fn process_inbound(
    State(state): State<AppState>,
    Path(id): Path<u16>,
    Json(session_message): Json<SessionMessage>,
) -> Json<Message> {
    let mut lobbies = state.lobbies.lock().unwrap();

    Json(match lobbies.get_mut(&id) {
        Some(lobby) => {
            let result: Message = lobby
                .act_player(session_message.session_id, session_message.message)
                .into();
            record_lobby(id, lobby);
            result
        }
        None => Message::LobbyError(LobbyError("lobby does not exist".to_string())),
    })
}

async fn post_ready(
    State(state): State<AppState>,
    Path(id): Path<u16>,
    Json(session_request): Json<SessionRequest>,
) -> Json<Message> {
    let mut lobbies = state.lobbies.lock().unwrap();

    Json(match lobbies.get_mut(&id) {
        Some(lobby) => {
            let result = lobby.join_player(session_request.session_id.clone()).into();
            lobby.beat_heart(session_request.session_id);

            result
        }
        None => Message::LobbyError(LobbyError("lobby does not exist".to_string())),
    })
}

async fn post_rematch(
    State(state): State<AppState>,
    Path(id): Path<u16>,
    Json(session_request): Json<SessionRequest>,
) -> Json<Message> {
    let mut lobbies = state.lobbies.lock().unwrap();

    Json(match lobbies.get_mut(&id) {
        Some(lobby) => {
            let result = lobby.request_rematch(session_request.session_id);

            if let Ok(true) = result {
                lobby.remake(timestamp());
            }

            result.into()
        }
        None => Message::LobbyError(LobbyError("lobby does not exist".to_string())),
    })
}

async fn obtain_session() -> Json<SessionRequest> {
    Json(SessionRequest {
        session_id: generate_session_id(),
    })
}

fn record_lobby(id: u16, lobby: &Lobby) {
    fs::create_dir_all("lobbies").unwrap();
    let file = File::create(format!("lobbies/{}.json", id)).unwrap();
    serde_json::to_writer(&file, lobby).unwrap();
}

fn generate_session_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

fn generate_lobby_id() -> u16 {
    loop {
        let res = rand::thread_rng().gen_range(u16::MIN..=u16::MAX);

        if res.count_ones() >= 4 {
            return res;
        }
    }
}
