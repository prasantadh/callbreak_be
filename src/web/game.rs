use std::sync::Mutex;

use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{model::PlayerInfo, AppState, Error, Game, Result};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/games", post(create_game))
        .route("/games/:id/calls", post(call_handler))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestS {}

async fn create_game(State(state): State<AppState>) -> Result<Json<()>> {
    // generate a new uuid
    let game_id = Uuid::new_v4();
    let player_id = Uuid::new_v4();
    state.data_store.insert(game_id, Mutex::new(Game::new()));
    match state.data_store.get(&game_id) {
        None => return Err(Error::GameDoesnotExistError),
        Some(v) => {
            let mut game = v.lock().map_err(|_e| Error::CouldnotLockError)?;
            let player_info = PlayerInfo::new(player_id);
            game.add_player(&player_info)?;
        }
    }
    Ok(Json(()))
}

async fn call_handler(
    State(state): State<AppState>,
    Path(game_id): Path<Uuid>,
) -> Result<Json<()>> {
    // player id has to come from context
    // does the player id come from cookie or handler?
    Ok(Json(()))
}
