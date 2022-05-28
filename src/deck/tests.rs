

use crate::Card;
use crate::deck::*;


#[test]
fn adding_card_to_deck() {
  let mut deck = Deck::new();
  deck.add_card(Card::from(8));
  assert_eq!(deck.value, 1 << 8);
}

#[test]
fn adding_multiple_cards_to_deck() {
  let mut deck = Deck::new();
  deck.add_card(Card::from(8));
  deck.add_card(Card::from(50));
  assert_eq!(deck.value, (1 << 8) | (1 << 50));
}

#[test]
fn adding_same_cards_to_deck() {
  let mut deck = Deck::new();
  deck.add_card(Card::from(8));
  deck.add_card(Card::from(8));
  assert_eq!(deck.value, (1 << 8));
}


#[test]
fn removing_card_from_deck() {
  let mut deck = Deck::new();
  deck.value = (1 << 5) | (1 << 8);
  deck.remove_card(Card::from(5));
  assert_eq!(deck.value, (1 << 8));
}


#[test]
fn has_card_returns_true() {
  let mut deck = Deck::new();
  deck.value = 1 << 5;
  assert!(deck.has_card(Card::from(5)));
}

#[test]
fn has_card_returns_false() {
  let deck = Deck::new();
  assert!(!deck.has_card(Card::from(5)));
}


#[test]
fn deck_addition() {
  let mut deck_a = Deck::new();
  deck_a.add_card(Card::from(8));
  let mut deck_b = Deck::new();
  deck_b.add_card(Card::from(16));

  let combined_deck = deck_a + deck_b;
  assert_eq!(combined_deck.value, (1 << 8 | 1 << 16));
}


#[test]
fn deck_addition_with_overlap() {
  let mut deck_a = Deck::new();
  deck_a.add_card(Card::from(8));
  let mut deck_b = Deck::new();
  deck_b.add_card(Card::from(8));

  let combined_deck = deck_a + deck_b;
  assert_eq!(combined_deck.value, 1 << 8);
}



#[test]
fn get_suit() {
  let mut deck = Deck::new();
  deck.add_card(Card::new(Suit::Spade, FaceValue::Three));
  deck.add_card(Card::new(Suit::Spade, FaceValue::Four));

  assert_eq!(deck.get_suit(Suit::Spade), 0b000000000000110);
}



