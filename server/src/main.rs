use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Json, Path, State},
    http::{header, HeaderValue},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use rand::Rng;
use serde::Serialize;
use shared::{Lobby, LobbyError, OutMessage, SessionMessage, SessionRequest};
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
        .route_service("/itch", ServeFile::new("html/itch.html"))
        .route_service("/local", ServeFile::new("html/game/local.html"))
        .route_service("/local/ai", ServeFile::new("html/game/local.html"))
        .route("/lobby/create", post(create_lobby))
        .route_service("/lobby/:id", ServeFile::new("html/game/online.html"))
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

async fn create_lobby(State(state): State<AppState>) -> impl IntoResponse {
    let lobby_id = generate_id();
    let mut lobbies = state.lobbies.lock().unwrap();

    lobbies.insert(lobby_id.clone(), Lobby::new(shared::LobbySort::Online));

    Redirect::to(&format!("/lobby/{lobby_id}"))
}

async fn get_turns_since(
    State(state): State<AppState>,
    Path((id, since)): Path<(String, usize)>,
) -> impl IntoResponse {
    let lobbies = state.lobbies.lock().unwrap();

    match lobbies.get(&id) {
        Some(lobby) => {
            let turns_since: Vec<OutMessage> = lobby
                .game
                .turns_since(since)
                .iter()
                .map(|turn| OutMessage::Move(**turn))
                .collect();
            serialized_response(&turns_since)
        }
        None => serialized_response(&OutMessage::LobbyError(LobbyError(
            "lobby does not exist".to_string(),
        ))),
    }
}

async fn get_state(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let lobbies = state.lobbies.lock().unwrap();

    match lobbies.get(&id) {
        Some(lobby) => serialized_response(&OutMessage::Lobby(lobby)),
        None => serialized_response(&OutMessage::LobbyError(LobbyError(
            "lobby does not exist".to_string(),
        ))),
    }
}

async fn process_inbound(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(session_message): Json<SessionMessage>,
) -> impl IntoResponse {
    let mut lobbies = state.lobbies.lock().unwrap();

    match lobbies.get_mut(&id) {
        Some(lobby) => lobby.act_player(session_message.session_id, session_message.message),
        None => Err(LobbyError("lobby does not exist".to_string())),
    };
}

async fn post_ready(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(session_request): Json<SessionRequest>,
) -> impl IntoResponse {
    let mut lobbies = state.lobbies.lock().unwrap();

    match lobbies.get_mut(&id) {
        Some(lobby) => lobby.join_player(session_request.session_id),
        None => Err(LobbyError("lobby does not exist".to_string())),
    };
}

async fn obtain_session() -> impl IntoResponse {
    serialized_response(&SessionRequest {
        session_id: generate_id(),
    })
}

fn serialized_response<T: Sized + Serialize>(value: &T) -> Response {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )],
        serde_json::to_string(value).unwrap(),
    )
        .into_response()
}

fn generate_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
