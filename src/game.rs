use crate::deck::Deck;
use crate::hand::*;
use crate::card::*;
use strum::IntoEnumIterator;

// enum Phase {
//   PreFlop,
//   Flop,
//   Turn,
//   River,
//   Showdown
// }




struct Game {
  // phase: Phase,
  used_cards: Deck,
  table: Deck,
  players: Vec<Deck>
}





fn simulate_game(game: &mut Game) {


  let table_cards_remaining = 5 - game.table.get_cards().len();

  let player_cards_remaining = (game.players.len() * 2) - game.players.iter().fold(0, |acc, p| acc + p.get_cards().len());









  for _ in 0..table_cards_remaining {
    let c = game.used_cards.pick_available_card();
    game.table.add_card(c);
  }



  for p in &mut game.players {
    for _ in 0..2 {
      let c = game.used_cards.pick_available_card();
      p.add_card(c);
    }
  }

  println!("{}", game.table);
}




fn evaluate_winner(game: &Game) -> u8 {

  let player_scores = game.players.iter().map(|p| {
    u16::from(evaluate_deck(&(*p + game.table)))
  }).collect::<Vec<_>>();

  let mut highest_value = 0;
  let mut highest_value_index = 0;


  for (i, rank_value) in player_scores.iter().enumerate() {
    println!("Player at {} had score of {}", i, rank_value);
    if *rank_value >= highest_value {
      highest_value_index = i;
      highest_value = *rank_value;
    }
  }

  highest_value_index as u8
}




#[cfg(test)]
mod tests;


