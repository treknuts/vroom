use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde::Serialize;
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Serialize)]
struct Chat {
    id: Uuid,
}

#[tokio::main]
async fn main() -> Result<(), StatusCode> {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/chat", get(create_chat));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() -> &'static str {
    "Server is healthy and running :)"
}

#[axum::debug_handler]
async fn create_chat() -> Result<Json<Chat>, StatusCode> {
    let chat = Chat { id: Uuid::new_v4() };
    Ok(Json(chat))
}
