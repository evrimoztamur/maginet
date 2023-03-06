use axum::{routing::get, Json, Router};
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

use shared::{Message, Position};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeFile::new("index.html"))
        .nest_service("/pkg", ServeDir::new("pkg"))
        .nest_service("/img", ServeDir::new("img"))
        .route("/test", get(get_test));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_test() -> Json<Message> {
    Json(Message::Move(Position(0, 0)))
}
