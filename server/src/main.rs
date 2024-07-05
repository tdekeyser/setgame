extern crate core;

use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, get_service, post};
use tower_http::services::ServeDir;

use crate::card_game::{CardGame, Triple};

mod error;
mod card_game;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/game", get(game_handler))
        .route("/api/set", post(set_handler))
        .fallback(
            get_service(ServeDir::new("./webapp/dist")).handle_error(|_| async move {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn game_handler() -> impl IntoResponse {
    let game = CardGame::generate_fixed(12, 6);
    (StatusCode::OK, Json(game))
}

async fn set_handler(triple: Json<Triple>) -> impl IntoResponse {
    (StatusCode::OK, Json(triple.is_a_set()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all() {}
}
