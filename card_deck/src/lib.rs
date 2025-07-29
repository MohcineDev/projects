#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug, PartialEq)]

pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    King,
    Queen,
    Jack,
}

impl Suit {
    pub fn random() -> Suit {
        // let aa: u8 = rand::random_range(1..5);
        let aa: u8 =2;
//inclusive or exclu
        // let aa: Suit = rand::random();
        // println!("{:?}", aa);
        Suit::translate(aa)
    }

    pub fn translate(value: u8) -> Suit {
        let mut s: Suit = Suit::Heart;
        if value == 1 {
            s = Suit::Heart;
        }
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
        let aa: u8 = 1;
        // let aa: u8 = rand::random_range(1..14);
        Rank::translate(aa)
    }

    pub fn translate(value: u8) -> Rank {
        let mut r: Rank = Rank::Ace;
        if value == 2 {
            r = Rank::Two;
        } else if value == 3 {
            r = Rank::Three;
        } else if value == 4 {
            r = Rank::Four;
        } else if value == 5 {
            r = Rank::Five;
        } else if value == 6 {
            r = Rank::Six;
        } else if value == 7 {
            r = Rank::Seven;
        } else if value == 8 {
            r = Rank::Eight;
        } else if value == 9 {
            r = Rank::Nine;
        } else if value == 10 {
            r = Rank::Ten;
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

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    let mut res = false;
    if card.rank == Rank::Ace && card.suit == Suit::Spade {
        res = true;
    }
    res
}
 