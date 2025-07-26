use rand::Rng;
use rand::distr::{Distribution, StandardUniform};

#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug)]

pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
}

impl Suit {
    pub fn random() -> Suit {
        let aa: Suit = rand::random();
        aa
    }

    pub fn translate(value: u8) -> Suit {
        let mut s: Suit = Suit::Heart;
        if value == 2 {
            s = Suit::Diamond;
        } else if value == 3 {
            s = Suit::Spade;
        } else if value == 4 {
            s = Suit::Club;
        }
        s
    }
}
impl Rank {
    pub fn random() -> Rank {
        let aa: Rank = rand::random();
        aa
    }

    pub fn translate(value: u8) -> Rank {
        let mut r: Rank = Rank::Ace;
        if value == 2 {
            r = 2  ;
        } else if value == 3 {
            r = Rank::(3);
        } else if value == 4 {
            r = 4;
        } else if value == 5 {
            r = 5;
        } else if value == 6 {
            r = 6;
        } else if value == 7 {
            r = 7;
        } else if value == 7 {
            r = 7;
        } else if value == 8 {
            r = 8;
        } else if value == 9 {
            r = 9;
        } else if value == 10 {
            r = 10;
        } else if value == 11 {
            r = Rank::Jack;
        } else if value == 12 {
            r = Rank::Queen;
        } else if value == 13 {
            r = Rank::King;
        }
        r
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

// pub fn winner_card(card: &Card) -> bool {}

impl Distribution<Rank> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rank {
        let index: u8 = rng.random_range(0..3);
        match index {
            0 => Rank::Ace,
            1 => Rank::King,
            2 => Rank::Queen,
            3 => Rank::Jack,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Suit> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Suit {
        let index: u8 = rng.random_range(0..3);
        match index {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => unreachable!(),
        }
    }
}
