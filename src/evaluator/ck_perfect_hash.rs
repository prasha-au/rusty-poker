mod tables;

use tables::*;
use super::types::{Hand};


const HASH_QUINARY_ARR_LEN: usize = 13;

fn hash_quinary(arr: &[u8; HASH_QUINARY_ARR_LEN]) -> u32 {
  let mut sum = 0u32;
  let mut k = 7u8;
  for i in 0..HASH_QUINARY_ARR_LEN {
    sum += HASH_CALC_TABLE[arr[i] as usize][HASH_QUINARY_ARR_LEN - i - 1][k as usize];
    k -= arr[i];
    if k <= 0 {
      break;
    }
  }
  sum
}


pub fn evaluate_score(cards: [u8; 7]) -> u16 {
  let suit_hash: u16 = cards.iter().map(|c| SUITBIT_BY_ID_TABLE[*c as usize]).sum();

  let suit_hash_value = SUITS_HASH_TABLE[suit_hash as usize] as usize;
  if suit_hash_value != 0 {
    let mut suit_binary = [0; 4];
    for c in cards {
      suit_binary[(c & 0x3) as usize] |= BINARIES_BY_ID_TABLE[c as usize];
    }
    return FLUSH_TABLE[suit_binary[suit_hash_value - 1] as usize];
  }

  let mut quinary: [u8; HASH_QUINARY_ARR_LEN] = [0; HASH_QUINARY_ARR_LEN];
  for c in cards {
    quinary[(c >> 2) as usize] += 1;
  }

  let hash = hash_quinary(&quinary);
  NOFLUSH_TABLE[hash as usize]
}




pub fn score_to_hand(score: u16) -> Hand {
  match score {
    1..=10 => Hand::StraightFlush,
    11..=166 => Hand::FourOfAKind,
    167..=322 => Hand::FullHouse,
    323..=1599 => Hand::Flush,
    1600..=1609 => Hand::Straight,
    1610..=2467 => Hand::ThreeOfAKind,
    2468..=3325 => Hand::TwoPairs,
    3326..=6185 => Hand::Pair,
    6186..=7462 => Hand::HighCard,
    _ => Hand::Invalid,
  }
}


#[cfg(test)]
mod tests;
