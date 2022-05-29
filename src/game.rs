use crate::deck::Deck;
use crate::hand::*;



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

  for c in game.used_cards.get_available_cards(5) {
    game.used_cards.add_card(c);
    game.table.add_card(c);
  }



  for p in &mut game.players {
    for c in game.used_cards.get_available_cards(2) {
      game.used_cards.add_card(c);
      p.add_card(c);
    }
  }

  println!("{}", game.table);

  let player_scores = game.players.iter().map(|p| {
    println!("{:?}", p.get_cards());
  }).collect::<Vec<_>>();

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


