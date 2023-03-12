use axum::{
    extract::{self, FromRef, Path},
    http::{header, HeaderValue},
    response::{IntoResponse, Redirect, Response},
    routing::get,
    routing::post,
    Router,
};
use axum_extra::extract::{
    cookie::{Cookie, Key},
    SignedCookieJar,
};
use rand::Rng;
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr};
use tower_http::services::{ServeDir, ServeFile};

use shared::{Lobby, LobbyError, Message, OutMessage};

use axum::extract::State;
use std::sync::{Arc, Mutex};

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
        .nest_service("/", ServeFile::new("html/index.html"))
        .route("/lobby/create", post(create_lobby))
        .nest_service("/lobby/:id", ServeFile::new("html/game.html"))
        .route("/lobby/:id/turns/:since", get(get_turns_since))
        .route("/lobby/:id/act", post(process_inbound))
        .route("/lobby/:id/ready", post(post_ready))
        .route("/lobby/:id/state", get(get_state))
        // .route("/reset", post(reset_game))
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

    lobbies.insert(lobby_id.clone(), Lobby::new());

    Redirect::to(&format!("/lobby/{lobby_id}"))
}

async fn get_turns_since(
    jar: SignedCookieJar,
    State(state): State<AppState>,
    Path((id, since)): Path<(String, usize)>,
) -> impl IntoResponse {
    let (jar, is_new_user, session_id) = identify_user(jar);

    let lobbies = state.lobbies.lock().unwrap();

    (
        jar,
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
        },
    )
}

async fn get_state(
    jar: SignedCookieJar,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let (jar, is_new_user, session_id) = identify_user(jar);

    let lobbies = state.lobbies.lock().unwrap();

    (
        jar,
        match lobbies.get(&id) {
            Some(lobby) => serialized_response(&OutMessage::Lobby(lobby)),
            None => serialized_response(&OutMessage::LobbyError(LobbyError(
                "lobby does not exist".to_string(),
            ))),
        },
    )
}

async fn process_inbound(
    jar: SignedCookieJar,
    State(state): State<AppState>,
    Path(id): Path<String>,
    extract::Json(message): extract::Json<Message>,
) -> impl IntoResponse {
    let (jar, is_new_user, session_id) = identify_user(jar);

    let mut lobbies = state.lobbies.lock().unwrap();

    match lobbies.get_mut(&id) {
        Some(lobby) => lobby.act_player(session_id, message),
        None => Err(LobbyError("lobby does not exist".to_string())),
    };

    jar
}
async fn post_ready(
    jar: SignedCookieJar,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let (jar, is_new_user, session_id) = identify_user(jar);

    let mut lobbies = state.lobbies.lock().unwrap();

    match lobbies.get_mut(&id) {
        Some(lobby) => {
            lobby.join_player(session_id)
        }
        None => Err(LobbyError("lobby does not exist".to_string())),
    };

    jar
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

// async fn reset_game(State(state): State<AppState>) -> impl IntoResponse {
//     let mut game = state.game.0.lock().unwrap();
//     std::mem::replace(&mut *game, Game::new(8, 8, 4).unwrap());
//     Redirect::to("/")
// }

fn identify_user(jar: SignedCookieJar) -> (SignedCookieJar, bool, String) {
    if let Some(session_id) = jar.get("session_id") {
        (jar, false, session_id.value().to_string())
    } else {
        let session_id = generate_id();
        let cookie = Cookie::build("session_id", session_id.clone()).path("/").finish();

        (jar.add(cookie), true, session_id)
    }
}

fn generate_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
