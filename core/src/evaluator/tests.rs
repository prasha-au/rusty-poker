use super::*;
use ntest::assert_true;

#[test]
fn create_a_fixed_array_from_card_vec() {
  let arr = cards_to_fixed_array(&vec![
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Heart, Rank::Three),
  ]);
  assert_eq!(arr, [0, 4, 255, 255, 255, 255, 255]);
}

#[test]
fn create_a_fixed_array_too_many_elements() {
  let arr = cards_to_fixed_array(&vec![
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Heart, Rank::Three),
    Card::new(Suit::Heart, Rank::Four),
    Card::new(Suit::Heart, Rank::Five),
    Card::new(Suit::Heart, Rank::Six),
    Card::new(Suit::Heart, Rank::Seven),
    Card::new(Suit::Heart, Rank::Eight),
    Card::new(Suit::Heart, Rank::Nine),
  ]);
  assert_eq!(arr, [0, 4, 8, 12, 16, 20, 24]);
}

#[test]
fn should_calculate_preflop_odds_unsuited() {
  let hand = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::Ace),
    Card::new(Suit::Spade, Rank::Ace),
  ]);
  assert_true!(chance_to_win_preflop(&hand, 2) > 0.0);
}

#[test]
fn should_calculate_preflop_odds_suited() {
  let hand = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::King),
    Card::new(Suit::Heart, Rank::Ace),
  ]);
  assert_true!(chance_to_win_preflop(&hand, 2) > 0.0);
}
