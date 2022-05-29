
use crate::card::*;

#[test]
fn conversion_from_u8() {
  assert_eq!(Card::new(Suit::Heart, FaceValue::Ten), Card::try_from(8).unwrap());
  assert_eq!(Card::new(Suit::Diamond, FaceValue::Five), Card::try_from(19).unwrap());
  assert_eq!(Card::new(Suit::Club, FaceValue::Ace), Card::try_from(60).unwrap());
  assert_eq!(Card::new(Suit::Heart, FaceValue::Two), Card::try_from(0).unwrap());
}

#[test]
fn conversion_to_u8() {
  assert_eq!(34, u8::from(Card::new(Suit::Spade, FaceValue::Four)));
  assert_eq!(16, u8::from(Card::new(Suit::Diamond, FaceValue::Two)));
  assert_eq!(60, u8::from(Card::new(Suit::Club, FaceValue::Ace)));
  assert_eq!(0, u8::from(Card::new(Suit::Heart, FaceValue::Two)));
}

#[test]
fn display_output() {
  assert_eq!(Card::new(Suit::Diamond, FaceValue::Jack).to_string(), "♦J");
  assert_eq!(Card::new(Suit::Heart, FaceValue::Queen).to_string(), "♥Q");
  assert_eq!(Card::new(Suit::Club, FaceValue::Ace).to_string(), "♣A");
  assert_eq!(Card::new(Suit::Spade, FaceValue::Four).to_string(), "♠4");
  assert_eq!(Card::new(Suit::Spade, FaceValue::Ten).to_string(), "♠10");
}

