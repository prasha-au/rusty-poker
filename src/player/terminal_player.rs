use crate::game::{Player, BettingAction, GameInfo};
use text_io::try_read;

pub struct TerminalPlayer {
  pub wallet: u32,
}

impl Player for TerminalPlayer {
  fn get_wallet(&self) -> u32 {
    self.wallet
  }

  fn add_to_wallet(&mut self, v: i32) {
    let new_total = i32::try_from(self.wallet).unwrap() + v;
    self.wallet = u32::try_from(new_total).unwrap();
  }

  fn request_action(&self, info: GameInfo) -> BettingAction {
    println!(
      "Your turn:  WALLET: ${}    POT: ${}   CALL: ${}   HAND: {}   TABLE: {}",
      self.wallet,
      info.total_pot,
      info.value_to_call,
      info.hand,
      info.table
    );

    loop {
      let bet_input: Result<u32, _> = try_read!("{}");
      if bet_input.is_err() {
        continue;
      }

      let bet_amount: u32 = bet_input.unwrap();
      if bet_amount < self.wallet && bet_amount < info.value_to_call {
        continue;
      } else if bet_amount > self.wallet {
        continue;
      }

      println!("we got a bet amount of ${}", bet_amount);
      if bet_amount == self.wallet {
        break BettingAction::AllIn(bet_amount);
      } else if bet_amount > info.value_to_call {
        break BettingAction::Raise(bet_amount);
      } else if bet_amount == 0 && info.value_to_call > 0 {
        break BettingAction::Fold;
      } else {
        break BettingAction::Call;
      }
    }
  }
}
