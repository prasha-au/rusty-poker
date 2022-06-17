
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
    fn evaluator_identifies_straight_flush_with_ace() {
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

    #[test]
    fn evaluator_identifies_four_of_a_kind() {
      let cards = [
        u8::from(Card::new(Suit::Diamond, Rank::Four)),
        u8::from(Card::new(Suit::Heart, Rank::Four)),
        u8::from(Card::new(Suit::Club, Rank::Four)),
        u8::from(Card::new(Suit::Spade, Rank::Four)),

        u8::from(Card::new(Suit::Diamond, Rank::Five)),
        u8::from(Card::new(Suit::Spade, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Five)),
      ];
      assert_eq!(Hand::FourOfAKind, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_uses_high_card_in_four_of_a_kind() {
      let winner = [
        u8::from(Card::new(Suit::Diamond, Rank::Four)),
        u8::from(Card::new(Suit::Heart, Rank::Four)),
        u8::from(Card::new(Suit::Club, Rank::Four)),
        u8::from(Card::new(Suit::Spade, Rank::Four)),

        u8::from(Card::new(Suit::Club, Rank::Queen)),
        u8::from(Card::new(Suit::Spade, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Five)),
      ];

      let mut loser = winner;
      loser[4] = u8::from(Card::new(Suit::Club, Rank::Jack));
      assert_eq!(true, $scorefn(winner) > $scorefn(loser));

      let mut equal_winner = winner;
      equal_winner[6] = u8::from(Card::new(Suit::Club, Rank::Two));
      assert_eq!($scorefn(winner), $scorefn(equal_winner));
    }

    #[test]
    fn evaluator_identifies_full_house() {
      let cards = [
        u8::from(Card::new(Suit::Diamond, Rank::Four)),
        u8::from(Card::new(Suit::Heart, Rank::Four)),
        u8::from(Card::new(Suit::Club, Rank::Four)),
        u8::from(Card::new(Suit::Spade, Rank::Seven)),
        u8::from(Card::new(Suit::Diamond, Rank::Seven)),

        u8::from(Card::new(Suit::Spade, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Five)),
      ];
      assert_eq!(Hand::FullHouse, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_identifies_flush() {
      let cards = [
        u8::from(Card::new(Suit::Heart, Rank::Two)),
        u8::from(Card::new(Suit::Heart, Rank::Four)),
        u8::from(Card::new(Suit::Heart, Rank::Six)),
        u8::from(Card::new(Suit::Heart, Rank::Eight)),
        u8::from(Card::new(Suit::Heart, Rank::Ten)),

        u8::from(Card::new(Suit::Spade, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Five)),
      ];
      assert_eq!(Hand::Flush, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_identifies_straight() {
      let cards = [
        u8::from(Card::new(Suit::Heart, Rank::Three)),
        u8::from(Card::new(Suit::Spade, Rank::Four)),
        u8::from(Card::new(Suit::Diamond, Rank::Five)),
        u8::from(Card::new(Suit::Club, Rank::Six)),
        u8::from(Card::new(Suit::Diamond, Rank::Seven)),

        u8::from(Card::new(Suit::Spade, Rank::Jack)),
        u8::from(Card::new(Suit::Club, Rank::Nine)),
      ];
      assert_eq!(Hand::Straight, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_identifies_straight_with_ace() {
      let cards = [
        u8::from(Card::new(Suit::Diamond, Rank::Ace)),
        u8::from(Card::new(Suit::Club, Rank::Two)),
        u8::from(Card::new(Suit::Heart, Rank::Three)),
        u8::from(Card::new(Suit::Spade, Rank::Four)),
        u8::from(Card::new(Suit::Diamond, Rank::Five)),

        u8::from(Card::new(Suit::Spade, Rank::Jack)),
        u8::from(Card::new(Suit::Club, Rank::Nine)),
      ];
      assert_eq!(Hand::Straight, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_identifies_three_of_a_kind() {
      let cards = [
        u8::from(Card::new(Suit::Diamond, Rank::Queen)),
        u8::from(Card::new(Suit::Club, Rank::Queen)),
        u8::from(Card::new(Suit::Heart, Rank::Queen)),

        u8::from(Card::new(Suit::Spade, Rank::Four)),
        u8::from(Card::new(Suit::Diamond, Rank::Five)),
        u8::from(Card::new(Suit::Spade, Rank::Eight)),
        u8::from(Card::new(Suit::Club, Rank::Ten)),
      ];
      assert_eq!(Hand::ThreeOfAKind, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_uses_high_cards_in_three_of_a_kind() {
      let winner = [
        u8::from(Card::new(Suit::Diamond, Rank::Four)),
        u8::from(Card::new(Suit::Heart, Rank::Four)),
        u8::from(Card::new(Suit::Club, Rank::Four)),

        u8::from(Card::new(Suit::Spade, Rank::Queen)),
        u8::from(Card::new(Suit::Diamond, Rank::Jack)),
        u8::from(Card::new(Suit::Heart, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Eight)),
      ];

      for i in 3..5 {
        let mut loser = winner;
        loser[i] = u8::from(Card::new(Suit::Club, Rank::Two));
        assert_eq!(true, $scorefn(winner) > $scorefn(loser));
      }
    }

    #[test]
    fn evaluator_identifies_two_pairs() {
      let cards = [
        u8::from(Card::new(Suit::Diamond, Rank::Queen)),
        u8::from(Card::new(Suit::Club, Rank::Queen)),
        u8::from(Card::new(Suit::Heart, Rank::Jack)),
        u8::from(Card::new(Suit::Spade, Rank::Jack)),

        u8::from(Card::new(Suit::Club, Rank::Two)),
        u8::from(Card::new(Suit::Diamond, Rank::Five)),
        u8::from(Card::new(Suit::Spade, Rank::Eight)),
      ];
      assert_eq!(Hand::TwoPairs, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_uses_high_card_in_two_pairs() {
      let winner = [
        u8::from(Card::new(Suit::Diamond, Rank::Queen)),
        u8::from(Card::new(Suit::Club, Rank::Queen)),
        u8::from(Card::new(Suit::Heart, Rank::Jack)),
        u8::from(Card::new(Suit::Spade, Rank::Jack)),

        u8::from(Card::new(Suit::Club, Rank::Nine)),
        u8::from(Card::new(Suit::Diamond, Rank::Eight)),
        u8::from(Card::new(Suit::Heart, Rank::Seven))
      ];

      let mut loser = winner;
      loser[4] = u8::from(Card::new(Suit::Club, Rank::Two));
      assert_eq!(true, $scorefn(winner) > $scorefn(loser));

      let mut equal_winner = winner;
      equal_winner[5] = u8::from(Card::new(Suit::Club, Rank::Two));
      assert_eq!($scorefn(winner), $scorefn(equal_winner));
    }

    #[test]
    fn evaluator_identifies_pair() {
      let cards = [
        u8::from(Card::new(Suit::Diamond, Rank::King)),
        u8::from(Card::new(Suit::Club, Rank::King)),

        u8::from(Card::new(Suit::Spade, Rank::Jack)),
        u8::from(Card::new(Suit::Heart, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Seven)),
        u8::from(Card::new(Suit::Diamond, Rank::Five)),
        u8::from(Card::new(Suit::Spade, Rank::Two)),
      ];
      assert_eq!(Hand::Pair, $handfn($scorefn(cards)));
    }

    #[test]
    fn evaluator_uses_high_cards_in_pair() {
      let winner = [
        u8::from(Card::new(Suit::Diamond, Rank::King)),
        u8::from(Card::new(Suit::Club, Rank::King)),

        u8::from(Card::new(Suit::Spade, Rank::Jack)),
        u8::from(Card::new(Suit::Heart, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Seven)),
        u8::from(Card::new(Suit::Diamond, Rank::Five)),
        u8::from(Card::new(Suit::Spade, Rank::Four)),
      ];

      for i in 2..5 {
        let mut loser = winner;
        loser[i]= u8::from(Card::new(Suit::Diamond, Rank::Two));
        assert_eq!(true, $scorefn(winner) > $scorefn(loser));
      }
    }

    #[test]
    fn evaluator_identifies_high_card() {
      let cards = [
        u8::from(Card::new(Suit::Diamond, Rank::King)),
        u8::from(Card::new(Suit::Club, Rank::Queen)),
        u8::from(Card::new(Suit::Spade, Rank::Jack)),
        u8::from(Card::new(Suit::Heart, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Seven)),
        u8::from(Card::new(Suit::Diamond, Rank::Six)),
        u8::from(Card::new(Suit::Spade, Rank::Five)),
      ];
      assert_eq!(Hand::HighCard, $handfn($scorefn(cards)));
    }


    #[test]

    fn evaluator_uses_kickers_on_high_card() {
      let winner = [
        u8::from(Card::new(Suit::Diamond, Rank::King)),
        u8::from(Card::new(Suit::Club, Rank::Queen)),
        u8::from(Card::new(Suit::Spade, Rank::Jack)),
        u8::from(Card::new(Suit::Heart, Rank::Nine)),
        u8::from(Card::new(Suit::Club, Rank::Eight)),
        u8::from(Card::new(Suit::Diamond, Rank::Five)),
        u8::from(Card::new(Suit::Spade, Rank::Four)),
      ];
      for i in 0..5 {
        let mut loser = winner;
        loser[i]= u8::from(Card::new(Suit::Diamond, Rank::Two));
        assert_eq!(true, $scorefn(winner) > $scorefn(loser));
      }
      let mut equal_winner = winner;
      equal_winner[5] = u8::from(Card::new(Suit::Diamond, Rank::Two));
      assert_eq!($scorefn(winner), $scorefn(equal_winner));
    }


  }
}
pub(crate) use evaluator_correctness_tests;

