use super::Player;
use crate::game::{EasyBettingAction, GameState};

pub struct CallingPlayer {
  pub id: u8,
}

impl Player for CallingPlayer {
  fn request_action(&self, info: GameState) -> EasyBettingAction {
    if info.wallet > info.value_to_call {
      EasyBettingAction::Call
    } else {
      EasyBettingAction::AllIn
    }
  }
}
