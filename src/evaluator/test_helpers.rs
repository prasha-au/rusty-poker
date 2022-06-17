
macro_rules! evaluator_correctness_tests {
  ($scorefn:expr, $handfn:expr) => {
    use crate::card::*;

    #[test]
    fn evaluator_identifies_royal_flush() {
      let cards = [
        u8::from(Card::new(Suit::Heart, Rank::Queen)),
        u8::from(Card::new(Suit::Heart, Rank::King)),
        u8::from(Card::new(Suit::Heart, Rank::Ace)),
        u8::from(Card::new(Suit::Heart, Rank::Jack)),
        u8::from(Card::new(Suit::Heart, Rank::Ten)),

        u8::from(Card::new(Suit::Diamond, Rank::Four)),
        u8::from(Card::new(Suit::Heart, Rank::Five)),
      ];
      assert_eq!(Hand::StraightFlush, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_identifies_straight_flush() {
      let cards = [
        u8::from(Card::new(Suit::Heart, Rank::Five)),
        u8::from(Card::new(Suit::Heart, Rank::Six)),
        u8::from(Card::new(Suit::Heart, Rank::Seven)),
        u8::from(Card::new(Suit::Heart, Rank::Eight)),
        u8::from(Card::new(Suit::Heart, Rank::Nine)),

        u8::from(Card::new(Suit::Spade, Rank::Four)),
        u8::from(Card::new(Suit::Club, Rank::Five)),
      ];
      assert_eq!(Hand::StraightFlush, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_identifies_straight_flush_low_end() {
      let cards = [
        u8::from(Card::new(Suit::Diamond, Rank::Ace)),
        u8::from(Card::new(Suit::Diamond, Rank::Two)),
        u8::from(Card::new(Suit::Diamond, Rank::Three)),
        u8::from(Card::new(Suit::Diamond, Rank::Four)),
        u8::from(Card::new(Suit::Diamond, Rank::Five)),

        u8::from(Card::new(Suit::Spade, Rank::Four)),
        u8::from(Card::new(Suit::Club, Rank::Five)),
      ];
      assert_eq!(Hand::StraightFlush, $handfn($scorefn(cards)));
    }


  }
}
pub(crate) use evaluator_correctness_tests;

