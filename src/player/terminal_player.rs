use crate::game::{Player, BettingAction};
use crate::deck::{Deck};
use text_io::scan;

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

  fn request_action(&self, total_pot: u32, value_to_call: u32, hand: Deck, table: Deck) -> BettingAction {
    println!("Your turn:  WALLET: ${}    POT: ${}   CALL: ${}   HAND: {}   TABLE: {}", self.wallet, total_pot, value_to_call, hand, table);
    let bet_amount: u32;
    scan!("{}", bet_amount);

    println!("we got a bet amount of ${}", bet_amount);


    if bet_amount == self.wallet {
      BettingAction::AllIn(bet_amount)
    } else if bet_amount > value_to_call {
      BettingAction::Raise(bet_amount)
    } else if bet_amount == 0 && value_to_call > 0 {
      BettingAction::Fold
    } else {
      BettingAction::Call
    }

  }
}


