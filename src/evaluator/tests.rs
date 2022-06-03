use crate::evaluator::*;




#[test]
fn should_be_able_to_load_the_table() {
  init_tables();

  let cards2 = [
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Diamond, Rank::Two),
    Card::new(Suit::Diamond, Rank::Three),
    Card::new(Suit::Club, Rank::Four),
    Card::new(Suit::Club, Rank::Jack),
    Card::new(Suit::Spade, Rank::Four),
    Card::new(Suit::Club, Rank::Queen)
  ];
  let card_values = cards2.map(|c| u8::from(c));
  for _ in 0..1000 {
    evaluate_hand_raw(card_values);
    // evaluate_hand(&cards2);
    // assert_ne!(v, 5);

  }

}
