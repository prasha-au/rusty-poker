use std::fs::File;
use byteorder::{ReadBytesExt, LittleEndian};
use std::sync::Once;

const TABLE_SIZE: usize = 32487834;

static mut HAND_RANKS: [u32; TABLE_SIZE] = [0 as u32; TABLE_SIZE];

static LOAD_RANKS_ONCE: Once = Once::new();

pub fn init_two_plus_two_table() {
  LOAD_RANKS_ONCE.call_once(|| {
    let mut file = File::open("HandRanks.dat").expect("File not found");
    unsafe {
      file.read_u32_into::<LittleEndian>(&mut HAND_RANKS).expect("Could not read the file.");
    }
  });
}


pub fn evaluate_two_plus_two(cards: [u8; 7]) -> u32 {
  init_two_plus_two_table();
  let mut p;
  unsafe {
    p = HAND_RANKS[53 + cards[0] as usize + 1];
    for i in 1..=6 {
      p = HAND_RANKS[(p as usize) + cards[i] as usize + 1];
    }
  }
  p
}
