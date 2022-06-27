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
  player_index: usize,
  hand: Deck,
}



pub struct Game<'a> {
  phase: Phase,
  available_cards: Deck,
  table: Deck,
  betting_round: BettingRound,
  dealer_index: u8,
  blind: u32,
  players: Vec<Box<&'a mut dyn Player>>,
  active_seats: Vec<Seat>,
}

#[derive(Copy, Clone)]
pub struct GameInfo {
  pub total_pot: u32,
  pub value_to_call: u32,
  pub hand: Deck,
  pub table: Deck,
  pub phase: Phase,
  pub num_players: u8,
  pub players_to_act: u8,
}


pub trait Player {
  fn get_wallet(&self) -> u32;
  fn add_to_wallet(&mut self, v: i32);
  fn request_action(&self, info: GameInfo) -> BettingAction;
}


impl Game<'_> {

  pub fn create<'a>(players: Vec<Box<&'a mut dyn Player>>) -> Game<'a> {
    let player_count = players.len();
    Game {
      phase: Phase::Init,
      available_cards: Deck::full_deck(),
      table: Deck::new(),
      dealer_index: 0,
      blind: 20,
      players: players,
      active_seats: (0..player_count).map(|player_index| Seat {
        player_index,
        hand: Deck::new()
      }).collect(),
      betting_round: BettingRound::create_for_players(player_count as u8)
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
    if self.betting_round.is_complete() {
      None
    } else {
      Some(&self.active_seats[self.betting_round.get_current_player_index() as usize])
    }
  }


  fn action_current_player(&mut self, action: BettingAction) -> Result<(), &'static str> {
    let player = self.get_current_seat();
    if player.is_none() {
      return Err("This is not the right time to bet.")
    }
    let player_index = player.unwrap().player_index;

    println!("Actioning {:?} for player at index {}", action, player_index);

    let player = &mut self.players[player_index];

    let wallet_value = player.get_wallet();

    if let BettingAction::Raise(amount) = action {
      if amount > wallet_value {
        return Err("Cannot raise more than you have!");
      } else if amount == wallet_value {
        return Err("You must go all in.");
      }
    } else if let BettingAction::Call = action {
      let deficit = self.betting_round.get_current_player_money_to_call();
      if deficit > wallet_value {
        return Err("Cannot call more than you have!");
      } else if deficit == wallet_value {
        return Err("You must go all in.");
      }
    } else if let BettingAction::AllIn(amount) = action {
      if amount != wallet_value {
        return Err("Must go all in with your entire wallet.");
      }
    }

    let new_money = self.betting_round.action_current_player(action).unwrap();
    player.add_to_wallet(-i32::try_from(new_money).unwrap());
    Ok(())
  }


  fn post_blind(&mut self, amount: u32) {
    let player = self.get_current_seat().unwrap();
    if self.players[player.player_index].get_wallet() == amount {
      self.action_current_player(BettingAction::AllIn(amount)).unwrap();
    } else {
      self.action_current_player(BettingAction::Raise(amount)).unwrap();
    }
  }


  fn init_round(&mut self) {
    self.available_cards = Deck::full_deck();
    self.table = Deck::new();
    for i in 0..self.active_seats.len() {
      self.active_seats[i].hand = Deck::new()
    }

    let old_dealer_player_index = self.active_seats[self.dealer_index as usize].player_index;

    while let Some(idx) = self.active_seats.iter().position(|p| self.players[p.player_index].get_wallet() < self.blind) {
      self.active_seats.remove(idx);
    }

    let num_players = self.active_seats.len() as u8;
    println!("We have {} active players this round", num_players);
    if num_players < 2 {
      panic!("We do not have enough players.");
    }

    let total_players = self.players.len();
    for i in 1..total_players {
      let new_dealer_player_index = (old_dealer_player_index + i) % total_players;
      if let Some(_) = self.active_seats.iter().position(|p| p.player_index == new_dealer_player_index) {
        break;
      }
    }

    self.betting_round = BettingRound::create_for_players(num_players);
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

    println!("Table {} Pot ${}", self.table, self.betting_round.get_pot());
    for (idx, seat) in self.active_seats.iter().enumerate() {
      println!(
        "Player {} has {} for {:?} ({}) [{}]",
        seat.player_index,
        seat.hand,
        get_hand_for_score(active_scores[idx]),
        active_scores[idx],
        if winning_indexes.contains(&idx) { 'W' } else { 'L' }
      );
    }


    let pot_splits = self.betting_round.get_pot_split(winning_indexes);
    for (idx, &split) in pot_splits.iter().enumerate() {
      println!("Player at index {} wins ${}", idx, split);
      self.players[self.active_seats[idx].player_index].add_to_wallet(i32::try_from(split).unwrap());
    }
  }
}


