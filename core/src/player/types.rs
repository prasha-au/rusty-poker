use crate::game::{BettingAction, GameState};

pub trait Player {
  fn request_action(&self, info: GameState) -> BettingAction;
}
