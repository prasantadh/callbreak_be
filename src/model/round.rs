#![allow(unused)]

use crate::model::deck::Deck;
use crate::model::{Call, Card, Hand, Trick};
use crate::{Error, Result};

use super::Turn;

#[derive(Debug, Clone, Copy)]
pub struct RoundId(usize);

impl TryFrom<usize> for RoundId {
    type Error = Error;
    fn try_from(value: usize) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            _ if value > 4 => Err(Error::RoundIdTooLarge),
            _ => Ok(RoundId(value)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Call,
    Break,
}

#[derive(Debug, Clone)]
pub struct Round {
    id: RoundId,
    hands: Vec<Hand>,
    state: State,
    calls: [Option<Call>; 4],
    tricks: Vec<Trick>,
}

impl Round {
    pub fn new(id: RoundId) -> Self {
        let mut deck = Deck::new();
        let hands = deck.deal();
        Round {
            id,
            hands,
            state: State::Call,
            calls: [None; 4],
            tricks: vec![],
        }
    }

    // returns what is next (Call/Break)
    // and who is next
    // and sets up the insfrasture (if needed) to perform the next action
    pub fn next(&mut self) -> Result<Turn> {
        for i in 0..4 {
            let next = (self.id.0 + i) % 4;
            if self.calls[next].is_none() && i == 3 {
                self.state = State::Break;
                return Turn::try_from(next);
            } else if self.calls[next].is_none() {
                return Turn::try_from(next);
            }
        }

        let len = self.tricks.len();
        match len {
            0 => {
                let next = Turn::try_from(self.id.0 % 4)?;
                self.tricks.push(Trick::new(next));
                Ok(next)
            }
            v if v < 12 => {
                if self.tricks[v].is_full() {
                    let winner = self.tricks[v - 1].winner()?.0;
                    self.tricks.push(Trick::new(winner));
                    return Ok(winner);
                }
                Ok(self.tricks[v].next()?)
            }
            12 => {
                if self.tricks[12].is_full() {
                    return Err(Error::RoundOver);
                }
                Ok(self.tricks[12].next()?)
            }
            _ => Err(Error::RoundOver),
        }
    }

    pub fn make_a_call(&mut self, turn: Turn, call: Call) -> Result<()> {
        if self.state == State::Call {
            return Err(Error::RoundIsNotCalling);
        }
        if self.next()? != turn {
            return Err(Error::NotYourTurn);
        }
        self.calls[turn.get_value()] = Some(call);
        Ok(())
    }

    pub fn make_a_break(&mut self, turn: Turn, card: Card) -> Result<()> {
        if self.state != State::Break {
            return Err(Error::RoundIsNotBreaking);
        }
        if self.next()? != turn {
            return Err(Error::NotYourTurn);
        }
        let trick = self.tricks.last().ok_or(Error::RoundExpectsTrick)?;
        let moves = self.hands[turn.get_value()].get_valid_card_for(trick)?;
        if !moves.contains(&card) {
            return Err(Error::InvalidCardForBreak);
        }
        let trick_id = self.tricks.len();
        self.tricks[trick_id - 1].add(card);
        Ok(())
    }
}
