use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    body::Body,
    extract::{self, FromRef, Path, State},
    http::{header, HeaderValue, Request},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Router,
};
use axum_extra::extract::{
    cookie::{Cookie, Key},
    SignedCookieJar,
};
use rand::Rng;
use serde::Serialize;
use shared::{Lobby, LobbyError, Message, OutMessage};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {
    lobbies: Arc<Mutex<HashMap<String, Lobby>>>,
    key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

#[tokio::main]
async fn main() {
    let state = AppState {
        lobbies: Arc::new(Mutex::new(HashMap::new())),
        key: Key::generate(),
    };

    let app = Router::new()
        .nest_service("/pkg", ServeDir::new("pkg"))
        .nest_service("/img", ServeDir::new("img"))
        .route_service("/", ServeFile::new("html/index.html"))
        .route("/lobby/create", post(create_lobby))
        .route("/lobby/:id", get(get_lobby))
        .route("/lobby/:id/turns/:since", get(get_turns_since))
        .route("/lobby/:id/act", post(process_inbound))
        .route("/lobby/:id/ready", post(post_ready))
        .route("/lobby/:id/state", get(get_state))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_lobby(jar: SignedCookieJar, request: Request<Body>) -> impl IntoResponse {
    let response = ServeFile::new("html/game.html").oneshot(request).await;

    if identify_user(&jar).is_none() {
        (jar.add(generate_session_id_cookie()), response)
    } else {
        (jar, response)
    }
}

async fn create_lobby(State(state): State<AppState>) -> impl IntoResponse {
    let lobby_id = generate_id();
    let mut lobbies = state.lobbies.lock().unwrap();

    lobbies.insert(lobby_id.clone(), Lobby::new());

    Redirect::to(&format!("/lobby/{lobby_id}"))
}

async fn get_turns_since(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Path((id, since)): Path<(String, usize)>,
) -> impl IntoResponse {
    if identify_user(&jar).is_some() {
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
    } else {
        serialized_response(&OutMessage::LobbyError(LobbyError(
            "no session ID provided".to_string(),
        )))
    }
}

async fn get_state(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if identify_user(&jar).is_some() {
        let lobbies = state.lobbies.lock().unwrap();

        match lobbies.get(&id) {
            Some(lobby) => serialized_response(&OutMessage::Lobby(lobby)),
            None => serialized_response(&OutMessage::LobbyError(LobbyError(
                "lobby does not exist".to_string(),
            ))),
        }
    } else {
        serialized_response(&OutMessage::LobbyError(LobbyError(
            "no session ID provided".to_string(),
        )))
    }
}

async fn process_inbound(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Path(id): Path<String>,
    extract::Json(message): extract::Json<Message>,
) {
    if let Some(session_id) = identify_user(&jar) {
        let mut lobbies = state.lobbies.lock().unwrap();

        match lobbies.get_mut(&id) {
            Some(lobby) => lobby.act_player(session_id, message),
            None => Err(LobbyError("lobby does not exist".to_string())),
        };
    }
}

async fn post_ready(State(state): State<AppState>, jar: SignedCookieJar, Path(id): Path<String>) {
    if let Some(session_id) = identify_user(&jar) {
        let mut lobbies = state.lobbies.lock().unwrap();

        match lobbies.get_mut(&id) {
            Some(lobby) => lobby.join_player(session_id),
            None => Err(LobbyError("lobby does not exist".to_string())),
        };
    }
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

fn identify_user(jar: &SignedCookieJar) -> Option<String> {
    jar.get("session_id")
        .map(|cookie| cookie.value().to_string())
}

fn generate_session_id_cookie<'c>() -> Cookie<'c> {
    Cookie::build("session_id", generate_id())
        .path("/")
        .finish()
}

fn generate_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
