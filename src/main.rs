extern crate core;

use axum::Router;
use axum::routing::get;

use crate::card_game::CardGame;

mod error;
mod card_game;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/game", get(game_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn game_handler() -> axum::Json<CardGame> {
    let game = CardGame::generate_fixed(12, 6);
    axum::Json(game)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_all() {}
}
