mod error;
mod model;
mod web;

use axum::Router;
use uuid::Uuid;

pub use error::{Error, Result};
pub use model::{Game, PlayerInfo};

pub fn app() -> Router {
    let _state = AppState { data_store: vec![] };
    // Router::new().merge(web::game::routes(state.clone()))
    Router::new()
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub data_store: Vec<u8>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut game = Game::new();
    game.add_player(&PlayerInfo::new(Uuid::new_v4()))?;
    game.add_player(&PlayerInfo::new(Uuid::new_v4()))?;
    game.add_player(&PlayerInfo::new(Uuid::new_v4()))?;
    game.add_player(&PlayerInfo::new(Uuid::new_v4()))?;
    println!("{:?}", game);
    println!("Hello, world!");
    Ok(())
}
