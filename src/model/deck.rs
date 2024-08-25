#![allow(unused)]
use crate::model::{Card, Hand, Rank, Suit};
use crate::{Error, Result};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
    cards: [Card; 52],
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Deck {
            cards: [Card::new(Suit::Hearts, Rank::Two); 52],
        };
        let mut idx = 0;
        for suit in Suit::all() {
            for rank in Rank::all() {
                deck.cards[idx] = Card::new(suit, rank);
                idx += 1
            }
        }
        deck
    }

    pub fn deal(&mut self) -> Vec<Hand> {
        let mut hands: Vec<Hand> = vec![];
        'shuffle: loop {
            self.cards.shuffle(&mut thread_rng());
            hands = vec![];
            for i in (0..52).step_by(13) {
                let hand = match Hand::try_from(self.cards[i..(i + 13)].to_vec().as_ref()) {
                    Err(e) => continue 'shuffle,
                    Ok(v) => v,
                };
                hands.push(hand);
            }
            break;
        }
        hands
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_deck_has_all_cards() {
        let deck = Deck::new();
        for suit in Suit::all() {
            for rank in Rank::all() {
                let mut found = false;
                let card = Card::new(suit, rank);
                for deck_card in deck.cards {
                    if card == deck_card {
                        found = true;
                        break;
                    }
                }
                assert!(found)
            }
        }
    }

    #[test]
    fn test_deal_works() {
        let mut deck = Deck::new();
        let hands = deck.deal();
        assert!(hands.len() == 4);
        // for manual inspection
        // for hand in hands {
        //     for card in hand.get_cards() {
        //         println!("{:?}", card);
        //     }
        //     println!("==============================")
        // }
    }
}
