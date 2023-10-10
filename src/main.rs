use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde::Serialize;
use std::io::prelude::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use uuid::Uuid;

#[derive(Serialize)]
struct Chat {
    id: Uuid,
}

//    Basic Axum stuff. Might just build TCP stuff from scratch
//    let app = Router::new()
//        .route("/", get(health_check))
//        .route("/chat", get(create_chat));
//
//    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
//
//    axum::Server::bind(&addr)
//        .serve(app.into_make_service())
//        .await
//        .unwrap();
//
//    Ok(())

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
}

// #[tokio::main]
// async
fn main() -> std::io::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:80")?;

    for stream in socket.incoming() {
        let _ = handle_client(stream?);
    }
    Ok(())
}

// async fn health_check() -> &'static str {
//     "Server is healthy and running :)"
// }

// #[axum::debug_handler]
// async fn create_chat() -> Result<Json<Chat>, StatusCode> {
//     let chat = Chat { id: Uuid::new_v4() };
//     Ok(Json(chat))
// }
