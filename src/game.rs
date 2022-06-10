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
  pub wallet: u32
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
  inactive_players: Vec<Player>,
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
      }).collect(),
      inactive_players: Vec::new(),
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
    let mut players = self.players.iter().map(|p| p).collect::<Vec<&Player>>();
    let inactive_players = self.inactive_players.iter().map(|p| p);
    players.extend(inactive_players);
    players
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



  fn init_round(&mut self) {
    self.pot = 0;
    self.available_cards = Deck::full_deck();
    self.table = Deck::new();
    for i in 0..self.players.len() {
      self.players[i].hand = Deck::new()
    }

    while let Some(idx) = self.players.iter().position(|p| p.wallet < self.blind) {
      let player = self.players.remove(idx);
      self.inactive_players.push(player);
    }

    let num_players = self.players.len() as u8;
    println!("we have {} active players this round", num_players);
    if num_players < 2 {
      panic!("We do not have enough players.");
    }

    // TODO: This is terrible... Find a better way to write this
    let total_players = (self.players.len() + self.inactive_players.len()) as u8;
    loop {
      self.dealer_id = (self.dealer_id + 1) % total_players;
      if let Some(_) = self.players.iter().find(|p| p.id == self.dealer_id) {
        break;
      }
    }
    println!("New dealer is {}", self.dealer_id);

    self.betting_round = BettingRound::create_for_players(num_players);

    let dealer_index = self.players.iter().position(|p| p.id == self.dealer_id).unwrap() as u8;
    self.betting_round.set_new_start_position(dealer_index + 1);

    self.betting_round.action_current_player(BettingAction::Raise(self.blind / 2)).unwrap();
    self.betting_round.action_current_player(BettingAction::Raise(self.blind)).unwrap();
    self.betting_round.set_new_start_position(dealer_index + 3);
  }


  // TODO: order is incorrect on deals
  fn deal_cards_to_players(&mut self) {
    for _ in 0..2 {
      for idx in 0..self.players.len() {
        let card = self.pick_available_card();
        self.players[idx].hand.add_card(card);
      }
    }
  }

  fn deal_cards_to_table(&mut self, num_cards: u8) {
    for _ in 0..num_cards {
      let card = self.pick_available_card();
      self.table.add_card(card);
    }
  }


  // TODO: Winners here may not all have an equal share of the pot
  fn finalize(&mut self) {
    let active_scores = self.players.iter().map(|p| {
      get_hand_score(&self.table, &p.hand)
    }).collect::<Vec<u32>>();


    let winning_score = active_scores.iter().max().unwrap();

    let winning_indexes = self.players.iter().enumerate()
      .filter(|(i, _)| active_scores[*i] == *winning_score)
      .map(|(i, _)| i).collect::<Vec<usize>>();
    let num_winners = winning_indexes.iter().count();

    println!("Pot of ${} will be split between {} winners: {:?}", self.pot, num_winners, winning_indexes);

    for idx in winning_indexes {
      self.players[idx].wallet += self.pot / num_winners as u32;
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
        self.init_round();
        println!("========= Dealing pre-flop ========");
        self.deal_cards_to_players();
        self.phase = Phase::PreFlop;
      },
      Phase::PreFlop => {
        self.perform_post_round();
        self.betting_round.restart();
        if self.betting_round.get_num_active_players() > 1 {
          println!("========= Dealing flop ========");
          self.deal_cards_to_table(3);

          let dealer_index = self.players.iter().position(|p| p.id == self.dealer_id).unwrap() as u8;
          self.betting_round.set_new_start_position(dealer_index + 1);
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



impl std::fmt::Display for Game {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {

    writeln!(f, "Pot: ${}   Table: {} ", self.pot, self.table)?;
    writeln!(f, "Plyr      Cards      Bid     Wallet")?;
    let player_bets = self.betting_round.get_player_bets();
    let curr_player = self.get_current_player();
    for (i, p) in self.players.iter().enumerate() {
      writeln!(f, "{} {}{}    {}    ${}     ${}   ",
        p.id,
        if p.id == self.dealer_id { 'D' } else { ' ' },
        if curr_player.is_some() && curr_player.unwrap().id == p.id { 'P' } else { ' ' },
        p.hand,
        player_bets[i],
        p.wallet
      )?;
    }
    write!(f, "")
  }
}







#[cfg(test)]
mod tests;
