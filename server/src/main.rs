use axum::{
    extract,
    http::{header, HeaderValue, Response},
    response::IntoResponse,
    routing::get,
    routing::post,
    Json, Router,
};
use serde::{Serialize, Serializer};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

use shared::{Game, MutexWrapper, OutMessage, Position, Message};

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
        .route("/test", get(get_test))
        .route("/test", post(process_inbound))
        .route("/state", get(get_state))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_test(State(_): State<AppState>) -> impl IntoResponse {
    (
        [(
            header::CONTENT_TYPE,
            HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
        )],
        serde_json::to_string(&vec![&OutMessage::Move(Position(2, 3), Position(2, 4))]).unwrap(),
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

async fn process_inbound(State(state): State<AppState>, extract::Json(message): extract::Json<Message>) {
    let mut game = state.game.0.lock().unwrap();
    match message {
        Message::Move(from, to) => {
            if let Some(active_mage) = game.live_occupant(&from) {
                // web_sys::console::log_1(&format!("{:?}", active_mage).into());
                let available_moves = active_mage.available_moves(&game);
                let potential_move =
                    available_moves.iter().find(|(position, _)| position == &to);

                if let Some((position, _)) = potential_move {
                    game.live_occupant_mut(&from).unwrap().position = *position;
                    game.attack();
                    game.end_turn();
                }
            }
        },
        // Message::Game(_) => todo!(),
        _ => ()
    }
}
