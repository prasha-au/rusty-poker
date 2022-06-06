mod betting_round;

use crate::deck::Deck;
use crate::card::*;
use rand::prelude::*;
use crate::evaluator::*;
use betting_round::*;




enum Phase {
  PreFlop,
  Flop,
  Turn,
  River,
  Showdown
}


struct Player {
  hand: Deck,
  wallet: u32,
}





struct GameRound {
  pot: u32,
  phase: Phase,
  available_cards: Deck,
  table: Deck,
  player_cards: Vec<Deck>,
  players_wallets: Vec<u32>,
}












struct Game {
  pot: u32,
  available_cards: Deck,
  table: Deck,
  betting_round: BettingRound,
  dealer_index: u8,
  blind: u32,
  players: Vec<Player>,
}



impl Game {

  pub fn create(player_count: u8) -> Game {
    Game {
      pot: 0,
      available_cards: Deck::full_deck(),
      table: Deck::new(),
      dealer_index: 0,
      blind: 20,
      players: (0..player_count).map(|_| Player {
        hand: Deck::new(),
        wallet: 0,
      }).collect(),
      betting_round: BettingRound::create_for_players(player_count)
    }
  }


  pub fn load_credit(&mut self, player_index: u8, credit: u32) {
    self.players[usize::from(player_index)].wallet += credit;
  }


  fn increment_player_index(&self, from_index: u8, value: u8) -> u8 {
    (from_index + value) % self.players.len() as u8
  }


  fn pick_available_card(&mut self) -> Card {
    let mut rng = thread_rng();
    let mut available_cards = self.available_cards.get_cards();
    let index = rng.gen_range(0..available_cards.len());
    let card = available_cards.remove(index);
    self.available_cards.remove_card(card);
    card
  }



  // Returns true if we are done...
  fn perform_post_round(&mut self) -> bool {
    let player_bets = self.betting_round.get_player_bets();
    let mut add_to_pot = 0;
    for (i, bet_amount) in player_bets.iter().enumerate() {
      add_to_pot += bet_amount;
      self.players[i].wallet -= bet_amount;
      // println!("New wallet value for {} is ${}", i, self.players[i].wallet);
    }
    self.pot += add_to_pot;

    self.betting_round.initialize(self.increment_player_index(self.dealer_index, 1));

    self.betting_round.get_active_player_indexes().iter().count() < 2
  }





  pub fn action_current_player(&mut self, action: CurrentPlayerAction) -> Result<(), &'static str> {

    // TODO: check if the player has enough funds...
    if self.betting_round.action_current_player(action).is_err() {
      panic!("Failed when trying to action player");
    }

    Ok(())
  }



  pub fn deal_pre_flop(&mut self) -> Result<(), &'static str> {
    let num_players = u8::try_from(self.players.len()).unwrap();

    self.pot = 0;

    self.available_cards = Deck::full_deck();
    for i in 0..num_players {
      self.players[i as usize].hand = Deck::new()
    }

    for i in 0..(num_players * 2) {
      let card = self.pick_available_card();
      let player_index = self.increment_player_index(self.dealer_index, i);
      self.players[player_index as usize].hand.add_card(card);
    }


    // TODO: Loop until we find someone with enough money to post blinds...
    self.betting_round.initialize(self.increment_player_index(self.dealer_index, 1));
    self.betting_round.action_current_player(CurrentPlayerAction::Raise(self.blind / 2)).unwrap();
    self.betting_round.action_current_player(CurrentPlayerAction::Raise(self.blind)).unwrap();

    Ok(())
  }


  pub fn deal_flop(&mut self) -> Result<(), &'static str> {
    if !self.betting_round.is_complete() {
      return Err("Betting is not complete. Cannot deal flop.");
    }

    if self.perform_post_round() {
      return Ok(());
    }

    for _ in 0..3 {
      let card = self.pick_available_card();
      self.table.add_card(card);
    }
    Ok(())
  }

  pub fn deal_turn(&mut self) -> Result<(), &'static str> {
    if !self.betting_round.is_complete() {
      return Err("Betting is not complete. Cannot deal flop.");
    }

    if self.perform_post_round() {
      return Ok(());
    }

    let card = self.pick_available_card();
    self.table.add_card(card);
    Ok(())
  }

  pub fn deal_river(&mut self) -> Result<(), &'static str> {
    if !self.betting_round.is_complete() {
      return Err("Betting is not complete. Cannot deal river.");
    }

    if self.perform_post_round() {
      return Ok(());
    }

    let card = self.pick_available_card();
    self.table.add_card(card);
    Ok(())
  }

  // TODO: Winners here may not all have an equal share of the pot
  pub fn finalize(&mut self) -> Result<(), &'static str> {
    let active_indexes = self.betting_round.get_active_player_indexes();
    let active_scores = active_indexes.iter().map(|i| {
      let player = &self.players[*i as usize];
      get_hand_score(&self.table, &player.hand)
    }).collect::<Vec<u32>>();
    let winning_score = active_scores.iter().max().unwrap();

    let winning_indexes = active_indexes.iter().filter(|i| active_scores[**i as usize] == *winning_score).map(|i| *i).collect::<Vec<u8>>();
    let num_winners = winning_indexes.iter().count();


    println!("Pot of ${} will be split between {} winners: {:?}", self.pot, num_winners, winning_indexes);

    for idx in winning_indexes {
      self.players[idx as usize].wallet += self.pot / num_winners as u32;
    }


    Ok(())
  }


}





pub fn test_game() {

  let mut game = Game::create(2);

  game.load_credit(0, 500);
  game.load_credit(1, 500);




  // pre-flop
  println!("========= Dealing pre-flop ========");
  game.deal_pre_flop().unwrap();
  for (i, p) in game.players.iter().enumerate() {
    println!("Player {} {}", i, p.hand);
  }


  println!("Game has a pot of {}", game.pot);
  game.action_current_player(CurrentPlayerAction::Raise(50)).unwrap();
  game.action_current_player(CurrentPlayerAction::Call).unwrap();

  println!("Game has a pot of {}", game.pot);

  // flop
  println!("========= Dealing flop ========");
  game.deal_flop().unwrap();
  println!("THE TABLE: {}", game.table);
  game.action_current_player(CurrentPlayerAction::Call).unwrap();
  game.action_current_player(CurrentPlayerAction::Raise(50)).unwrap();
  game.action_current_player(CurrentPlayerAction::Call).unwrap();


  // turn
  println!("========= Dealing turn ========");
  game.deal_turn().unwrap();
  println!("THE TABLE: {}", game.table);
  game.action_current_player(CurrentPlayerAction::Call).unwrap();
  game.action_current_player(CurrentPlayerAction::Call).unwrap();

  // river
  println!("========= Dealing river ========");
  game.deal_river().unwrap();
  println!("THE TABLE: {}", game.table);

  for (i, p) in game.players.iter().enumerate() {
    let score = get_hand_score(&game.table, &p.hand);
    println!("Player {} {} {:?} ({:?})", i, p.hand, score, get_hand_for_score(score));
  }

  game.finalize().unwrap();



  for (i, p) in game.players.iter().enumerate() {
    println!("Player {} ${}", i, p.wallet);
  }




}







#[cfg(test)]
mod tests;
