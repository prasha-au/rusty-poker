mod betting_round;

use crate::deck::Deck;
use crate::card::*;
use rand::prelude::*;
use crate::evaluator::*;
use betting_round::*;


pub use betting_round::BettingAction;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Phase {
  Init,
  PreFlop,
  Flop,
  Turn,
  River,
  Showdown
}


pub struct Player {
  pub id: u8,
  hand: Deck,
  pub wallet: u32,
  is_active: bool,
}


pub struct Game {
  phase: Phase,
  pot: u32,
  available_cards: Deck,
  table: Deck,
  betting_round: BettingRound,
  dealer_id: u8,
  blind: u32,
  players: Vec<Player>,
  // inactive_players: Vec<Player>,
}



impl Game {

  pub fn create(player_count: u8, initial_credit: u32) -> Game {
    Game {
      phase: Phase::Init,
      pot: 0,
      available_cards: Deck::full_deck(),
      table: Deck::new(),
      dealer_id: 0,
      blind: 20,
      players: (0..player_count).map(|id| Player {
        id,
        hand: Deck::new(),
        wallet: initial_credit,
        is_active: true,
      }).collect(),
      // inactive_players: Vec::new(),
      betting_round: BettingRound::create_for_players(player_count)
    }
  }


  fn pick_available_card(&mut self) -> Card {
    let mut rng = thread_rng();
    let mut available_cards = self.available_cards.get_cards();
    let index = rng.gen_range(0..available_cards.len());
    let card = available_cards.remove(index);
    self.available_cards.remove_card(card);
    card
  }


  fn perform_post_round(&mut self) {
    let player_bets = self.betting_round.get_player_bets();
    let mut add_to_pot = 0;
    for (i, bet_amount) in player_bets.iter().enumerate() {
      add_to_pot += bet_amount;
      self.players[i].wallet -= bet_amount;
    }
    self.pot += add_to_pot;
  }



  pub fn get_all_players(&self) -> Vec<&Player> {
    self.players.iter().map(|p| p).collect::<Vec<&Player>>()
  }


  pub fn get_current_player(&self) -> Option<&Player> {
    if self.phase == Phase::Showdown || self.phase == Phase::Init || self.betting_round.is_complete() {
      None
    } else {
      Some(&self.players[self.betting_round.get_current_player_index() as usize])
    }
  }

  pub fn get_current_bet(&self) -> u32 {
    self.betting_round.get_current_bet()
  }

  pub fn action_current_player(&mut self, action: BettingAction) -> Result<(), &'static str> {
    let player = self.get_current_player();
    if player.is_none() {
      return Err("This is not the right time to bet.")
    }

    let player = player.unwrap();
    println!("Actioning {:?} for player {}", action, player.id);


    if let BettingAction::Raise(amount) = action {
      if amount > player.wallet {
        return Err("Cannot raise more than you have!");
      } else if amount == player.wallet {
        return Err("You must go all in.");
      }
    } else if let BettingAction::Call = action {
      if self.betting_round.get_current_bet() > player.wallet {
        return Err("Cannot call more than you have!");
      } else if self.betting_round.get_current_bet() == player.wallet {
        return Err("You must go all in.");
      }
    } else if let BettingAction::AllIn(amount) = action {
      if amount != player.wallet {
        return Err("Must go all in with your entire wallet.");
      }
    }

