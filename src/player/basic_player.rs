use crate::game::{Phase,Player,BettingAction,GameInfo};
use crate::evaluator::{chance_to_win, chance_to_win_preflop};

pub struct BasicPlayer {
  pub id: u8,
  pub wallet: u32,
}

impl Player for BasicPlayer {
  fn get_wallet(&self) -> u32 {
    self.wallet
  }

  fn add_to_wallet(&mut self, v: i32) {
    let new_total = i32::try_from(self.wallet).unwrap() + v;
    self.wallet = u32::try_from(new_total).unwrap();
  }



  fn request_action(&self, info: GameInfo) -> BettingAction {

    let raise_or_call = |amount: u32| -> BettingAction {
      if amount > self.wallet {
        BettingAction::AllIn(self.wallet)
      } else if amount > info.value_to_call {
        BettingAction::Raise(amount - info.value_to_call)
      } else {
        BettingAction::Call
      }
    };




    match info.phase {
      Phase::PreFlop => {
        let odds = chance_to_win_preflop(&info.hand, info.num_players);
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

