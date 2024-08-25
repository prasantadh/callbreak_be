#![allow(unused)]
use crate::model::{hand, Card, Rank, Suit, Trick};
use crate::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardState {
    Playable,
    NotPlayable,
}

#[derive(Debug, Clone)]
pub struct Hand {
    cards: [(Card, CardState); 13],
}

impl Hand {
    pub fn is_valid(value: &[Card]) -> Result<()> {
        // the vector must have exactly 13 cards
        match value.len() {
            v if v < 13 => return Err(Error::NotEnoughCardsForHand),
            v if v > 13 => return Err(Error::TooManyCardsForHand),
            _ => (),
        };

        // a hand must have a spade
        if value
            .iter()
            .filter(|card| card.get_suit() == Suit::Spades)
            .count()
            == 0
        {
            return Err(Error::SpadeNotInHand);
        }

        // a hand must have one of Ace, Jack, Queen, King
        if value
            .iter()
            .filter(|card| card.get_rank() >= Rank::Jack)
            .count()
            == 0
        {
            return Err(Error::FaceCardNotInHand);
        }

        // a hand must not have duplicate cards
        for i in 0..13 {
            for j in (i + 1)..13 {
                if value[i] == value[j] {
                    return Err(Error::DuplicateCardInHand);
                }
            }
        }

        Ok(())
    }

    pub fn get_valid_card_for(&self, trick: &Trick) -> Result<Vec<Card>> {
        // TODO: should we handle the error when the trick is full?
        let playables = self
            .cards
            .iter()
            .filter_map(|v| {
                if v.1 == CardState::Playable {
                    Some(v.0)
                } else {
                    None
                }
            })
            .collect::<Vec<Card>>();

        trick.next()?; // check there is space to play in the trick
        let starter = match trick.starter() {
            None => return Ok(playables),
            Some(v) => v,
        };
        let winner = trick.winner()?.1;

        if starter.get_suit() != winner.get_suit() {
            // only possible when starter is Clubs, Diamonds, Hearts and winner is Spades.
            // Play any respective if available, else winning Spade, else anything
            let starter_suit = playables
                .iter()
                .filter(|card| card.get_suit() == starter.get_suit())
                .cloned()
                .collect::<Vec<Card>>();
            if !starter_suit.is_empty() {
                return Ok(starter_suit);
            }

            let spade_winners = playables
                .iter()
                .filter(|card| {
                    card.get_suit() == Suit::Spades && card.get_rank() > winner.get_rank()
                })
                .cloned()
                .collect::<Vec<Card>>();
            if !spade_winners.is_empty() {
                return Ok(spade_winners);
            }
        } else {
            // play winning of the starting suit if available, else any of starting suit
            // else any spade, else anything
            let starter_suit_winners = playables
                .iter()
                .filter(|card| {
                    card.get_suit() == winner.get_suit() && card.get_rank() > winner.get_rank()
                })
                .cloned()
                .collect::<Vec<Card>>();
            if !starter_suit_winners.is_empty() {
                return Ok(starter_suit_winners);
            }

            // TODO: this is duplicated code, refactor
            let starter_suit = playables
                .iter()
                .filter(|card| card.get_suit() == starter.get_suit())
                .cloned()
                .collect::<Vec<Card>>();
            if !starter_suit.is_empty() {
                return Ok(starter_suit);
            }

            let spades = playables
                .iter()
                .filter(|card| card.get_suit() == Suit::Spades)
                .cloned()
                .collect::<Vec<Card>>();
            if !spades.is_empty() {
                return Ok(spades);
            }
        }

        Ok(playables)
    }

    //    pub fn get_cards(&self) -> &[Card; 13] {
    //        &self.cards
    //    }
}