    self.betting_round.action_current_player(action)
  }


  fn deal_pre_flop(&mut self) -> Result<(), &'static str> {
    self.pot = 0;

    for p in &mut self.players {
      if p.wallet < self.blind {
        p.is_active = false;
      }
    }

    let active_players = self.players.iter().filter(|p| p.is_active).collect::<Vec<&Player>>();

    let total_players = self.players.len() as u8;
    let num_players = active_players.len() as u8;
    println!("we have {} active players this round", num_players);

    if num_players < 2 {
      panic!("We do not have enough players.");
    }

    // TODO: This is terrible... Find a better way to write this
    let mut dealer_index = self.players.iter().position(|p| p.id == self.dealer_id).unwrap() as u8;
    loop {
      dealer_index = (dealer_index + 1) % total_players;
      if active_players[dealer_index as usize].is_active {
        self.dealer_id = active_players[dealer_index as usize].id;
        break;
      }
    }
    println!("New dealer is {}", self.dealer_id);

    self.betting_round = BettingRound::create_for_players(num_players);

    // TODO: Refactor this copy paste...
    let new_index = (dealer_index + 1) % self.players.len() as u8;
    self.betting_round.set_new_start_position(new_index as u8);

    self.betting_round.action_current_player(BettingAction::Raise(self.blind / 2)).unwrap();
    self.betting_round.action_current_player(BettingAction::Raise(self.blind)).unwrap();
    self.betting_round.set_new_start_position(new_index + 2);


    self.available_cards = Deck::full_deck();
    self.table = Deck::new();
    for i in 0..self.players.len() {
      self.players[i as usize].hand = Deck::new()
    }


    let active_players = self.betting_round.get_active_player_indexes();
    for _ in 0..2 {
      for idx in active_players.iter() {
        let card = self.pick_available_card();
        self.players[*idx as usize].hand.add_card(card);
      }
    }

    Ok(())
  }


  fn deal_cards_to_table(&mut self, num_cards: u8) {
    for _ in 0..num_cards {
      let card = self.pick_available_card();
      self.table.add_card(card);
    }
  }


  // TODO: Winners here may not all have an equal share of the pot
  fn finalize(&mut self) {
    let active_indexes = self.betting_round.get_active_player_indexes();
    let active_scores = active_indexes.iter().map(|i| {
      let player = &self.players[*i as usize];
      get_hand_score(&self.table, &player.hand)
    }).collect::<Vec<u32>>();
    let winning_score = active_scores.iter().max().unwrap();


    let winning_indexes = active_indexes.iter().enumerate()
      .filter(|(i, _)| active_scores[*i] == *winning_score)
      .map(|(_, v)| *v).collect::<Vec<u8>>();
    let num_winners = winning_indexes.iter().count();

    println!("Table: {}", self.table);
    for (i, p) in self.players.iter().enumerate() {
      println!("Player {} {}", i, p.hand);
    }
    println!("Pot of ${} will be split between {} winners: {:?}", self.pot, num_winners, winning_indexes);

    for idx in winning_indexes {
      self.players[idx as usize].wallet += self.pot / num_winners as u32;
    }
  }
}


impl Iterator for Game {
  type Item = Phase;


  fn next(&mut self) -> Option<Self::Item> {
    if !self.betting_round.is_complete() && self.phase != Phase::Showdown && self.phase != Phase::Init {
      return Some(self.phase);
    }

    match self.phase {
      Phase::Init => {
        println!("========= Dealing pre-flop ========");
        if self.deal_pre_flop().is_ok() {
          self.phase = Phase::PreFlop;
        }
      },
      Phase::PreFlop => {
        self.perform_post_round();
        self.betting_round.restart();
        if self.betting_round.get_num_active_players() > 1 {
          println!("========= Dealing flop ========");
          self.deal_cards_to_table(3);

          let dealer_index = self.players.iter().position(|p| p.id == self.dealer_id).unwrap();
          let new_index = (dealer_index + 1) % self.players.len();
          self.betting_round.set_new_start_position(new_index as u8);
          self.phase = Phase::Flop;
        } else {
          println!("========= Dealing flop and going to showdown ========");
          self.deal_cards_to_table(3);
          self.phase = Phase::Showdown;
        }
      }
      Phase::Flop => {
        self.perform_post_round();
        self.betting_round.restart();
        if self.betting_round.get_num_active_players() > 1 {
          println!("========= Dealing turn ========");
          self.deal_cards_to_table(1);
          self.phase = Phase::Turn;
        } else {
          self.phase = Phase::Showdown;
        }
      }
      Phase::Turn => {
        self.perform_post_round();
        self.betting_round.restart();
        if self.betting_round.get_num_active_players() > 1 {
          println!("========= Dealing river ========");
          self.deal_cards_to_table(1);
          self.phase = Phase::River;
        } else {
          self.phase = Phase::Showdown;
        }
      },
      Phase::River => {
        println!("========= Showdown ========");
        self.perform_post_round();
        self.phase = Phase::Showdown;
      },
      Phase::Showdown => {
        self.finalize();
        let valid_players = self.players.iter().filter(|p| p.wallet >= self.blind).count();
        if valid_players < 2 {
          return None;
        }
        self.phase = Phase::Init;
      }
    };
    Some(self.phase)
  }
}



#[cfg(test)]
mod tests;
