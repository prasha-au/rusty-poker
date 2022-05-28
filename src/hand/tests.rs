
use crate::card::*;
use crate::deck::*;
use crate::hand::*;



#[test]
fn should_get_highest_straight_card_simple() {
  let res = get_straight_high_card(0b0001111100000);
  assert_eq!(res, Some(FaceValue::Jack));
}


#[test]
fn should_get_highest_straight_card_wraparound() {
  let res = get_straight_high_card(0b1000000001111);
  assert_eq!(res, Some(FaceValue::Five));
}





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
  deck.add_card(Card::new(Suit::Spade, FaceValue::Two));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Five));
  assert_eq!(evaluate_deck(&deck), HandRank::Straight { high_card: FaceValue::Six });
}


#[test]
fn evaluate_a_straight_with_offset_highcard() {
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





