mod betting_round;

use crate::deck::Deck;
use crate::card::*;
use rand::prelude::*;
use crate::evaluator::*;
use betting_round::*;


pub use betting_round::BettingAction;


#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Phase {
  Init = 0,
  PreFlop = 1,
  Flop = 2,
  Turn = 3,
  River = 4,
  Showdown = 5
}


struct Seat {
  player_index: u8,
  hand: Deck,
  wallet: u32,
}



pub struct Game {
  phase: Phase,
  available_cards: Deck,
  table: Deck,
  betting_round: BettingRound,
  dealer_index: u8,
  blind: u32,
  active_seats: Vec<Seat>,
}


#[derive(Copy, Clone, Debug)]
pub struct PlayerState {
  pub is_folded: bool,
  pub wallet: u32,
  pub money_on_table: u32,
}


#[derive(Copy, Clone, Debug)]
pub struct GameState {
  pub total_pot: u32,
  pub hand: Deck,
  pub table: Deck,
  pub wallet: u32,
  pub phase: Phase,
  pub players: [Option<PlayerState>; 8],
  pub current_player_index: Option<u8>,
  pub dealer_index: u8,
  pub value_to_call: u32,
}


impl Game {

  pub fn create(num_players: u8, initial_wallet: u32) -> Game {
    Game {
      phase: Phase::Init,
      available_cards: Deck::full_deck(),
      table: Deck::new(),
      dealer_index: num_players - 1,
      blind: 20,
      active_seats: (0..num_players).map(|player_index| Seat {
        player_index,
        hand: Deck::new(),
        wallet: initial_wallet,
      }).collect(),
      betting_round: BettingRound::create_for_players(num_players)
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


  fn get_current_seat(&self) -> Option<&Seat> {
    if self.phase == Phase::Init || self.phase == Phase::Showdown || self.betting_round.is_complete() {
      None
    } else {
      Some(&self.active_seats[self.betting_round.get_current_player_index() as usize])
    }
  }

  pub fn get_current_player_index(&self) -> Option<u8> {
    if let Some(curr_seat) = self.get_current_seat() {
      Some(curr_seat.player_index)
    } else {
      None
    }
  }


  pub fn action_current_player(&mut self, action: BettingAction) -> Result<(), &'static str> {
    let current_seat_index = self.betting_round.get_current_player_index();

    let seat = self.get_current_seat();
    if seat.is_none() {
      return Err("This is not the right time to bet.")
    }
    let seat = seat.unwrap();

    if let BettingAction::Raise(amount) = action {
      if amount > seat.wallet {
        return Err("Cannot raise more than you have.");
      } else if amount == seat.wallet {
        return Err("You must go all in.");
      }
    } else if let BettingAction::Call = action {
      let deficit = self.betting_round.get_player_money_to_call(current_seat_index);
      if deficit > seat.wallet {
        return Err("Cannot call more than you have!");
      } else if deficit == seat.wallet {
        return Err("You must go all in.");
      }
    } else if let BettingAction::AllIn(amount) = action {
      if amount != seat.wallet {
        return Err("Must go all in with your entire wallet.");
      }
    }

    let new_money = self.betting_round.action_current_player(action).unwrap();
    self.active_seats[current_seat_index as usize].wallet -= new_money;
    Ok(())
  }


  fn post_blind(&mut self, amount: u32) {
    let mut seat = &mut self.active_seats[self.betting_round.get_current_player_index() as usize];
    let betting_action = if seat.wallet == amount { BettingAction::AllIn(amount) } else { BettingAction::Raise(amount) };
    let new_money = self.betting_round.action_current_player(betting_action).unwrap();
    seat.wallet -= new_money;
  }


  fn init_round(&mut self) {
    self.available_cards = Deck::full_deck();
    self.table = Deck::new();
    for i in 0..self.active_seats.len() {
      self.active_seats[i].hand = Deck::new()
    }

    let mut new_dealer_player_index = None;
    let mut invalid_player_indexes: Vec<u8> = vec![];
    for i in 1..self.active_seats.len() {
      let seat = &self.active_seats[(self.dealer_index as usize + i) % self.active_seats.len()];
      if seat.wallet < self.blind {
        invalid_player_indexes.push(seat.player_index);
      } else if new_dealer_player_index.is_none() {
        new_dealer_player_index = Some(seat.player_index);
      }
    }

    self.active_seats.retain(|s| !invalid_player_indexes.contains(&s.player_index));

    let num_active_players = self.active_seats.len() as u8;
    if num_active_players < 2 {
      panic!("We do not have enough players.");
    }

    self.dealer_index = self.active_seats.iter().position(|s| s.player_index == new_dealer_player_index.unwrap()).unwrap() as u8;

    self.betting_round = BettingRound::create_for_players(num_active_players);
    self.betting_round.set_new_start_position(self.dealer_index + 1);

    self.post_blind(self.blind / 2);
    self.post_blind(self.blind);
    self.betting_round.set_new_start_position(self.dealer_index + 3);
  }


  fn deal_cards_to_players(&mut self) {
    let num_active_seats = self.active_seats.len();
    for _ in 0..2 {
      for pnum in 0..num_active_seats {
        let idx = (self.dealer_index as usize + pnum) % num_active_seats;
        let card = self.pick_available_card();
        self.active_seats[idx].hand.add_card(card);
      }
    }
  }

  fn deal_cards_to_table(&mut self, num_cards: u8) {
    for _ in 0..num_cards {
      let card = self.pick_available_card();
      self.table.add_card(card);
    }
  }


  fn finalize(&mut self) {
    let active_indexes = self.betting_round.get_unfolded_player_indexes();

    let active_scores = self.active_seats.iter().enumerate()
    .map(|(i, p)| {
      if active_indexes.contains(&(i as u8)) {
        get_hand_score(&self.table, &p.hand)
      } else {
        0
      }
    }).collect::<Vec<u16>>();


    let winning_score = active_scores.iter().max().unwrap();
    let winning_indexes = self.active_seats.iter().enumerate()
      .filter(|(i, _)| active_scores[*i] == *winning_score)
      .map(|(i, _)| i).collect::<Vec<usize>>();

    let pot_splits = self.betting_round.get_pot_split(winning_indexes);
    for (idx, &split) in pot_splits.iter().enumerate() {
      self.active_seats[idx].wallet += split;
    }
  }


  pub fn get_state(&self, player_index: u8) -> GameState {
    let player_bets = self.betting_round.get_player_bets();
    let unfolded_players = self.betting_round.get_unfolded_player_indexes();

    let mut players = [None; 8];
    for (idx, s) in self.active_seats.iter().enumerate() {
      players[s.player_index as usize] = Some(PlayerState {
        wallet: s.wallet,
        money_on_table: player_bets[idx],
        is_folded: !unfolded_players.contains(&(idx as u8)),
      });
    }

    let active_seat_index = self.active_seats.iter().position(|p| p.player_index == player_index);
    let player_seat = if let Some(idx) = active_seat_index { Some(&self.active_seats[idx]) } else { None };

    GameState {
      total_pot: self.betting_round.get_pot(),
      hand: if let Some(s) = player_seat { s.hand } else { Deck::new() },
      table: self.table,
      phase: self.phase,
      wallet: if let Some(s) = player_seat { s.wallet } else { 0 },
      players,
      current_player_index: if let Some(cs) = self.get_current_seat() { Some(cs.player_index) } else { None },
      dealer_index: self.active_seats[self.dealer_index as usize].player_index,
      value_to_call: if let Some(idx) = active_seat_index { self.betting_round.get_player_money_to_call(idx as u8) } else { 0 },
    }
  }

}


impl Iterator for Game {
  type Item = Phase;


