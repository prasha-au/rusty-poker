use std::fs::File;
use byteorder::{ReadBytesExt, LittleEndian};
use crate::card::*;
use crate::deck::*;
use std::sync::Once;

const TABLE_SIZE: usize = 32487834;

static mut HAND_RANKS: [u32; TABLE_SIZE] = [0 as u32; TABLE_SIZE];

static LOAD_RANKS_ONCE: Once = Once::new();

pub fn init_tables() {
  LOAD_RANKS_ONCE.call_once(|| {
    let mut file = File::open("HandRanks.dat").expect("File not found");
    unsafe {
      file.read_u32_into::<LittleEndian>(&mut HAND_RANKS).expect("Could not read the file.");
    }
  });
}

fn evaluate_hand_raw(cards: [u8; 7]) -> u32 {
  let mut p;
  unsafe {
    p = HAND_RANKS[53 + cards[0] as usize + 1];
    for i in 1..=6 {
      p = HAND_RANKS[(p as usize) + cards[i] as usize + 1];
    }
  }
  p
}


fn cards_to_fixed_array(cards: &Vec<Card>) -> [u8; 7] {
  let mut numeric_vec: Vec<u8> = cards.iter().map(|c| u8::from(*c)).collect();
  while numeric_vec.len() < 7 {
    numeric_vec.push(u8::MAX);
  }
  numeric_vec[0..7].try_into().unwrap()
}


fn iterate_end_game(
  table_values: &Deck,
  player_values: &Deck,
  wins: &mut u32,
  games: &mut u32
) {
  let used_cards = *table_values + *player_values;
  let available_cards = used_cards.get_available_cards();

  let fixed_arr = cards_to_fixed_array(&table_values.get_cards());

  let player_hand = cards_to_fixed_array(&used_cards.get_cards());
  let player_score = evaluate_hand_raw(player_hand);

  for c1 in &available_cards {
    for c2 in &available_cards {
      if c1 == c2 {
        continue;
      }
      let mut opponent_hand = fixed_arr;
      opponent_hand[5] = u8::from(*c1);
      opponent_hand[6] = u8::from(*c2);
      let opponent_score = evaluate_hand_raw(opponent_hand);
      *games = *games + 1;
      if player_score >= opponent_score {
        *wins = *wins + 1;
      }
    }
  }
}


fn iterate_games(
  table_values: &Deck,
  player_values: &Deck,
  wins: &mut u32,
  games: &mut u32
) {
  let used_cards = *table_values + *player_values;
  let available_cards = used_cards.get_available_cards();
  let table_cards_played = 52 - 2 - available_cards.len();

  for c in &available_cards {
    let mut new_table_value = *table_values;
    new_table_value.add_card(*c);
    if table_cards_played + 1 < 5 {
      iterate_games(&new_table_value, player_values, wins, games);
    } else {
      iterate_end_game(&new_table_value, player_values, wins, games);
    }
  }
}


pub fn chance_to_win(table: &Deck, player: &Deck) -> f32 {
  let mut wins = 0;
  let mut games = 0;

  if table.get_cards().len() >= 5 {
    iterate_end_game(table, player, &mut wins, &mut games);
  } else {
    iterate_games(table, player, &mut wins, &mut games);
  }

  let percent = (wins as f32) / (games as f32);

  // println!("Player score {:.5}", player_score);
  println!("{}/{} {:.5}", wins, games, percent);

  percent

}



#[cfg(test)]
mod tests;

