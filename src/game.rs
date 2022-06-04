use crate::deck::Deck;
use crate::card::*;
use rand::prelude::*;
use crate::evaluator::*;



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


  pub fn get_player_index(&self, position_from_dealer: u8) -> usize {
    usize::from(self.dealer_index + position_from_dealer) % self.players.len()
  }




  pub fn load_credit(&mut self, player_index: u8, credit: u32) {
    self.players[usize::from(player_index)].wallet += credit;
  }


  fn pick_available_card(&mut self) -> Card {
    let mut rng = thread_rng();
    let mut available_cards = self.available_cards.get_cards();
    let index = rng.gen_range(0..available_cards.len());
    let card = available_cards.remove(index);
    self.available_cards.remove_card(card);
    card
  }


  // TODO: Handle players being unable to call to max...
  fn have_all_players_posted(&self) -> bool {
    let max_bet = self.players.iter().map(|p| p.current_bet).max().unwrap();
    for p in &self.players {
      if p.current_bet != max_bet && !p.has_folded {
        return false;
      }
    }
    true
  }



  pub fn raise_for_player(&mut self, player_index: usize, bet: u32) -> Result<u32, &'static str> {
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

  pub fn fold_player(&mut self, player_index: usize) {
    self.players[player_index].current_bet = 0;
    self.players[player_index].has_folded = true;
  }



  pub fn deal_pre_flop(&mut self) -> Result<(), &'static str> {
    let num_players = u8::try_from(self.players.len()).unwrap();

    self.raise_for_player(self.get_player_index(1), self.blind / 2).expect("Failed to post small blind");
    self.raise_for_player(self.get_player_index(2), self.blind).expect("Failed to post big blind");

    for i in 0..(num_players * 2) {
      let card = self.pick_available_card();
      let player_index = self.get_player_index(i);
      self.players[player_index].hand.add_card(card);
    }
    Ok(())
  }


  pub fn deal_flop(&mut self) -> Result<(), &'static str> {
    if !self.have_all_players_posted() {
      return Err("Not all players have posted. Cannot deal flop.");
    }
    for _ in 0..3 {
      let card = self.pick_available_card();
      self.table.add_card(card);
    }
    for p in &mut self.players {
      p.current_bet = 0;
    }
    Ok(())
  }

  pub fn deal_turn(&mut self) -> Result<(), &'static str> {
    if !self.have_all_players_posted() {
      return Err("Not all players have posted. Cannot deal turn.");
    }
    let card = self.pick_available_card();
    self.table.add_card(card);
    Ok(())
  }

  pub fn deal_river(&mut self) -> Result<(), &'static str> {
    if !self.have_all_players_posted() {
      return Err("Not all players have posted. Cannot deal river.");
    }
    let card = self.pick_available_card();
    self.table.add_card(card);
    Ok(())
  }


}





pub fn test_game() {

  let mut game = Game::create(2);

  game.load_credit(0, 200);
  game.load_credit(1, 500);


  game.raise_for_player(1, 10).unwrap();

  // pre-flop
  game.deal_pre_flop().unwrap();
  for (i, p) in game.players.iter().enumerate() {
    println!("Player {} {}", i, p.hand);
  }


  // flop
  game.deal_flop().unwrap();
  println!("THE TABLE: {}", game.table);


  // turn
  game.deal_turn().unwrap();
  println!("THE TABLE: {}", game.table);

  // river
  game.deal_river().unwrap();
  println!("THE TABLE: {}", game.table);

  for (i, p) in game.players.iter().enumerate() {
    println!("Player {} {} {:?}", i, p.hand, get_hand(&game.table, &p.hand));
  }

}



#[cfg(test)]
mod tests;
