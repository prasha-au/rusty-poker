
use crate::card::*;
use crate::deck::Deck;
use std::cmp::Reverse;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq)]
pub enum HandRank {
  RoyalFlush,
  StraightFlush { high_card: FaceValue },
  FourOfAKind { value: FaceValue, kicker: FaceValue },
  FullHouse { three_value: FaceValue, two_value: FaceValue },
  Flush { values: [FaceValue; 5] },
  Straight { high_card: FaceValue },
  ThreeOfAKind { value: FaceValue, kickers: [FaceValue; 2] },
  TwoPairs { high_value: FaceValue, second_value: FaceValue, kicker: FaceValue },
  OnePair { value: FaceValue, kickers: [FaceValue; 3] },
  HighCard { values: [FaceValue; 4] },
}


struct SuitCount {
  suit: Suit,
  count: u8,
}

struct FaceValueCount {
  face: FaceValue,
  count: u8,
}


fn get_straight_high_card(val: u16) -> Option<FaceValue> {
  let mut straight_count = if val & (1 << FaceValue::Ace as u8) > 0 { 1 } else { 0 };
  for fv in FaceValue::iter() {
    if val & (1 << fv as u8) > 0 {
      straight_count += 1;
    } else {
      straight_count = 0;
    }
    if straight_count == 5 {
      return Some(fv);
    }
  }
  None
}



fn get_suit_count(deck: &Deck) -> [SuitCount; 4] {
  let count_bits_set = |sv: u16| -> u8 {
    FaceValue::iter().fold(0, |acc, fv| { if sv & (1 << fv as u8) > 0 { acc + 1 } else { acc } })
  };
  let mut counts = [
    SuitCount { suit: Suit::Diamond, count:count_bits_set(deck.get_suit(Suit::Diamond)) },
    SuitCount { suit: Suit::Club, count: count_bits_set(deck.get_suit(Suit::Club)) },
    SuitCount { suit: Suit::Heart, count: count_bits_set(deck.get_suit(Suit::Heart)) },
    SuitCount { suit: Suit::Spade, count: count_bits_set(deck.get_suit(Suit::Spade)) },
  ];
  counts.sort_unstable_by_key(|v| Reverse(v.count));
  counts
}


fn get_face_count(deck: &Deck) -> [FaceValueCount; 13] {
  let count_card_bits = |fv: FaceValue| -> u8 {
    Suit::iter().fold(0, |acc, sv| { if deck.has_card(Card::new(sv, fv)) { acc + 1 } else { acc } })
  };
  let mut counts = [
    FaceValueCount { face: FaceValue::Two, count: count_card_bits(FaceValue::Two) },
    FaceValueCount { face: FaceValue::Three, count: count_card_bits(FaceValue::Three) },
    FaceValueCount { face: FaceValue::Four, count: count_card_bits(FaceValue::Four) },
    FaceValueCount { face: FaceValue::Five, count: count_card_bits(FaceValue::Five) },
    FaceValueCount { face: FaceValue::Six, count: count_card_bits(FaceValue::Six) },
    FaceValueCount { face: FaceValue::Seven, count: count_card_bits(FaceValue::Seven) },
    FaceValueCount { face: FaceValue::Eight, count: count_card_bits(FaceValue::Eight) },
    FaceValueCount { face: FaceValue::Nine, count: count_card_bits(FaceValue::Nine) },
    FaceValueCount { face: FaceValue::Ten, count: count_card_bits(FaceValue::Ten) },
    FaceValueCount { face: FaceValue::Jack, count: count_card_bits(FaceValue::Jack) },
    FaceValueCount { face: FaceValue::Queen, count: count_card_bits(FaceValue::Queen) },
    FaceValueCount { face: FaceValue::King, count: count_card_bits(FaceValue::King) },
    FaceValueCount { face: FaceValue::Ace, count: count_card_bits(FaceValue::Ace) },
  ];
  counts.sort_unstable_by_key(|v| Reverse(v.count * 16 + v.face as u8));
  counts
}


fn get_kickers<F: Fn(&Card) -> bool>(deck: &Deck, filter_fn: F) -> Vec<FaceValue> {
  let mut kickers = deck.get_cards().iter()
    .filter(|c| filter_fn(c))
    .map(|v| v.value)
    .collect::<Vec<FaceValue>>();
  kickers.sort_unstable_by_key(|v| Reverse(*v as u8));
  kickers
}




pub fn evaluate_deck(deck: &Deck) -> HandRank {

  let suit_counts = get_suit_count(deck);
  let face_counts = get_face_count(deck);



  // Royal flush + straight flush
  if suit_counts[0].count >= 5 {
    let straight_high = get_straight_high_card(deck.get_suit(suit_counts[0].suit));
    match straight_high {
      Some(FaceValue::Ace) => return HandRank::RoyalFlush,
      Some(high_card) => return HandRank::StraightFlush { high_card },
      None => {},
    }
  }


  // Four of a kind...
  if face_counts[0].count >= 4 {
    let kickers = get_kickers(deck, |c| c.value != face_counts[0].face);
    return HandRank::FourOfAKind { value: face_counts[0].face, kicker: kickers[0] };
  }



  // Full house
  if face_counts[0].count == 3 && face_counts[1].count >= 2 {
    return HandRank::FullHouse { three_value: face_counts[0].face, two_value: face_counts[1].face };
  }


  // Flush
  if suit_counts[0].count >= 5 {
    let high_cards = get_kickers(deck, |v: &Card| v.suit == suit_counts[0].suit);
    return HandRank::Flush { values: high_cards[0..5].try_into().expect("slice with incorrect length") };
  }

  // Straight
  let combined_suit = Suit::iter().map(|s| deck.get_suit(s)).fold(0, |acc, v| acc | v);
  match get_straight_high_card(combined_suit) {
    Some(high_card) => return HandRank::Straight { high_card },
    None => {}
  }

  // Three of a kind
  if face_counts[0].count >= 3 {
    let kickers = get_kickers(deck, |c| c.value != face_counts[0].face);
    return HandRank::ThreeOfAKind { value: face_counts[0].face, kickers: kickers[0..2].try_into().expect("slice with incorrect length") };
  }


  // Two pairs
  if face_counts[0].count == 2 && face_counts[1].count == 2 {
    let kickers = get_kickers(deck, |v| v.value != face_counts[0].face && v.value != face_counts[1].face);
    return HandRank::TwoPairs { high_value: face_counts[0].face, second_value: face_counts[1].face, kicker: kickers[0] };
  }

  // One pair
  if face_counts[0].count == 2 {
    let kickers = get_kickers(deck, |v| v.value != face_counts[0].face);
    return HandRank::OnePair { value: face_counts[0].face, kickers: kickers[0..3].try_into().expect("slice with incorrect length") };
  }



  let kickers = get_kickers(deck, |_| true);
  return HandRank::HighCard { values: kickers[0..=5].try_into().expect("Slice with incorrect length") };

}


#[cfg(test)]
mod tests;
