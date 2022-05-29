

use crate::Card;
use crate::deck::*;



#[test]
fn adding_card_to_deck() {
  let mut deck = Deck::new();
  deck.add_card(Card::try_from(8).unwrap());
  assert_eq!(deck.value, 1 << 8);
}

#[test]
fn adding_multiple_cards_to_deck() {
  let mut deck = Deck::new();
  deck.add_card(Card::try_from(8).unwrap());
  deck.add_card(Card::try_from(50).unwrap());
  assert_eq!(deck.value, (1 << 8) | (1 << 50));
}

#[test]
fn adding_same_cards_to_deck() {
  let mut deck = Deck::new();
  deck.add_card(Card::try_from(8).unwrap());
  deck.add_card(Card::try_from(8).unwrap());
  assert_eq!(deck.value, (1 << 8));
}


#[test]
fn removing_card_from_deck() {
  let mut deck = Deck::new();
  deck.value = (1 << 5) | (1 << 8);
  deck.remove_card(Card::try_from(5).unwrap());
  assert_eq!(deck.value, (1 << 8));
}


#[test]
fn has_card_returns_true() {
  let mut deck = Deck::new();
  deck.value = 1 << 5;
  assert!(deck.has_card(Card::try_from(5).unwrap()));
}

#[test]
fn has_card_returns_false() {
  let deck = Deck::new();
  assert!(!deck.has_card(Card::try_from(5).unwrap()));
}


#[test]
fn deck_addition() {
  let mut deck_a = Deck::new();
  deck_a.add_card(Card::try_from(8).unwrap());
  let mut deck_b = Deck::new();
  deck_b.add_card(Card::try_from(16).unwrap());

  let combined_deck = deck_a + deck_b;
  assert_eq!(combined_deck.value, (1 << 8 | 1 << 16));
}


#[test]
fn deck_addition_with_overlap() {
  let mut deck_a = Deck::new();
  deck_a.add_card(Card::try_from(8).unwrap());
  let mut deck_b = Deck::new();
  deck_b.add_card(Card::try_from(8).unwrap());

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


#[test]
fn create_from_cards() {
  let cards = vec![Card::try_from(8).unwrap(), Card::try_from(18).unwrap()];
  let deck = Deck::from_cards(cards);
  assert_eq!(deck.value, (1 << 8) | (1 << 18));
}


#[test]
fn pick_available_card() {
  let mut cards = Vec::<Card>::new();
  for s in Suit::iter() {
    for fv in FaceValue::iter() {
      if s != Suit::Spade || fv != FaceValue::Three {
        cards.push(Card::new(s, fv));
      }
    }
  }
  let mut deck = Deck::from_cards(cards);
  assert_eq!(deck.pick_available_card(), Card::new(Suit::Spade, FaceValue::Three));
}


