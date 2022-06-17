use crate::card::*;
use crate::deck::*;

mod types;
mod two_plus_two;
mod ck_perfect_hash;

use types::*;


fn score_to_hand(score: u16) -> Hand {
  if cfg!(feature = "eval_two_plus_two") {
    two_plus_two::score_to_hand(score)
  } else {
    ck_perfect_hash::score_to_hand(score)
  }
}


fn evaluate_score(cards: [u8; 7]) -> u16 {
  if cfg!(feature = "eval_two_plus_two") {
    two_plus_two::evaluate_score(cards)
  } else {
    ck_perfect_hash::evaluate_score(cards)
  }
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
  let player_score = evaluate_score(player_hand);

  for c1 in &available_cards {
    for c2 in &available_cards {
      if c1 == c2 {
        continue;
      }
      let mut opponent_hand = fixed_arr;
      opponent_hand[5] = u8::from(*c1);
      opponent_hand[6] = u8::from(*c2);
      let opponent_score = evaluate_score(opponent_hand);
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


pub fn get_hand_score(table: &Deck, player: &Deck) -> u16 {
  let combined = *table + *player;
  let fixedarr = cards_to_fixed_array(&combined.get_cards());
  evaluate_score(fixedarr)
}

pub fn get_hand_for_score(score: u16) -> Hand {
  score_to_hand(score)
}



#[cfg(test)]
mod test_helpers;
mod tests;
