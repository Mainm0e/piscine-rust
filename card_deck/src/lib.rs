// todo: card_deck
/*
Instructions
A standard deck of cards has 52 cards: 4 suits with 13 cards per suit. Represent the cards from a deck:

Create an enum to represent the Suit.
Implement the associated function random, which returns a random Suit (Heart, Diamond, Spade or Club).
Create a Rank enum. For ace and face cards, it can be one of Ace, King, Queen or Jack. For the values from 2 to 10, 
it can have a Number value associated to a u8.
Create an associated function to Rank called Random that returns a random Rank.
Create a structure name Card which has the fields suit and rank.
Define:

The associated function translate for Rank and Suit:
For Suit, translate converts an integer value (u8) to a suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club).
For Rank, translate converts an integer value (u8) to a rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King).
The associated function random for Rank and Suit returns a random Rank and Suit respectively.
Finally define the function winner_card which returns true if the card passed as an argument is an ace of spades.
Dependencies
rand = "0.3.14"

Expected Functions and Structures
pub enum Suit {
}

pub enum Rank {
}

impl Suit {
	pub fn random() -> Suit {
	}

	pub fn translate(value: u8) -> Suit {
	}
}

impl Rank {
	pub fn random() -> Rank {
	}

	pub fn translate(value: u8) -> Rank {
	}
}

pub struct Card {
	pub suit: Suit,
	pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
} */


extern crate rand;
use rand::Rng;
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
    King,
    Queen,
    Jack,
    Number(u8),
}


impl Suit {
	pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, 5);
        Suit::translate(value)

	}

	pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid value"),
        }
	}
}

impl Rank {
	pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1, 14);
        Rank::translate(value)
	}

	pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2 => Rank::Number(2),
            3 => Rank::Number(3),
            4 => Rank::Number(4),
            5 => Rank::Number(5),
            6 => Rank::Number(6),
            7 => Rank::Number(7),
            8 => Rank::Number(8),
            9 => Rank::Number(9),
            10 => Rank::Number(10),
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid value"),
        }
	}
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub fn winner_card(card: &Card) -> bool {
    match card {
        Card { rank: Rank::Ace , suit: Suit::Spade } => true,
        _ => false,
    }
}