
use crate::card::*;

#[test]
fn conversion_from_u8() {
  assert_eq!(40/13, 3);
  assert_eq!(Card::new(Suit::Heart, Rank::Ten), Card::try_from(32).unwrap());
  assert_eq!(Card::new(Suit::Diamond, Rank::Five), Card::try_from(13).unwrap());
  assert_eq!(Card::new(Suit::Club, Rank::Ace), Card::try_from(51).unwrap());
  assert_eq!(Card::new(Suit::Heart, Rank::Two), Card::try_from(0).unwrap());
}

#[test]
fn conversion_to_u8() {
  assert_eq!(1, u8::from(Card::new(Suit::Diamond, Rank::Two)));
  assert_eq!(51, u8::from(Card::new(Suit::Club, Rank::Ace)));
  assert_eq!(0, u8::from(Card::new(Suit::Heart, Rank::Two)));
}

#[test]
fn display_output() {
  assert_eq!(Card::new(Suit::Diamond, Rank::Jack).to_string(), "♦J");
  assert_eq!(Card::new(Suit::Heart, Rank::Queen).to_string(), "♥Q");
  assert_eq!(Card::new(Suit::Club, Rank::Ace).to_string(), "♣A");
  assert_eq!(Card::new(Suit::Spade, Rank::Four).to_string(), "♠4");
  assert_eq!(Card::new(Suit::Spade, Rank::Ten).to_string(), "♠10");
}

