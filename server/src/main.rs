use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeFile::new("index.html"))
        .nest_service("/pkg", ServeDir::new("pkg"))
        .nest_service("/img", ServeDir::new("img"))
        .route("/test", get(get_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Message {
    value: String,
}

async fn get_user() -> Json<Message> {
    Json(Message {
        value: "Hello".to_string(),
    })
}
