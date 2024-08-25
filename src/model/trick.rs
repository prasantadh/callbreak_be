#![allow(unused)]

use crate::model::{Card, Rank, Suit};
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Turn(usize);

impl Turn {
    pub fn get_value(&self) -> usize {
        self.0
    }
}

impl TryFrom<usize> for Turn {
    type Error = Error;
    fn try_from(value: usize) -> std::prelude::v1::Result<Self, Self::Error> {
        if value > 3 {
            return Err(Error::DuplicateCardInHand);
        }
        Ok(Turn(value))
    }
}

#[derive(Debug, Clone)]
pub struct Trick {
    start: Turn,
    cards: [Option<Card>; 4],
}

impl Trick {
    pub fn new(start: Turn) -> Self {
        Trick {
            start,
            cards: [None; 4],
        }
    }

    pub fn is_full(&self) -> bool {
        self.cards.iter().filter(|card| card.is_some()).count() == 4
    }

    pub fn is_empty(&self) -> bool {
        self.cards.iter().filter(|card| card.is_none()).count() == 4
    }

    pub fn next(&self) -> Result<Turn> {
        if self.is_full() {
            return Err(Error::TrickIsFull);
        }

        let mut next = self.start;
        while self.cards[next.0].is_some() {
            next = Turn::try_from((next.0 + 1) % 4)?;
        }
        Ok(next)
    }

    // we have two choices
    // provide add(card) as api or
    // provide add(turn, card) as api
    // it might be nice to verify turn here
    // however to verify card, something also needs access to hand
    // so it might be best left to the Round to figure this out and simply use add(card)
    pub fn add(&mut self, card: Card) -> Result<()> {
        let next = self.next()?;
        self.cards[next.0] = Some(card);
        Ok(())
    }

    pub fn winner(&self) -> Result<(Turn, Card)> {
        if self.is_empty() {
            return Err(Error::TrickIsEmpty);
        }

        let mut winner = (
            self.start,
            self.cards[self.start.0].ok_or(Error::TrickHasNoneCardSomeExpected)?,
        );
        for i in 1..=3 {
            let next = (self.start.0 + i) % 4;
            match self.cards[next] {
                None => return Ok(winner),
                Some(v) => {
                    if (v.get_suit() == winner.1.get_suit() && v.get_rank() > winner.1.get_rank())
                        || (v.get_suit() == Suit::Spades)
                    {
                        winner = (next.try_into()?, v);
                    }
                }
            }
        }

        Ok(winner)
    }

    pub fn starter(&self) -> Option<Card> {
        self.cards[self.start.0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_works() {
        let turn = Turn::try_from(2).unwrap();
        let mut trick = Trick::new(turn);
        let card0 = Card::new(Suit::Hearts, Rank::Two);
        let card1 = Card::new(Suit::Hearts, Rank::Three);
        let card2 = Card::new(Suit::Diamonds, Rank::Two);
        let card3 = Card::new(Suit::Spades, Rank::Three);
        trick.add(card0);
        assert_eq!(trick.cards, [None, None, Some(card0), None]);
        trick.add(card1);
        assert_eq!(trick.cards, [None, None, Some(card0), Some(card1)]);
        trick.add(card2);
        assert_eq!(trick.cards, [Some(card2), None, Some(card0), Some(card1)]);
        trick.add(card3);
        assert_eq!(
            trick.cards,
            [Some(card2), Some(card3), Some(card0), Some(card1)]
        );
    }

    #[test]
    fn test_winner_works_with_same_suit() {
        // set up a few cards
        let ten = Card::new(Suit::Diamonds, Rank::Ten);
        let queen = Card::new(Suit::Diamonds, Rank::Queen);
        let ace = Card::new(Suit::Diamonds, Rank::Ace);
        let two = Card::new(Suit::Diamonds, Rank::Two);

        // set up a trick
        let turn = Turn::try_from(1).unwrap();
        let mut trick = Trick::new(turn);
        // empty trick does not have a winner
        assert!(trick.winner().is_err());
        trick.add(ten);
        trick.add(queen);
        trick.add(ace);
        trick.add(two);
        // winner should now be ace
        let winner = trick.winner().unwrap();
        assert_eq!(winner, (Turn::try_from(3).unwrap(), ace));

        // try a diffent order just cause
        let turn = Turn::try_from(0).unwrap();
        let mut trick = Trick::new(turn);
        trick.add(ace);
        trick.add(queen);
        trick.add(two);
        trick.add(ten);
        let winner = trick.winner().unwrap();
        assert_eq!(winner, (Turn::try_from(0).unwrap(), ace));
    }

    #[test]
    fn test_winner_works_with_spade() {
        // set up a few cards
        let ace_of_clubs = Card::new(Suit::Clubs, Rank::Ace);
        let two_of_clubs = Card::new(Suit::Clubs, Rank::Two);
        let ace_of_diamonds = Card::new(Suit::Diamonds, Rank::Ace);
        let ace_of_hearts = Card::new(Suit::Hearts, Rank::Ace);
        let ace_of_spades = Card::new(Suit::Spades, Rank::Ace);
        let two_of_spades = Card::new(Suit::Spades, Rank::Two);

        // set up a trick
        let turn = Turn::try_from(0).unwrap();
        let mut trick = Trick::new(turn);
        trick.add(ace_of_hearts);
        trick.add(ace_of_clubs);
        trick.add(ace_of_diamonds);
        trick.add(two_of_spades);
        let winner = trick.winner().unwrap();
        assert_eq!(winner, (Turn::try_from(3).unwrap(), two_of_spades));

        // try a different order
        let turn = Turn::try_from(3).unwrap();
        let mut trick = Trick::new(turn);
        trick.add(two_of_spades);
        trick.add(ace_of_spades);
        trick.add(ace_of_clubs);
        trick.add(ace_of_diamonds);
        let winner = trick.winner().unwrap();
        assert_eq!(winner, (Turn::try_from(0).unwrap(), ace_of_spades));
    }
}