impl Iterator for Game<'_> {
  type Item = Phase;


  fn next(&mut self) -> Option<Self::Item> {

    if self.phase != Phase::Init && self.phase != Phase::Showdown {
      if let Some(curr_player) = self.get_current_seat() {
        let action = self.players[curr_player.player_index].request_action(GameInfo {
          total_pot: self.betting_round.get_pot(),
          value_to_call: self.betting_round.get_current_player_money_to_call(),
          hand: self.get_current_seat().unwrap().hand,
          table: self.table,
          phase: self.phase,
          num_players: self.betting_round.get_num_players_able_to_bets(),
          players_to_act: self.betting_round.get_num_players_to_act()
        });
        self.action_current_player(action).unwrap();
        return Some(self.phase);
      }
    }

    match self.phase {
      Phase::Init => {
        println!("========= Init =========");
        self.init_round();
        println!("========= Dealing pre-flop ========");
        self.deal_cards_to_players();
        self.phase = Phase::PreFlop;
      },
      Phase::PreFlop => {
        if self.betting_round.get_num_players_able_to_bets() > 1 {
          self.betting_round.reset_for_next_phase();
          println!("========= Dealing flop ========");
          self.deal_cards_to_table(3);

          self.betting_round.set_new_start_position(self.dealer_index + 1);
          self.phase = Phase::Flop;
        } else {
          println!("Going to showdown");
          self.phase = Phase::Showdown;
        }
      }
      Phase::Flop => {
        if self.betting_round.get_num_players_able_to_bets() > 1 {
          self.betting_round.reset_for_next_phase();
          println!("========= Dealing turn ========");
          self.deal_cards_to_table(1);
          self.phase = Phase::Turn;
        } else {
          self.phase = Phase::Showdown;
        }
      }
      Phase::Turn => {
        if self.betting_round.get_num_players_able_to_bets() > 1 {
          self.betting_round.reset_for_next_phase();
          println!("========= Dealing river ========");
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
        println!("========= Showdown ========");
        let cards_on_table = self.table.get_cards().len() as u8;
        if cards_on_table < 5 {
          self.deal_cards_to_table(5 - cards_on_table);
        }
        self.finalize();
        let valid_players = self.active_seats.iter().filter(|p|
          self.players[p.player_index].get_wallet() >= self.blind).count();
        if valid_players < 2 {
          return None;
        }

        self.phase = Phase::Init;
      }
    };
    Some(self.phase)
  }
}



impl std::fmt::Display for Game<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    writeln!(f, "Pot: ${}   Table: {} ", self.betting_round.get_pot(), self.table)?;
    writeln!(f, "Plyr      Cards      Bid     Wallet")?;
    let player_bets = self.betting_round.get_player_bets();
    let curr_player = self.get_current_seat();
    for (i, p) in self.active_seats.iter().enumerate() {
      writeln!(f, "{} {}{}    {}    ${}     ${}   ",
        p.player_index,
        if p.player_index as u8 == self.dealer_index { 'D' } else { ' ' },
        if curr_player.is_some() && curr_player.unwrap().player_index == p.player_index { 'P' } else { ' ' },
        p.hand,
        player_bets[i],
        self.players[p.player_index].get_wallet()
      )?;
    }
    write!(f, "")
  }
}



#[cfg(test)]
mod tests;
