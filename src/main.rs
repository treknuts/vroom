use axum::routing::{get, Router};
// use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
// use uuid::Uuid;

// #[derive(Deserialize, Serialize, Clone)]
// pub struct User {
//     id: Uuid,
//     username: String,
//     password: String,
// }
//
// impl User {
//     fn new(username: String, password: String) -> User {
//         User {
//             id: Uuid::new_v4(),
//             username: username,
//             password: password,
//         }
//     }
// }
//
// #[derive(Deserialize, Serialize)]
// struct RegisterUser {
//     username: String,
//     password: String,
// }

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/", get(health_check));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn health_check() -> &'static str {
    "Looking good!"
}

// #[debug_handler]
// async fn register(
//     State(users): State<Vec<User>>,
//     Json(payload): Json<RegisterUser>,
// ) -> Result<Json<User>, StatusCode> {
//     let user = User::new(payload.username, payload.password);
//
//     users.push(user);
//
//     Ok(Json(users.get(users.len() - 1).unwrap().clone()))
// }
