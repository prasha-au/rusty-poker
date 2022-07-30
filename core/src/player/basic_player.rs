use super::{Player};
use crate::game::{Phase,BettingAction,GameState};
use crate::evaluator::{chance_to_win, chance_to_win_preflop};

pub struct BasicPlayer {
  pub id: u8,
}

impl Player for BasicPlayer {
  fn request_action(&self, info: GameState) -> BettingAction {
    let raise_or_call = |amount: u32| -> BettingAction {
      if amount > info.wallet {
        BettingAction::AllIn(info.wallet)
      } else if amount > info.value_to_call {
        BettingAction::Raise(amount - info.value_to_call)
      } else {
        BettingAction::Call
      }
    };


    let num_players = info.players.iter().filter(|p| p.is_some()).count() as u8;
    match info.phase {
      Phase::PreFlop => {
        let odds = chance_to_win_preflop(&info.hand, num_players);
        if odds > 60.00 {
          raise_or_call(info.total_pot)
        } else if info.value_to_call == 0 {
          raise_or_call(0)
        } else if odds > 20.00 {
          raise_or_call(info.value_to_call)
        } else {
          BettingAction::Fold
        }
      },
      Phase::Flop | Phase::River | Phase::Turn => {
        let odds = chance_to_win(&info.table, &info.hand);
        if odds > 70.00 {
          raise_or_call(info.total_pot)
        } else if odds > 50.00 {
          raise_or_call(info.value_to_call)
        } else {
          BettingAction::Fold
        }
      },
      _ => {
        panic!("Invalid phase to bet on.");
      }
    }
  }


}

