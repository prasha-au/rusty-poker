
use crate::card::*;
use crate::deck::Deck;
use std::cmp::Reverse;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq)]
pub enum HandRank {
  RoyalFlush,
  StraightFlush { high_card: FaceValue },
  FourOfAKind { high_card: FaceValue, kicker: FaceValue },
  FullHouse { three_high_card: FaceValue, two_high_card: FaceValue },
  // TODO: This needs all cards...
  Flush { high_card: FaceValue },
  Straight { high_card: FaceValue },
  ThreeOfAKind { high_card: FaceValue, kicker: FaceValue },
  TwoPairs { high_card: FaceValue, second_high_card: FaceValue, kicker: FaceValue },
  OnePair { high_card: FaceValue, kicker: FaceValue, kicker2: FaceValue, kicker3: FaceValue },
  HighCard { high_card: FaceValue, kicker: FaceValue, kicker2: FaceValue, kicker3: FaceValue, kicker4: FaceValue },
}


struct SuitCount {
  suit: Suit,
  count: u8,
}

struct FaceValueCount {
  face: FaceValue,
  count: u8,
}


fn count_bits_set(val: u16) -> u8 {
  let mut count = 0;
  for i in 0..16 {
    if (val & (1 << i)) != 0 {
      count += 1;
    }
  }
  count
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
  let mut counts = [
    SuitCount { suit: Suit::Diamond, count:count_bits_set(deck.get_suit(Suit::Diamond)) },
    SuitCount { suit: Suit::Club, count: count_bits_set(deck.get_suit(Suit::Club)) },
    SuitCount { suit: Suit::Heart, count: count_bits_set(deck.get_suit(Suit::Heart)) },
    SuitCount { suit: Suit::Spade, count: count_bits_set(deck.get_suit(Suit::Spade)) },
  ];
  counts.sort_by_key(|v| Reverse(v.count));
  counts
}


fn get_face_count(deck: &Deck) -> [FaceValueCount; 13] {
  let count_card_bits = |fv: FaceValue| -> u8 {
    let mut count = 0;
    for suit in Suit::iter() {
      count += if deck.has_card(Card::new(suit, fv)) { 1 } else { 0 };
    }
    count
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
  counts.sort_by_key(|v| Reverse(v.count));
  counts
}



// TODO: Fix this function - it should be an option...
fn get_biggest_face(suit_val: u16) -> FaceValue {
  for fv in FaceValue::iter().rev() {
    if suit_val & (1 << fv as u8) > 0 {
      return fv;
    }
  }
  FaceValue::Two
}







pub fn evaluate_deck(deck: &Deck) -> HandRank {

  let suit_counts = get_suit_count(deck);

  // Royal flush + straight flush
  if suit_counts[0].count >= 5 {
    let straight_high = get_straight_high_card(deck.get_suit(suit_counts[0].suit));
    match straight_high {
      Some(FaceValue::Ace) => return HandRank::RoyalFlush,
      Some(high_card) => return HandRank::StraightFlush { high_card },
      None => {},
    }
  }


  let face_counts = get_face_count(deck);


  // Four of a kind...
  if face_counts[0].count >= 4 {
    let kicker = deck.get_cards().iter()
      .filter(|v| v.value != face_counts[0].face)
      .fold(FaceValue::Two, |acc, c| if (c.value as u8) > (acc as u8) { c.value } else { acc });
    return HandRank::FourOfAKind { high_card: face_counts[0].face, kicker };
  }



  // Full house
  if face_counts[0].count == 3 && face_counts[1].count >= 2 {
    return HandRank::FullHouse { three_high_card: face_counts[0].face, two_high_card: face_counts[1].face };
  }


  // Flush
  if suit_counts[0].count >= 5 {
    return HandRank::Flush { high_card: get_biggest_face(deck.get_suit(suit_counts[0].suit)) };
  }

  // Straight
  let combined_suit = Suit::iter().map(|s| deck.get_suit(s)).fold(0, |acc, v| acc | v);
  match get_straight_high_card(combined_suit) {
    Some(high_card) => return HandRank::Straight { high_card },
    None => {}
  }

  // Three of a kind
  if face_counts[0].count >= 3 {
    let kicker = deck.get_cards().iter()
      .filter(|v| v.value != face_counts[0].face)
      .fold(FaceValue::Two, |acc, c| if (c.value as u8) > (acc as u8) { c.value } else { acc });
    return HandRank::ThreeOfAKind { high_card: face_counts[0].face, kicker };
  }


  // Two pairs
  if face_counts[0].count == 2 && face_counts[1].count == 2 {
    let kicker = deck.get_cards().iter()
      .filter(|v| v.value != face_counts[0].face && v.value != face_counts[1].face)
      .fold(FaceValue::Two, |acc, c| if (c.value as u8) > (acc as u8) { c.value } else { acc });
    return HandRank::TwoPairs { high_card: face_counts[0].face, second_high_card: face_counts[1].face, kicker };
  }

  // One pair
  // if face_counts[0].count == 2 {
  //   let kicker = deck.get_cards().iter()
  //     .filter(|v| v.value != face_counts[0].face && v.value != face_counts[1].face)
  //     .fold(FaceValue::Two, |acc, c| if (c.value as u8) > (acc as u8) { c.value } else { acc });
  //   return HandRank::OnePair { high_card: face_counts[0].face, kicker };
  // }



  return HandRank::HighCard { high_card: FaceValue::Ace, kicker: FaceValue::Ace, kicker2: FaceValue::Ace, kicker3: FaceValue::Ace, kicker4: FaceValue::Ace };

}


#[cfg(test)]
mod tests;
