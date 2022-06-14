use crate::game::{Player,BettingAction,GameInfo};

pub struct CallingPlayer {
  pub id: u8,
  pub wallet: u32,
}

impl Player for CallingPlayer {
  fn get_wallet(&self) -> u32 {
    self.wallet
  }

  fn add_to_wallet(&mut self, v: i32) {
    let new_total = i32::try_from(self.wallet).unwrap() + v;
    self.wallet = u32::try_from(new_total).unwrap();
  }

  fn request_action(&self, info: GameInfo) -> BettingAction {
    if self.wallet > info.value_to_call {
      BettingAction::Call
    } else {
      BettingAction::AllIn(self.wallet)
    }
  }
}

