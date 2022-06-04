use crate::deck::Deck;
use crate::card::*;
use rand::prelude::*;
use crate::evaluator::*;



struct Player {
  hand: Deck,
  wallet: u32,
}


struct Game {
  available_cards: Deck,
  table: Deck,
  players: Vec<Player>,
}


fn print_game(game: &Game) {
  println!("THE TABLE: {}", game.table);
  for p in &game.players {
    println!("{}", p.hand);
  }
}


fn get_random_available_card(game: &Game) -> Card {
  let mut rng = thread_rng();
  loop {
    let possible_card = Card::try_from(rng.gen_range(0..52)).unwrap();
    if game.available_cards.has_card(possible_card) {
      break possible_card;
    }
  }

}


pub fn test_game() {

  let mut game = Game {
    available_cards: Deck::full_deck(),
    table: Deck::new(),
    players: vec!(
      Player { hand: Deck::new(), wallet: 0 },
      Player { hand: Deck::new(), wallet: 0 },
    )
  };

  let get_random_available_card = || {
    let mut rng = thread_rng();
    loop {
      let possible_card = Card::try_from(rng.gen_range(0..52)).unwrap();
      if game.available_cards.has_card(possible_card) {
        break possible_card;
      }
    }

  };


  // pre-flop
  for p in game.players.iter_mut() {
    for _ in 0..2 {
      let card = get_random_available_card();
      p.hand.add_card(card);
    }
  }

  print_game(&game);


  // flop
  for _ in 0..3 {
    game.table.add_card(get_random_available_card());
  }
  print_game(&game);

  // turn
  game.table.add_card(get_random_available_card());
  print_game(&game);

  // river

  game.table.add_card(get_random_available_card());
  print_game(&game);

  for p in &game.players {
    println!("{:?}", get_hand(&game.table, &p.hand));
  }

}



#[cfg(test)]
mod tests;
