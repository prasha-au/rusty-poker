mod utils;

use wasm_bindgen::prelude::*;
use rusty_poker_core::{evaluator, deck::Deck};


#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
  alert("Hello, rusty poker!");
}


#[wasm_bindgen]
pub fn enable_debug() {
  utils::set_panic_hook();
}


#[wasm_bindgen]
pub fn hand_score(table: u64, hand: u64) -> u16 {
  evaluator::get_hand_score(&Deck::from_value(table), &Deck::from_value(hand))
}


#[wasm_bindgen]
pub fn chance_to_win(table: u64, hand: u64) -> f32 {
  evaluator::chance_to_win(&Deck::from_value(table), &Deck::from_value(hand))
}

#[wasm_bindgen]
pub fn chance_to_win_preflop(hand: u64, num_players: u8) -> f32 {
  evaluator::chance_to_win_preflop(&Deck::from_value(hand), num_players)
}

