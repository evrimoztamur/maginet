use std::{
    collections::HashMap,
    fs::File,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Json, Path, State},
    response::Redirect,
    routing::{get, post},
    Router,
};
use rand::Rng;
use shared::{Lobby, LobbyError, Message, SessionMessage, SessionRequest, Turn};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {
    lobbies: Arc<Mutex<HashMap<String, Lobby>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        lobbies: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .nest_service("/pkg", ServeDir::new("pkg"))
        .nest_service("/static", ServeDir::new("static"))
        .route_service("/", ServeFile::new("html/index.html"))
        .route_service("/game", ServeFile::new("html/game.html"))
        .route("/lobby/create", post(create_lobby))
        .route("/lobby/:id/turns/:since", get(get_turns_since))
        .route("/lobby/:id/act", post(process_inbound))
        .route("/lobby/:id/ready", post(post_ready))
        .route("/lobby/:id/state", get(get_state))
        .route("/session", get(obtain_session))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_lobby(State(state): State<AppState>) -> Redirect {
    let lobby_id = generate_id();
    let mut lobbies = state.lobbies.lock().unwrap();

    lobbies.insert(lobby_id.clone(), Lobby::new(shared::LobbySettings::default()));

    Redirect::to(&format!("/lobby/{lobby_id}"))
}

async fn get_turns_since(
    State(state): State<AppState>,
    Path((id, since)): Path<(String, usize)>,
) -> Json<Message> {
    let lobbies = state.lobbies.lock().unwrap();

    if let Some(lobby) = lobbies.get(&id) {
        let turns_since: Vec<Turn> = lobby.game.turns_since(since).into_iter().cloned().collect();
        Json(Message::Moves(turns_since))
    } else {
        Json(Message::LobbyError(LobbyError(
            "lobby does not exist".to_string(),
        )))
    }
}

async fn get_state(State(state): State<AppState>, Path(id): Path<String>) -> Json<Message> {
    let lobbies = state.lobbies.lock().unwrap();

    match lobbies.get(&id) {
        Some(lobby) => Json(Message::Lobby(lobby.clone())),
        None => Json(Message::LobbyError(LobbyError(
            "lobby does not exist".to_string(),
        ))),
    }
}

async fn process_inbound(
    State(state): State<AppState>,
    Path(id): Path<String>,
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
    Path(id): Path<String>,
    Json(session_request): Json<SessionRequest>,
) -> Json<Message> {
    let mut lobbies = state.lobbies.lock().unwrap();

    Json(match lobbies.get_mut(&id) {
        Some(lobby) => lobby.join_player(session_request.session_id).into(),
        None => Message::LobbyError(LobbyError("lobby does not exist".to_string())),
    })
}

async fn obtain_session() -> Json<SessionRequest> {
    Json(SessionRequest {
        session_id: generate_id(),
    })
}

fn record_lobby(id: String, lobby: &Lobby) {
    let file = File::create(format!("lobbies/{}.json", id)).unwrap();
    serde_json::to_writer(&file, lobby).unwrap();
}

fn generate_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
