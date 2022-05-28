
use crate::card::*;
use crate::deck::*;
use crate::hand::*;



#[test]
fn evaluate_a_royal_flush() {
  let mut deck = Deck::new();
  deck.add_card(Card::new(Suit::Heart, FaceValue::Ace));
  deck.add_card(Card::new(Suit::Heart, FaceValue::King));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Queen));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Jack));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Ten));
  assert_eq!(evaluate_deck(&deck), HandRank::RoyalFlush);
}

#[test]
fn evaluate_a_straight_flush() {
  let mut deck = Deck::new();
  deck.add_card(Card::new(Suit::Heart, FaceValue::Queen));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Jack));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Ten));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Nine));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Eight));
  deck.add_card(Card::new(Suit::Diamond, FaceValue::Six));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Ace));
  assert_eq!(evaluate_deck(&deck), HandRank::StraightFlush { high_card: FaceValue::Queen });
}

#[test]
fn evaluate_a_four_of_a_kind() {
  let mut deck = Deck::new();
  deck.add_card(Card::new(Suit::Heart, FaceValue::King));
  deck.add_card(Card::new(Suit::Diamond, FaceValue::King));
  deck.add_card(Card::new(Suit::Spade, FaceValue::King));
  deck.add_card(Card::new(Suit::Club, FaceValue::King));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Five));
  deck.add_card(Card::new(Suit::Diamond, FaceValue::Ten));
  deck.add_card(Card::new(Suit::Spade, FaceValue::Ace));
  assert_eq!(evaluate_deck(&deck), HandRank::FourOfAKind { high_card: FaceValue::King, kicker: FaceValue::Ace });
}

#[test]
fn evaluate_a_full_house() {
  let mut deck = Deck::new();
  deck.add_card(Card::new(Suit::Heart, FaceValue::King));
  deck.add_card(Card::new(Suit::Diamond, FaceValue::King));
  deck.add_card(Card::new(Suit::Spade, FaceValue::King));
  deck.add_card(Card::new(Suit::Club, FaceValue::Six));
  deck.add_card(Card::new(Suit::Diamond, FaceValue::Six));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Ten));
  deck.add_card(Card::new(Suit::Spade, FaceValue::Ace));
  assert_eq!(evaluate_deck(&deck), HandRank::FullHouse { three_high_card: FaceValue::King, two_high_card: FaceValue::Six });
}

#[test]
fn evaluate_a_flush() {
  let mut deck = Deck::new();
  deck.add_card(Card::new(Suit::Heart, FaceValue::Queen));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Ten));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Six));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Four));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Two));
  deck.add_card(Card::new(Suit::Diamond, FaceValue::Ace));
  deck.add_card(Card::new(Suit::Spade, FaceValue::Ace));
  assert_eq!(evaluate_deck(&deck), HandRank::Flush { high_card: FaceValue::Queen });
}

#[test]
fn evaluate_a_straight() {
  let mut deck = Deck::new();
  deck.add_card(Card::new(Suit::Heart, FaceValue::Six));
  deck.add_card(Card::new(Suit::Diamond, FaceValue::Five));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Four));
  deck.add_card(Card::new(Suit::Spade, FaceValue::Three));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Two));
  deck.add_card(Card::new(Suit::Spade, FaceValue::Jack));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Jack));
  assert_eq!(evaluate_deck(&deck), HandRank::Straight { high_card: FaceValue::Six });
}





