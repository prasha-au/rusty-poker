use std::fs::File;
use byteorder::{ReadBytesExt, LittleEndian};
use crate::card::*;


const TABLE_SIZE: usize = 32487834;

static mut HAND_RANKS: [u32; TABLE_SIZE] = [0 as u32; TABLE_SIZE];


pub fn init_tables() {
  let mut file = File::open("HandRanks.dat").expect("File not found");
  unsafe {
    file.read_u32_into::<LittleEndian>(&mut HAND_RANKS).expect("Could not read the file.");
  }
}


pub fn evaluate_hand_raw(cards: [u8; 7]) -> u32 {
  let mut p;
  unsafe {
    p = &HAND_RANKS[53 + u8::from(cards[0]) as usize];
    for i in 1..=6 {
      p = &HAND_RANKS[(*p as usize) + u8::from(cards[i]) as usize];
    }
  }
  *p
}

pub fn evaluate_hand(cards: &[Card; 7]) -> u32 {
  let card_values = cards.map(|c| ((c.suit as u8 / 13) + 1) * (c.rank as u8 + 1));
  evaluate_hand_raw(card_values)
}





#[cfg(test)]
mod tests;

