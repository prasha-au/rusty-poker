use crate::game::{EasyBettingAction, GameState};

pub trait Player {
  fn request_action(&self, info: GameState) -> EasyBettingAction;
}
