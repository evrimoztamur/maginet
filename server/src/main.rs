use axum::{
    extract::{self, Path},
    http::{header, HeaderValue},
    response::IntoResponse,
    routing::get,
    routing::post,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

use shared::{Game, Message, MutexWrapper, OutMessage, Position, Turn};

use axum::extract::State;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct AppState {
    game: Arc<MutexWrapper<Game>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        game: Arc::new(MutexWrapper(Mutex::new(Game::new(8, 8, 4).unwrap()))),
    };

    let app = Router::new()
        .nest_service("/", ServeFile::new("index.html"))
        .nest_service("/pkg", ServeDir::new("pkg"))
        .nest_service("/img", ServeDir::new("img"))
        .route("/turns/:since", get(get_turns_since))
        .route("/test", post(process_inbound))
        .route("/state", get(get_state))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_turns_since(
    State(state): State<AppState>,
    Path(since): Path<usize>,
) -> impl IntoResponse {
    let game = state.game.0.lock().unwrap();
    let turns_since: Vec<OutMessage> = game
        .turns_since(since)
        .iter()
        .map(|turn| OutMessage::Move(**turn))
        .collect();

    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )],
        serde_json::to_string(&turns_since).unwrap(),
    )
        .into_response()
}

async fn get_state(State(state): State<AppState>) -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )],
        serde_json::to_string(&OutMessage::Game(state.game.as_ref())).unwrap(),
    )
        .into_response()
}

async fn process_inbound(
    State(state): State<AppState>,
    extract::Json(message): extract::Json<Message>,
) {
    let mut game = state.game.0.lock().unwrap();
    match message {
        Message::Move(Turn(from, to)) => {
            game.take_move(from, to);
        }
        _ => (),
    }
}
