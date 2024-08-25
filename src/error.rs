use std::fmt::Display;

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    // Hand errors
    NotEnoughCardsForHand,
    TooManyCardsForHand,
    SpadeNotInHand,
    FaceCardNotInHand,
    DuplicateCardInHand,
    // Trick errors
    TrickIsFull,
    TrickIsEmpty,
    TrickHasNoneCardSomeExpected,
    // Round Errors
    RoundIdTooLarge,
    RoundAlreadyDealt,
    RoundOver,
    RoundExpectsTrick,
    RoundIsNotCalling,
    RoundIsNotBreaking,
    // Turn Errors
    NotYourTurn,
    InvalidCardForBreak,
    // Game errors
    GameExpectsRound,
    GameNotAcceptingPlayers,
    GameWaitingForPlayersToJoin,
    GameOver,
    // Player Errors
    PlayerNotFound,
    // Web Game Error
    GameDoesnotExistError,
    CouldnotLockError,
    // Call Errors
    CallValueTooHigh(usize),
    CallValueTooLow(usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}
