#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn all() -> [Self; 4] {
        [Self::Clubs, Self::Diamonds, Self::Hearts, Self::Spades]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn all() -> [Self; 13] {
        [
            Self::Two,
            Self::Three,
            Self::Four,
            Self::Five,
            Self::Six,
            Self::Seven,
            Self::Eight,
            Self::Nine,
            Self::Ten,
            Self::Jack,
            Self::Queen,
            Self::King,
            Self::Ace,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    pub fn get_suit(self) -> Suit {
        self.suit
    }

    pub fn get_rank(self) -> Rank {
        self.rank
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_construct_card_works() {
        assert_eq!(
            Card {
                suit: Suit::Hearts,
                rank: Rank::Two
            },
            Card::new(Suit::Hearts, Rank::Two)
        )
    }
}
