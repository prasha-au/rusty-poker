use crate::card::*;
use crate::deck::*;

mod types;
mod two_plus_two;

use types::*;
use two_plus_two::{evaluate_two_plus_two};

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
  let player_score = evaluate_two_plus_two(player_hand);

  for c1 in &available_cards {
    for c2 in &available_cards {
      if c1 == c2 {
        continue;
      }
      let mut opponent_hand = fixed_arr;
      opponent_hand[5] = u8::from(*c1);
      opponent_hand[6] = u8::from(*c2);
      let opponent_score = evaluate_two_plus_two(opponent_hand);
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

  println!("{}/{} {:.5}", wins, games, percent);

  percent
}

pub fn get_hand(table: &Deck, player: &Deck) -> Hand {
  let combined = *table + *player;
  let fixedarr = cards_to_fixed_array(&combined.get_cards());
  let value = evaluate_two_plus_two(fixedarr);
  let hand_value = (value >> 12 & 0xF) as u8;
  Hand::from(hand_value)
}



#[cfg(test)]
mod tests;

