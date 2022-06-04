
#[repr(u8)]
#[derive(Debug)]
pub enum Hand {
  Invalid = 0,
  HighCard = 1,
  Pair = 2,
  TwoPairs = 3,
  ThreeOfAKind = 4,
  Straight = 5,
  Flush = 6,
  FullHouse = 7,
  FourOfAKind = 8,
  StraightFlush = 9,
}



impl From<u8> for Hand {
  fn from(value: u8) -> Hand {
    match value {
      0 => Hand::Invalid,
      1 => Hand::HighCard,
      2 => Hand::Pair,
      3 => Hand::TwoPairs,
      4 => Hand::ThreeOfAKind,
      5 => Hand::Straight,
      6 => Hand::Flush,
      7 => Hand::FullHouse,
      8 => Hand::FourOfAKind,
      9 => Hand::StraightFlush,
      _ => Hand::Invalid,
    }
  }
}

