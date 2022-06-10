use strum_macros::EnumIter;

#[repr(u8)]
#[derive(Debug, PartialEq, EnumIter, Copy, Clone)]
pub enum Suit {
  Heart = 0,
  Diamond = 1,
  Spade = 2,
  Club = 3
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
pub enum Rank {
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


impl std::fmt::Display for Rank {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
     match *self {
        Rank::Two => write!(f, "2"),
        Rank::Three => write!(f, "3"),
        Rank::Four => write!(f, "4"),
        Rank::Five => write!(f, "5"),
        Rank::Six => write!(f, "6"),
        Rank::Seven => write!(f, "7"),
        Rank::Eight => write!(f, "8"),
        Rank::Nine => write!(f, "9"),
        Rank::Ten => write!(f, "T"),
        Rank::Jack => write!(f, "J"),
        Rank::Queen => write!(f, "Q"),
        Rank::King => write!(f, "K"),
        Rank::Ace => write!(f, "A"),
     }
  }
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Card {
  pub suit: Suit,
  pub rank: Rank,
}


impl std::fmt::Display for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "{}{}", self.suit, self.rank)
  }
}


impl Card {
  pub fn new(suit: Suit, rank: Rank) -> Card {
    Card { rank, suit }
  }
}


impl TryFrom<u8> for Card {
  type Error = &'static str;
  fn try_from(num_value: u8) -> Result<Self, Self::Error> {

    let suit = match num_value % 4 {
      0 => Ok(Suit::Heart),
      1 => Ok(Suit::Diamond),
      2 => Ok(Suit::Spade),
      3 => Ok(Suit::Club),
      bad_value => Err(format!("Invalid suit {}", bad_value))
    };

    let value = match num_value / 4 {
      0 => Ok(Rank::Two),
      1 => Ok(Rank::Three),
      2 => Ok(Rank::Four),
      3 => Ok(Rank::Five),
      4 => Ok(Rank::Six),
      5 => Ok(Rank::Seven),
      6 => Ok(Rank::Eight),
      7 => Ok(Rank::Nine),
      8 => Ok(Rank::Ten),
      9 => Ok(Rank::Jack),
      10 => Ok(Rank::Queen),
      11 => Ok(Rank::King),
      12 => Ok(Rank::Ace),
      bad_value => Err(format!("Invalid face value {}", bad_value))
    };

    if suit.is_err() || value.is_err() {
      Err("Bad card value")
    } else {
      Ok(Card { suit: suit.unwrap(), rank: value.unwrap() })
    }
  }
}

impl From<Card> for u8 {
  fn from(item: Card) -> Self {
    item.rank as u8 * 4 + item.suit as u8
  }
}


#[cfg(test)]
mod tests;
