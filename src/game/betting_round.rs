
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
      final_player_index: 0,
      is_complete: false,
    }
  }

  // TODO: Test this
  pub fn initialize(&mut self, start_on: u8) {
    self.current_bet = 0;
    self.is_complete = false;
    for p in &mut self.player_bets {
      p.money_on_table = 0;
    }
    self.current_player_index = start_on;
    self.final_player_index = start_on;
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
        self.final_player_index = self.current_player_index;
      }
      BettingAction::AllIn(total) => {
        player.money_on_table = total;
        player.is_all_in = true;
        if total > self.current_bet {
          self.current_bet = total;
          self.final_player_index = self.current_player_index;
        }
      }
    };

    loop {
      self.current_player_index = (self.current_player_index + 1) % self.player_bets.len() as u8;
      let next_player = &self.player_bets[self.current_player_index as usize];
      if next_player.is_folded || next_player.is_all_in {
        continue;
      }
      if self.current_player_index == self.final_player_index {
        self.is_complete = true;
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


  pub fn set_player_folded(&mut self, player: u8) {
    self.player_bets[player as usize].is_folded = true;
  }

  pub fn get_player_bets(&self) -> Vec<u32> {
    self.player_bets.iter().map(|p| p.money_on_table).collect()
  }

  // TODO: Test this
  pub fn get_active_player_indexes(&self) -> Vec<u8> {
    let mut active_indexes = Vec::new();
    for (i, p) in self.player_bets.iter().enumerate() {
      if !p.is_folded && !p.is_all_in {
        active_indexes.push(i as u8);
      }
    }
    active_indexes
  }


}




#[cfg(test)]
mod tests;

