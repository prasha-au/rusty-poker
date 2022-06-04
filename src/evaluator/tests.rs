use crate::evaluator::*;
use ntest::{timeout,assert_about_eq};
use crate::evaluator::two_plus_two::init_two_plus_two_table;


#[test]
#[timeout(200)]

fn init_two_plus_two_table_should_be_spammable() {
  for _ in 0..10000 {
    init_two_plus_two_table();
  }
}

#[test]
fn should_calculate_odds_of_winning() {
  init_two_plus_two_table();
  let table = Deck::from_cards(&vec!(
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Heart, Rank::Three),
    Card::new(Suit::Diamond, Rank::Four),
    Card::new(Suit::Diamond, Rank::Ten),
    Card::new(Suit::Diamond, Rank::Jack),
  ));
  let player = Deck::from_cards(&vec!(
    Card::new(Suit::Diamond, Rank::Queen),
    Card::new(Suit::Diamond, Rank::King),
  ));
  assert_about_eq!(0.9929293, chance_to_win(&table, &player));
}


#[test]
fn should_always_win_on_royal_flush() {
  init_two_plus_two_table();
  let table = Deck::from_cards(&vec!(
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Heart, Rank::Three),
    Card::new(Suit::Diamond, Rank::Ace),
    Card::new(Suit::Diamond, Rank::Ten),
    Card::new(Suit::Diamond, Rank::Jack),
  ));
  let player = Deck::from_cards(&vec!(
    Card::new(Suit::Diamond, Rank::Queen),
    Card::new(Suit::Diamond, Rank::King),
  ));
  assert_about_eq!(1f32, chance_to_win(&table, &player));
}


#[test]
fn create_a_fixed_array_from_card_vec() {
  let arr = cards_to_fixed_array(&vec!(
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Heart, Rank::Three),
  ));
  assert_eq!(arr, [0, 4, 255, 255, 255, 255, 255]);
}


#[test]
fn create_a_fixed_array_too_many_elements() {
  let arr = cards_to_fixed_array(&vec!(
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Heart, Rank::Three),
    Card::new(Suit::Heart, Rank::Four),
    Card::new(Suit::Heart, Rank::Five),
    Card::new(Suit::Heart, Rank::Six),
    Card::new(Suit::Heart, Rank::Seven),
    Card::new(Suit::Heart, Rank::Eight),
    Card::new(Suit::Heart, Rank::Nine),
  ));
  assert_eq!(arr, [0, 4, 8, 12, 16, 20, 24]);
}

