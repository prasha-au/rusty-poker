use super::types::Hand;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::sync::Once;

const TABLE_SIZE: usize = 32487834;

static mut HAND_RANKS: [u32; TABLE_SIZE] = [0 as u32; TABLE_SIZE];

static LOAD_RANKS_ONCE: Once = Once::new();

fn init_two_plus_two_table() {
  LOAD_RANKS_ONCE.call_once(|| {
    let mut file = File::open("../HandRanks.dat").expect("File not found");
    unsafe {
      file
        .read_u32_into::<LittleEndian>(&mut HAND_RANKS)
        .expect("Could not read the file.");
    }
  });
}

pub fn evaluate_score(cards: [u8; 7]) -> u16 {
  init_two_plus_two_table();
  let mut p;
  unsafe {
    p = HAND_RANKS[53 + cards[0] as usize + 1];
    for i in 1..=6 {
      p = HAND_RANKS[(p as usize) + cards[i] as usize + 1];
    }
  }
  u16::try_from(p).unwrap_or_default()
}

pub fn score_to_hand(score: u16) -> Hand {
  Hand::from((score >> 12 & 0xF) as u8)
}

#[cfg(test)]
mod tests;
