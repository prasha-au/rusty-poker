use super::Player;
use crate::game::{BettingAction, GameState};

pub struct CallingPlayer {
  pub id: u8,
}

impl Player for CallingPlayer {
  fn request_action(&self, info: GameState) -> BettingAction {
    if info.wallet > info.value_to_call {
      BettingAction::Call
    } else {
      BettingAction::AllIn
    }
  }
}
