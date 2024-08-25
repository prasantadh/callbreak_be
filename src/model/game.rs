#![allow(unused)]
use tokio::io::Join;
use uuid::Uuid;

use super::round::{self, Round, RoundId};
use super::Turn;
use super::{Call, Card};
use crate::{Error, Result};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Join,
    Play,
}

// FIXME: thinking it might be cool to call the game callbrust = callbreak + rust
#[derive(Debug)]
pub struct Game {
    id: Uuid,
    rounds: Vec<Round>,
    players: Vec<PlayerInfo>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            id: Uuid::new_v4(),
            rounds: vec![],
            players: vec![],
        }
    }

    // returns current state as well as manages rounds
    // for when rounds need to be added
    fn next(&mut self) -> Result<State> {
        // TODO: State::Join is currently managed by Game while other states
        // are managed by Round. See if there is a way to merge them into one place
        if self.players.len() != 4 {
            return Ok(State::Join);
        }

        if self.rounds.is_empty() {
            self.rounds.push(Round::new(RoundId::try_from(0)?));
        }

        let round_number = self.rounds.len() - 1;
        match self.rounds[round_number].next() {
            Err(Error::RoundOver) => {
                if round_number == 4 {
                    return Err(Error::GameOver);
                }
                self.rounds
                    .push(Round::new(RoundId::try_from(round_number + 1)?));
                Ok(State::Play)
            }
            Ok(_) => Ok(State::Play),
            Err(v) => Err(v),
        }
    }

    pub fn add_player(&mut self, player: &PlayerInfo) -> Result<()> {
        if self.next()? != State::Join {
            return Err(Error::GameNotAcceptingPlayers);
        }
        self.players.push(player.clone());
        self.players.shuffle(&mut thread_rng());
        Ok(())
    }

    fn get_player_turn(&mut self, player: &PlayerInfo) -> Result<Turn> {
        let player_idx = self
            .players
            .iter()
            .position(|p| p == player)
            .ok_or(Error::PlayerNotFound)?;
        Turn::try_from(player_idx)
    }

    pub fn make_a_call(&mut self, player: &PlayerInfo, call: Call) -> Result<()> {
        if self.next()? != State::Play {
            return Err(Error::GameWaitingForPlayersToJoin);
        }
        let turn = self.get_player_turn(player)?;
        let round_number = self.rounds.len() - 1;
        self.rounds[round_number].make_a_call(turn, call)
    }

    pub fn make_a_break(&mut self, player: &PlayerInfo, card: Card) -> Result<()> {
        if self.next()? != State::Play {
            return Err(Error::GameWaitingForPlayersToJoin);
        }
        let turn = self.get_player_turn(player)?;
        let round_number = self.rounds.len() - 1;
        self.rounds[round_number].make_a_break(turn, card)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfo {
    // this id likely will eventually be mongodb objectid
    // TODO while sending data to players, do not send ids if we are using
    // ids to verify context. send the names instead
    id: Uuid,
}

impl PlayerInfo {
    pub fn new(id: Uuid) -> Self {
        PlayerInfo { id }
    }
}
