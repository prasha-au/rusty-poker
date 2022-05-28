
use crate::card::*;

pub enum HandRank {
  RoyalFlush,
  StraightFlush { high_card: FaceValue },
  FourOfAKind { kicker: FaceValue },
  FullHouse { three_high_card: FaceValue, pair_high_card: FaceValue },
  Flush { high_card: FaceValue },
  Straight { high_card: FaceValue },
  ThreeOfAKind { high_card: FaceValue, kicker: FaceValue },
  TwoPairs { high_card: FaceValue, kicker: FaceValue },
  OnePair { high_card: FaceValue, kicker: FaceValue, kicker2: FaceValue, kicker3: FaceValue },
  HighCard { high_card: FaceValue, kicker: FaceValue, kicker2: FaceValue, kicker3: FaceValue, kicker4: FaceValue },
}



pub fn evaluate_deck() -> HandRank {







  HandRank::HighCard { high_card: FaceValue::Ace, kicker: FaceValue::Ace, kicker2: FaceValue::Ace, kicker3: FaceValue::Ace, kicker4: FaceValue::Ace }

}

