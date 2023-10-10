use serde::Serialize;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::net::{TcpListener, TcpStream};
use uuid::Uuid;

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
    let outgoing = stream.write(b"Welcome to my Rusty TCP Listener")?;
    println!("Sending: {}", outgoing.to_string());

    let mut reader = io::BufReader::new(&mut stream);
    let received = reader.fill_buf()?.to_vec();

    reader.consume(received.len());

    let _ = String::from_utf8(received)
        .map(|msg| println!("Received: {}", msg))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Couldn't parse message as utf8"));

    Ok(())
}

// #[tokio::main]
// async
fn main() -> std::io::Result<()> {
    let socket = TcpListener::bind("127.0.0.1:8080")?;

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