  fn next(&mut self) -> Option<Self::Item> {

    if self.get_current_seat().is_some() {
      return Some(self.phase);
    }

    match self.phase {
      Phase::Init => {
        self.init_round();
        self.deal_cards_to_players();
        self.phase = Phase::PreFlop;
      },
      Phase::PreFlop => {
        if self.betting_round.get_num_players_able_to_bets() > 1 {
          self.betting_round.reset_for_next_phase();
          self.deal_cards_to_table(3);

          self.betting_round.set_new_start_position(self.dealer_index + 1);
          self.phase = Phase::Flop;
        } else {
          self.phase = Phase::Showdown;
        }
      }
      Phase::Flop => {
        if self.betting_round.get_num_players_able_to_bets() > 1 {
          self.betting_round.reset_for_next_phase();
          self.deal_cards_to_table(1);
          self.phase = Phase::Turn;
        } else {
          self.phase = Phase::Showdown;
        }
      }
      Phase::Turn => {
        if self.betting_round.get_num_players_able_to_bets() > 1 {
          self.betting_round.reset_for_next_phase();
          self.deal_cards_to_table(1);
          self.phase = Phase::River;
        } else {
          self.phase = Phase::Showdown;
        }
      },
      Phase::River => {
        self.phase = Phase::Showdown;
      },
      Phase::Showdown => {
        let cards_on_table = self.table.get_cards().len() as u8;
        if cards_on_table < 5 {
          self.deal_cards_to_table(5 - cards_on_table);
        }
        self.finalize();
        let valid_players = self.active_seats.iter().filter(|p| p.wallet >= self.blind).count();
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