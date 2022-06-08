
#[derive(Debug)]
pub enum BettingAction {
  Fold,
  Call,
  Raise(u32),
  AllIn(u32),
}

struct PlayerBet {
  money_on_table: u32,
  is_folded: bool,
  is_all_in: bool,
}

impl PlayerBet {
  pub fn is_active(&self) -> bool {
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
        money_on_table: 0,
        is_folded: false,
        is_all_in: false,
      }).collect(),
      current_player_index: 0,
      final_player_index: players - 1,
      is_complete: false,
    }
  }

  pub fn restart(&mut self) {
    self.current_bet = 0;
    self.is_complete = false;
    for p in &mut self.player_bets {
      p.money_on_table = 0;
    }
    self.set_new_start_position(self.current_player_index + 1);
  }


  // TODO: Refactor this into a find circular function
  fn get_prev_active_index(&self, start_index: u8) -> u8 {
    let plens = self.player_bets.len() as u8;
    let mut prev_index = start_index;
    loop {
      prev_index = (plens + prev_index - 1) % plens;
      if self.player_bets[prev_index as usize].is_active() {
        return prev_index;
      }
    }
  }



  pub fn set_new_start_position(&mut self, start_index: u8) {
    println!("num actives {:?}", self.get_active_player_indexes());
    let active_indexes = self.get_active_player_indexes();
    let mut next_index = start_index % self.player_bets.len() as u8;
    loop {
      if active_indexes.contains(&next_index) {
        break;
      }
      next_index = (next_index + 1) % self.player_bets.len() as u8;
    }
    self.current_player_index = next_index;
    self.final_player_index = self.get_prev_active_index(next_index);
    println!("Set new start positions {} {}", self.current_player_index, self.final_player_index);
  }


  pub fn action_current_player(&mut self, action: BettingAction) -> Result<(), &'static str> {
    if self.is_complete {
      return Err("Betting has concluded.");
    }

    let player = &mut self.player_bets[self.current_player_index as usize];
    match action {
      BettingAction::Fold => {
        player.is_folded = true;
      }
      BettingAction::Call => {
        player.money_on_table = self.current_bet;
      }
      BettingAction::Raise(bet) => {
        if bet < self.current_bet {
          return Err("Raise must be greater than current bet");
        }
        player.money_on_table = bet;
        self.current_bet = bet;
        self.final_player_index = self.get_prev_active_index(self.current_player_index);

      }
      BettingAction::AllIn(total) => {
        player.money_on_table = total;
        player.is_all_in = true;
        if total > self.current_bet {
          self.current_bet = total;
          self.final_player_index = self.get_prev_active_index(self.current_player_index);
        }
      }
    };

    println!("Debug yo {} {}", self.current_player_index, self.final_player_index);
    if self.current_player_index == self.final_player_index {
      self.is_complete = true;
      return Ok(());
    }


    let new_final = &self.player_bets[self.final_player_index as usize];

    if !new_final.is_active() {
      panic!("We are setting an invalid item as final player");
    }


    loop {
      self.current_player_index = (self.current_player_index + 1) % self.player_bets.len() as u8;
      let next_player = &self.player_bets[self.current_player_index as usize];
      if !next_player.is_active() {
        continue;
      }
      break;
    }
    Ok(())
  }

  pub fn is_complete(&self) -> bool {
    self.is_complete
  }

  pub fn get_current_player_index(&self) -> u8 {
    self.current_player_index
  }

  pub fn get_current_bet(&self) -> u32 {
    self.current_bet
  }

  pub fn get_player_bets(&self) -> Vec<u32> {
    self.player_bets.iter().map(|p| p.money_on_table).collect()
  }

  pub fn get_active_player_indexes(&self) -> Vec<u8> {
    self.player_bets.iter().enumerate()
      .filter(|(_, p)| p.is_active())
      .map(|(i, _)| i as u8).collect()
  }

  pub fn get_num_active_players(&self) -> u8 {
    self.player_bets.iter().filter(|p| p.is_active()).count() as u8
  }

}




#[cfg(test)]
mod tests;

