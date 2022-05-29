use std::fmt::Display;
use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
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

#[repr(u8)]
#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
pub enum FaceValue {
  Two = 0,
  Three = 1,
  Four = 2,
  Five = 3,
  Six = 4,
  Seven = 5,
  Eight = 6,
  Nine = 7,
  Ten = 8,
  Jack = 9,
  Queen = 10,
  King = 11,
  Ace = 12
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


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Card {
  pub suit: Suit,
  pub value: FaceValue,
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

// TODO: Fix this to stop panicing
impl TryFrom<u8> for Card {
  type Error = &'static str;
  fn try_from(num_value: u8) -> Result<Self, Self::Error> {

    let suit = match num_value / 16 {
      0 => Ok(Suit::Heart),
      1 => Ok(Suit::Diamond),
      2 => Ok(Suit::Spade),
      3 => Ok(Suit::Club),
      bad_value => Err(format!("Invalid suit {}", bad_value))
    };

    let value = match num_value % 16 {
      0 => Ok(FaceValue::Two),
      1 => Ok(FaceValue::Three),
      2 => Ok(FaceValue::Four),
      3 => Ok(FaceValue::Five),
      4 => Ok(FaceValue::Six),
      5 => Ok(FaceValue::Seven),
      6 => Ok(FaceValue::Eight),
      7 => Ok(FaceValue::Nine),
      8 => Ok(FaceValue::Ten),
      9 => Ok(FaceValue::Jack),
      10 => Ok(FaceValue::Queen),
      11 => Ok(FaceValue::King),
      12 => Ok(FaceValue::Ace),
      bad_value => Err(format!("Invalid face value {}", bad_value))
    };

    if suit.is_err() || value.is_err() {
      Err("Bad card value")
    } else {
      Ok(Card { suit: suit.unwrap(), value: value.unwrap() })
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
