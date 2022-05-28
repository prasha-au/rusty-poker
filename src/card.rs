
use std::fmt::Display;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Suit {
  Heart = 0,
  Diamond = 16,
  Spade = 32,
  Club = 48
}


impl std::fmt::Display for Suit {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
     match *self {
         Suit::Heart => write!(f, "♥"),
         Suit::Diamond => write!(f, "♦"),
         Suit::Spade => write!(f, "♠"),
         Suit::Club => write!(f, "♣"),
     }
  }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum FaceValue {
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
  Ace
}


impl std::fmt::Display for FaceValue {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
     match *self {
        FaceValue::Two => write!(f, "2"),
        FaceValue::Three => write!(f, "3"),
        FaceValue::Four => write!(f, "4"),
        FaceValue::Five => write!(f, "5"),
        FaceValue::Six => write!(f, "6"),
        FaceValue::Seven => write!(f, "7"),
        FaceValue::Eight => write!(f, "8"),
        FaceValue::Nine => write!(f, "9"),
        FaceValue::Ten => write!(f, "10"),
        FaceValue::Jack => write!(f, "J"),
        FaceValue::Queen => write!(f, "Q"),
        FaceValue::King => write!(f, "K"),
        FaceValue::Ace => write!(f, "A"),
     }
  }
}


#[derive(Debug)]
#[derive(PartialEq)]
pub struct Card {
  suit: Suit,
  value: FaceValue,
}


impl Display for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}{}", self.suit, self.value)
  }
}



impl Card {
  pub fn new(suit: Suit, value: FaceValue) -> Card {
    Card { value, suit }
  }
}

impl From<u8> for Card {
  fn from(num_value: u8) -> Self {
    println!("Testvalue {}", num_value);
    Card {
      suit: match num_value / 16 {
        0 => Suit::Heart,
        1 => Suit::Diamond,
        2 => Suit::Spade,
        3 => Suit::Club,
        bad_value => panic!("Invalid suit {}", bad_value)
      },
      value: match num_value % 16 {
        0 => FaceValue::Two,
        1 => FaceValue::Three,
        2 => FaceValue::Four,
        3 => FaceValue::Five,
        4 => FaceValue::Six,
        5 => FaceValue::Seven,
        6 => FaceValue::Eight,
        7 => FaceValue::Nine,
        8 => FaceValue::Ten,
        9 => FaceValue::Jack,
        10 => FaceValue::Queen,
        11 => FaceValue::King,
        12 => FaceValue::Ace,
        bad_value => panic!("Invalid face value {}", bad_value)
      }
    }
  }
}

impl From<Card> for u8 {
  fn from(item: Card) -> Self {
    (item.suit as u8) + (item.value as u8)
  }
}

#[cfg(test)]
mod tests;