impl TryFrom<&Vec<Card>> for Hand {
    type Error = Error;
    fn try_from(value: &Vec<Card>) -> std::result::Result<Self, Self::Error> {
        Hand::is_valid(value)?;

        let mut hand = Hand {
            cards: [(Card::new(Suit::Hearts, Rank::Ace), CardState::NotPlayable); 13],
        };
        for (idx, card) in value.iter().enumerate() {
            hand.cards[idx] = (*card, CardState::Playable);
        }
        Ok(hand)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Call(usize);

impl TryFrom<usize> for Call {
    type Error = Error;
    fn try_from(value: usize) -> std::prelude::v1::Result<Self, Self::Error> {
        match value {
            _ if value < 1 => Err(Error::CallValueTooLow(value)),
            _ if value > 8 => Err(Error::CallValueTooHigh(value)),
            _ => Ok(Call(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Turn;

    use super::*;

    #[test]
    fn test_call_try_from_usize_works() {
        assert_eq!(Call::try_from(5), Ok(Call(5)));
        assert_eq!(Call::try_from(0), Err(Error::CallValueTooLow(0)));
        assert_eq!(Call::try_from(9), Err(Error::CallValueTooHigh(9)));
    }

    #[test]
    fn test_get_valid_card_for_same_suit_starter_winner_works() {
        let two_of_hearts = Card::new(Suit::Hearts, Rank::Two);
        let five_of_hearts = Card::new(Suit::Hearts, Rank::Five);
        let seven_of_hearts = Card::new(Suit::Hearts, Rank::Seven);
        let nine_of_hearts = Card::new(Suit::Hearts, Rank::Nine);
        let queen_of_hearts = Card::new(Suit::Hearts, Rank::Queen);
        let king_of_hearts = Card::new(Suit::Hearts, Rank::King);
        let two_of_spades = Card::new(Suit::Spades, Rank::Two);
        let five_of_spades = Card::new(Suit::Spades, Rank::Five);
        let seven_of_spades = Card::new(Suit::Spades, Rank::Seven);
        let nine_of_spades = Card::new(Suit::Spades, Rank::Nine);
        let queen_of_spades = Card::new(Suit::Spades, Rank::Queen);
        let king_of_spades = Card::new(Suit::Spades, Rank::King);
        let king_of_clubs = Card::new(Suit::Clubs, Rank::King);

        let mut hand: Vec<Card> = vec![];
        let hand = Hand::try_from(
            vec![
                two_of_hearts,
                five_of_hearts,
                seven_of_hearts,
                nine_of_hearts,
                queen_of_hearts,
                king_of_hearts,
                two_of_spades,
                five_of_spades,
                seven_of_spades,
                nine_of_spades,
                queen_of_spades,
                king_of_spades,
                king_of_clubs,
            ]
            .as_ref(),
        )
        .unwrap();

        let mut trick = Trick::new(Turn::try_from(0).unwrap());
        trick.add(Card::new(Suit::Hearts, Rank::Three)).unwrap();
        trick.add(Card::new(Suit::Hearts, Rank::Ten)).unwrap();
        let ans = hand.get_valid_card_for(&trick).unwrap();
        assert_eq!(ans, vec![queen_of_hearts, king_of_hearts]);

        let mut trick = Trick::new(Turn::try_from(0).unwrap());
        trick.add(Card::new(Suit::Hearts, Rank::Three)).unwrap();
        trick.add(Card::new(Suit::Hearts, Rank::Ace)).unwrap();
        let ans = hand.get_valid_card_for(&trick).unwrap();
        assert_eq!(
            ans,
            vec![
                two_of_hearts,
                five_of_hearts,
                seven_of_hearts,
                nine_of_hearts,
                queen_of_hearts,
                king_of_hearts
            ]
        );

        let mut trick = Trick::new(Turn::try_from(0).unwrap());
        trick.add(Card::new(Suit::Diamonds, Rank::Three)).unwrap();
        trick.add(Card::new(Suit::Diamonds, Rank::Ace)).unwrap();
        let ans = hand.get_valid_card_for(&trick).unwrap();
        assert_eq!(
            ans,
            vec![
                two_of_spades,
                five_of_spades,
                seven_of_spades,
                nine_of_spades,
                queen_of_spades,
                king_of_spades
            ]
        );

        let mut trick = Trick::new(Turn::try_from(0).unwrap());
        trick.add(Card::new(Suit::Diamonds, Rank::Three)).unwrap();
        trick.add(Card::new(Suit::Spades, Rank::Ten)).unwrap();
        let ans = hand.get_valid_card_for(&trick).unwrap();
        assert_eq!(ans, vec![queen_of_spades, king_of_spades]);
    }
}
