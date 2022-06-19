use crate::game::{Phase, Player, BettingAction, GameInfo};
use crate::evaluator::{chance_to_win, chance_to_win_preflop};
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
      "Your turn:  WALLET: ${}    POT: ${}   CALL: ${}   HAND: {}   TABLE: {}   EST: {:.2}%",
      self.wallet,
      info.total_pot,
      info.value_to_call,
      info.hand,
      info.table,
      if info.phase > Phase::PreFlop { chance_to_win(&info.table, &info.hand) * 100.00 } else { chance_to_win_preflop(&info.hand, info.num_players) }
    );

    loop {
      let bet_input: Result<u32, _> = try_read!("{}");
      if bet_input.is_err() {
        continue;
      }

      let bet_amount: u32 = bet_input.unwrap();
      if bet_amount > self.wallet {
        continue;
      }

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
