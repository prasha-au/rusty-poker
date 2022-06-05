use crate::deck::Deck;
use crate::card::*;
use rand::prelude::*;
use crate::evaluator::*;



enum CurrentPlayerAction {
  Fold,
  Call,
  Raise(u32),
}


struct Player {
  hand: Deck,
  current_bet: u32,
  wallet: u32,
  has_folded: bool,
}


struct Game {
  available_cards: Deck,
  table: Deck,
  pot: u32,
  dealer_index: u8,
  current_player_index: u8,
  final_player_index: u8,
  blind: u32,
  // players clockwise around the table
  players: Vec<Player>,
}



impl Game {

  pub fn create(player_count: u8) -> Game {
    let mut game = Game {
      available_cards: Deck::full_deck(),
      table: Deck::new(),
      pot: 0,
      dealer_index: 0,
      current_player_index: 0,
      final_player_index: 0,
      blind: 20,
      players: Vec::new(),
    };

    for _ in 0..player_count {
      game.players.push(Player {
        hand: Deck::new(),
        current_bet: 0,
        wallet: 0,
        has_folded: false,
      });
    }

    game
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


  fn move_player_funds(&mut self, player_index: u8, bet: u32) -> Result<u32, &'static str> {
    let player = &mut self.players[usize::from(player_index)];
    if player.wallet < bet {
      return Err("Player does not have enough credit.");
    }
    if player.has_folded {
      return Err("Player has already folded.");
    }
    player.current_bet += bet;
    self.pot += bet;
    player.wallet -= bet;
    Ok(player.wallet)
  }



  // Returns true if more players need to have a go
  pub fn action_current_player(&mut self, action: CurrentPlayerAction) -> bool {

    println!("Actioning for player {}", self.current_player_index);

    let max_bet = self.players.iter().map(|p| p.current_bet).max().unwrap();
    match action {
      CurrentPlayerAction::Fold => {
        self.players[self.current_player_index as usize].has_folded = true;
        self.players[self.current_player_index as usize].current_bet = 0;
      }
      CurrentPlayerAction::Call => {
        let current_bet = self.players[self.current_player_index as usize].current_bet;
        let increase = max_bet - current_bet;
        self.move_player_funds(self.current_player_index, increase).unwrap();
      }
      CurrentPlayerAction::Raise(bet) => {
        let increase = bet - max_bet;
        self.move_player_funds(self.current_player_index, increase).unwrap();
        self.final_player_index = self.current_player_index;
      }
    };

    loop {
      self.current_player_index = self.increment_player_index(self.current_player_index, 1);
      if self.players[self.current_player_index as usize].has_folded {
        continue;
      }
      return self.current_player_index != self.final_player_index;
    }
  }


  pub fn get_players_left(&mut self) -> u8 {
    self.players.iter().filter(|p| !p.has_folded).count() as u8
  }


  pub fn deal_pre_flop(&mut self) -> Result<(), &'static str> {
    let num_players = u8::try_from(self.players.len()).unwrap();

    self.move_player_funds(self.increment_player_index(self.dealer_index, 1), self.blind / 2).expect("Failed to post small blind");
    self.move_player_funds(self.increment_player_index(self.dealer_index, 2), self.blind).expect("Failed to post big blind");

    for i in 0..(num_players * 2) {
      let card = self.pick_available_card();
      let player_index = self.increment_player_index(self.dealer_index, i);
      self.players[player_index as usize].hand.add_card(card);
    }
    self.final_player_index = self.increment_player_index(self.dealer_index, 2);
    self.current_player_index = self.increment_player_index(self.dealer_index, 2);
    Ok(())
  }


  pub fn deal_flop(&mut self) -> Result<(), &'static str> {
    if self.current_player_index != self.final_player_index {
      return Err("Not all players have posted. Cannot deal flop.");
    }
    if self.get_players_left() < 2 {
      return Err("Not enough players to continue.");
    }

    for _ in 0..3 {
      let card = self.pick_available_card();
      self.table.add_card(card);
    }
    for p in &mut self.players {
      p.current_bet = 0;
    }
    self.final_player_index = self.dealer_index;
    self.current_player_index = self.dealer_index;
    Ok(())
  }

  pub fn deal_turn(&mut self) -> Result<(), &'static str> {
    if self.current_player_index != self.final_player_index {
      return Err("Not all players have posted. Cannot deal turn.");
    }
    if self.get_players_left() < 2 {
      return Err("Not enough players to continue.");
    }
    let card = self.pick_available_card();
    self.table.add_card(card);
    for p in &mut self.players {
      p.current_bet = 0;
    }
    self.final_player_index = self.dealer_index;
    self.current_player_index = self.dealer_index;
    Ok(())
  }

  pub fn deal_river(&mut self) -> Result<(), &'static str> {
    if self.current_player_index != self.final_player_index {
      return Err("Not all players have posted. Cannot deal river.");
    }
    if self.get_players_left() < 2 {
      return Err("Not enough players to continue.");
    }
    let card = self.pick_available_card();
    self.table.add_card(card);
    for p in &mut self.players {
      p.current_bet = 0;
    }
    self.final_player_index = self.dealer_index;
    self.current_player_index = self.dealer_index;
    Ok(())
  }



  pub fn finalize_game(&mut self) -> Result<(), &'static str> {
    if self.current_player_index != self.final_player_index {
      return Err("Not all players have posted. Cannot finalize game.");
    }
    let players_left = self.get_players_left();
    if players_left > 1 && self.table.get_cards().len() < 5 {
      return Err("Game is not finished yet.");
    }

    if players_left == 1 {
      let winner_idx = self.players.iter().position(|p| !p.has_folded).unwrap();
      self.players[winner_idx].wallet += self.pot;
    } else {
      let player_hand_scores = self.players.iter().map(|p| get_hand_score(&self.table, &p.hand)).collect::<Vec<u32>>();
      let highest_score = player_hand_scores.iter().max().unwrap();
      let num_player_with_highest_score = player_hand_scores.iter().filter(|score| *score == highest_score).count();
      // TODO: Distribute funds evenly
    }


    self.pot = 0;
    for p in &mut self.players {
      p.current_bet = 0;
      p.has_folded = false;
    }

    Ok(())
  }


}





pub fn test_game() {

  let mut game = Game::create(2);

  game.load_credit(0, 200);
  game.load_credit(1, 500);




  // pre-flop
  println!("========= Dealing pre-flop ========");
  game.deal_pre_flop().unwrap();
  for (i, p) in game.players.iter().enumerate() {
    println!("Player {} {}", i, p.hand);
  }


  println!("Game has a pot of {}", game.pot);
  game.action_current_player(CurrentPlayerAction::Raise(50));
  game.action_current_player(CurrentPlayerAction::Call);

  println!("Game has a pot of {}", game.pot);

  // flop
  println!("========= Dealing flop ========");
  game.deal_flop().unwrap();
  println!("THE TABLE: {}", game.table);
  game.action_current_player(CurrentPlayerAction::Call);
  game.action_current_player(CurrentPlayerAction::Raise(50));
  game.action_current_player(CurrentPlayerAction::Call);


  // turn
  println!("========= Dealing turn ========");
  game.deal_turn().unwrap();
  println!("THE TABLE: {}", game.table);


  // river
  println!("========= Dealing river ========");
  game.deal_river().unwrap();
  println!("THE TABLE: {}", game.table);

  for (i, p) in game.players.iter().enumerate() {
    println!("Player {} {} {:?}", i, p.hand, get_hand(&game.table, &p.hand));
  }

}



#[cfg(test)]
mod tests;
