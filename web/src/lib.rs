mod utils;

use wasm_bindgen::prelude::*;
use rusty_poker_core::{evaluator, deck::Deck, game::{Game, GameState, BettingAction}, card::Card, player::Player};
use serde::{Serialize, Deserialize};

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



pub struct GameInstance {
  pub game: Game,
  // pub opponents: [*mut dyn Player; 8],
}


#[wasm_bindgen]
pub fn create_game() -> *mut GameInstance {
  Box::into_raw(Box::new(GameInstance {
    game: Game::create(4, 1000),
  }))
}







#[wasm_bindgen]
pub fn iterate_game(instance: *mut GameInstance) {
  unsafe { (*instance).game.next() };
}



#[derive(Serialize)]
pub struct PlayerState {
  pub is_folded: bool,
  pub wallet: u32,
  pub money_on_table: u32,
}

#[derive(Serialize)]
pub struct MyGameState {
  pub total_pot: u32,
  pub wallet: u32,
  pub value_to_call: u32,
  pub players: [Option<PlayerState>; 8],
  pub is_my_turn: bool,
  pub dealer_index: u8,
}


#[wasm_bindgen]
pub fn get_game_state(instance: *mut GameInstance) -> JsValue {
  let state = unsafe { (*instance).game.get_state(0) };
  serde_wasm_bindgen::to_value(&MyGameState {
    total_pot: state.total_pot,
    wallet: state.wallet,
    value_to_call: state.value_to_call,
    players: state.players.map(|p_opt| if let Some(p) = p_opt { Some(PlayerState {
      is_folded: p.is_folded,
      wallet: p.wallet,
      money_on_table: p.money_on_table,
    }) } else { None }),
    is_my_turn: state.current_player_index.is_some() && state.current_player_index.unwrap() == 0,
    dealer_index: state.dealer_index,
  }).unwrap()
}


#[derive(Deserialize)]
pub struct Action {
  pub action: String,
  pub amount: Option<u32>,
}


#[wasm_bindgen]
pub fn action_player(instance: *mut GameInstance, action: JsValue) {
  let action: Action = serde_wasm_bindgen::from_value(action).unwrap();
  let betting_action = match action.action.as_str() {
    "fold" => BettingAction::Fold,
    "call" => BettingAction::Call,
    "raise" => BettingAction::Raise(action.amount.unwrap()),
    "all_in" => BettingAction::AllIn(action.amount.unwrap()),
    _ => BettingAction::Fold,
  };
  unsafe { (*instance).game.action_current_player(betting_action) }.unwrap();
}

