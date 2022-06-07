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
  hand: Deck,
  wallet: u32,
}


pub struct Game {
  phase: Phase,
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
      phase: Phase::Init,
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
    self.players[player_index as usize].wallet += credit;
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



  // Returns true if only 1 player is left
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


  pub fn get_current_player(&self) -> Option<&Player> {
    if self.phase == Phase::Showdown || self.phase == Phase::Init || self.betting_round.is_complete() {
      None
    } else {
      Some(&self.players[self.betting_round.get_current_player_index() as usize])
    }
  }


  pub fn action_current_player(&mut self, action: BettingAction) -> Result<(), &'static str> {
    let player = self.get_current_player();
    if player.is_none() {
      return Err("This is not the right time to bet.")
    }

    let player = player.unwrap();
    println!("Actioning {:?} for player {}", action, self.betting_round.get_current_player_index());


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
    let num_players = u8::try_from(self.players.len()).unwrap();

    self.pot = 0;
    self.dealer_index = self.increment_player_index(self.dealer_index, 1);

    // TODO: Loop until we find someone with enough money to post blinds...
    self.betting_round.initialize(self.increment_player_index(self.dealer_index, 1));
    self.betting_round.action_current_player(BettingAction::Raise(self.blind / 2)).unwrap();
    self.betting_round.action_current_player(BettingAction::Raise(self.blind)).unwrap();

    self.available_cards = Deck::full_deck();
    self.table = Deck::new();
    for i in 0..num_players {
      self.players[i as usize].hand = Deck::new()
    }

    for i in 0..(num_players * 2) {
      let card = self.pick_available_card();
      let player_index = self.increment_player_index(self.dealer_index, i);
      self.players[player_index as usize].hand.add_card(card);
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

    let winning_indexes = active_indexes.iter().filter(|i| active_scores[**i as usize] == *winning_score).map(|i| *i).collect::<Vec<u8>>();
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
    println!("Next starts with {:?}", self.phase);


    if !self.betting_round.is_complete() && self.phase != Phase::Showdown && self.phase != Phase::Init {
      return Some(self.phase);
    }


    match self.phase {
      Phase::Init => {
        if self.deal_pre_flop().is_ok() {
          self.phase = Phase::PreFlop;
        }
      },
      Phase::PreFlop => {
        if self.perform_post_round() {
          self.phase = Phase::Showdown;
        } else {
          println!("========= Dealing flop ========");
          self.deal_cards_to_table(3);
          self.phase = Phase::Flop;
        }
      }
      Phase::Flop => {
        if self.perform_post_round() {
          self.phase = Phase::Showdown;
        } else {
          println!("========= Dealing turn ========");
          self.deal_cards_to_table(1);
          self.phase = Phase::Turn;
        }
      }
      Phase::Turn => {
        if self.perform_post_round() {
          self.phase = Phase::Showdown;
        } else {
          println!("========= Dealing river ========");
          self.deal_cards_to_table(1);
          self.phase = Phase::River;
        }
      },
      Phase::River => {
        self.perform_post_round();
        self.phase = Phase::Showdown;
      },
      Phase::Showdown => {
        self.finalize();
        self.phase = Phase::Init;
      }
    };

    println!("Next is going to {:?}", self.phase);
    Some(self.phase)

  }

}






#[cfg(test)]
mod tests;
