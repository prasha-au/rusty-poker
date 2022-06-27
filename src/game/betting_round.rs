
#[derive(Debug)]
pub enum BettingAction {
  Fold,
  Call,
  Raise(u32),
  AllIn(u32),
}

struct PlayerBet {
  money_in_pot: u32,
  money_on_table: u32,
  is_folded: bool,
  is_all_in: bool,
}

impl PlayerBet {
  pub fn is_able_to_bet(&self) -> bool {
    !self.is_folded && !self.is_all_in
  }
}


pub struct BettingRound {
  current_bet: u32,
  player_bets: Vec<PlayerBet>,
  current_player_index: u8,
  final_player_index: u8,
  is_complete: bool,
}



impl BettingRound {

  pub fn create_for_players(players: u8) -> BettingRound {
    BettingRound {
      current_bet: 0,
      player_bets: (0..players).map(|_| PlayerBet {
        money_in_pot: 0,
        money_on_table: 0,
        is_folded: false,
        is_all_in: false,
      }).collect(),
      current_player_index: 0,
      final_player_index: players - 1,
      is_complete: false,
    }
  }

  pub fn reset_for_next_phase(&mut self) {
    self.current_bet = 0;
    self.is_complete = false;
    for p in &mut self.player_bets {
      p.money_on_table = 0;
    }
    self.set_new_start_position(self.current_player_index + 1);
  }


  fn get_prev_active_index(&self, start_index: u8) -> u8 {
    let total_players = self.player_bets.len() as u8;
    let mut prev_index = start_index;
    loop {
      prev_index = (total_players + prev_index - 1) % total_players;
      if self.player_bets[prev_index as usize].is_able_to_bet() {
        return prev_index;
      }
    }
  }

  pub fn set_new_start_position(&mut self, start_index: u8) {
    let total_players = self.player_bets.len() as u8;
    let mut next_index = start_index % total_players;
    loop {
      let player = &self.player_bets[next_index as usize];
      if player.is_able_to_bet() {
        break;
      }
      next_index = (next_index + 1) % total_players;
    }
    self.current_player_index = next_index;
    self.final_player_index = self.get_prev_active_index(next_index);
  }

  pub fn action_current_player(&mut self, action: BettingAction) -> Result<u32, &'static str> {
    if self.is_complete {
      return Err("Betting has concluded.");
    }

    let previous_player_index = self.get_prev_active_index(self.current_player_index);
    let player = &mut self.player_bets[self.current_player_index as usize];
    let mut value_to_subtract = 0;
    match action {
      BettingAction::Fold => {
        player.is_folded = true;
        if self.get_num_players_able_to_bets() < 2 {
          self.is_complete = true;
        }
      }
      BettingAction::Call => {
        value_to_subtract = self.current_bet - player.money_on_table;
        player.money_on_table = self.current_bet;
        player.money_in_pot += value_to_subtract;
      }
      BettingAction::Raise(bet) => {
        if (player.money_on_table + bet) < self.current_bet {
          return Err("Raise must be greater than current bet");
        }
        value_to_subtract = bet;
        player.money_on_table += bet;
        player.money_in_pot += value_to_subtract;
        self.current_bet = player.money_on_table;
        self.final_player_index = previous_player_index;
      }
      BettingAction::AllIn(remaining_amount) => {
        value_to_subtract = remaining_amount;
        player.money_on_table += remaining_amount;
        player.money_in_pot += value_to_subtract;
        player.is_all_in = true;
        if player.money_on_table > self.current_bet {
          self.current_bet = player.money_on_table;
          self.final_player_index = previous_player_index;
        }
      }
    };

    if self.current_player_index == self.final_player_index {
      self.is_complete = true;
      return Ok(value_to_subtract);
    }

    let new_final_player = &self.player_bets[self.final_player_index as usize];
    if !new_final_player.is_able_to_bet() {
      panic!("We are setting an invalid item as final player");
    }

    loop {
      self.current_player_index = (self.current_player_index + 1) % self.player_bets.len() as u8;
      let next_player = &self.player_bets[self.current_player_index as usize];
      if !next_player.is_able_to_bet() {
        continue;
      }
      break;
    }
    Ok(value_to_subtract)
  }

  pub fn is_complete(&self) -> bool {
    self.is_complete
  }

  pub fn get_current_player_index(&self) -> u8 {
    self.current_player_index
  }

  pub fn get_player_bets(&self) -> Vec<u32> {
    self.player_bets.iter().map(|p| p.money_on_table).collect()
  }

  pub fn get_num_players_able_to_bets(&self) -> u8 {
    self.player_bets.iter().filter(|p| p.is_able_to_bet()).count() as u8
  }

  pub fn get_current_player_money_to_call(&self) -> u32 {
    let player = &self.player_bets[self.current_player_index as usize];
    self.current_bet - player.money_on_table
  }

  pub fn get_num_players_to_act(&self) -> u8 {
    if self.is_complete {
      0
    } else {
      self.final_player_index.abs_diff(self.current_player_index) + 1
    }
  }

  pub fn get_unfolded_player_indexes(&self) -> Vec<u8> {
    self.player_bets.iter().enumerate()
    .filter(|(_, p)| !p.is_folded)
    .map(|(i, _)| i as u8).collect()
  }

  pub fn get_pot(&self) -> u32 {
    self.player_bets.iter().map(|p| p.money_in_pot).sum()
  }

  fn get_player_money_in_pot(&self) -> Vec<u32> {
    self.player_bets.iter().map(|p| p.money_in_pot).collect()
  }


  pub fn get_pot_split(&self, winning_indexes: Vec<usize>) -> Vec<u32> {
    let mut pot_split = vec![0; self.player_bets.len()];

    let player_money_in_pot = self.get_player_money_in_pot();
    let mut split_amounts = player_money_in_pot.to_vec();
    split_amounts.retain(|&v| v > 0);
    split_amounts.sort();
    split_amounts.dedup();

    let mut prev_split_value = 0;
    for split in split_amounts {
      let pot_size = player_money_in_pot.iter().filter(|pb| **pb >= split).collect::<Vec<_>>().len() as u32 * (split - prev_split_value);

      let indexes_eligible_for_split = winning_indexes.iter().filter(|&idx| player_money_in_pot[*idx] >= split).collect::<Vec<_>>();
      let num_splits = indexes_eligible_for_split.len() as u32;
      for &idx in indexes_eligible_for_split {
        pot_split[idx] += pot_size / num_splits;
      }
      prev_split_value = split;
    }

    pot_split
  }

}


#[cfg(test)]
mod tests;
