use rusty_poker_core::{
  evaluator::{chance_to_win, chance_to_win_preflop},
  game::{EasyBettingAction, GameState, Phase},
  player::Player,
};
use text_io::try_read;

pub struct TerminalPlayer {}

impl Player for TerminalPlayer {
  fn request_action(&self, info: GameState) -> EasyBettingAction {
    let num_players = info.players.iter().filter(|p| p.is_some()).count() as u8;

    println!(
      "Your turn:  WALLET: ${}    POT: ${}   CALL: ${}   HAND: {}   TABLE: {}   EST: {:.2}%",
      info.wallet,
      info.total_pot,
      info.value_to_call,
      info.hand,
      info.table,
      if info.phase > Phase::PreFlop {
        chance_to_win(&info.table, &info.hand) * 100.00
      } else {
        chance_to_win_preflop(&info.hand, num_players)
      }
    );

    loop {
      let bet_input: Result<u32, _> = try_read!("{}");
      if bet_input.is_err() {
        continue;
      }

      let bet_amount: u32 = bet_input.unwrap();
      if bet_amount > info.wallet {
        continue;
      }

      if bet_amount == info.wallet {
        break EasyBettingAction::AllIn;
      } else if bet_amount > info.value_to_call {
        break EasyBettingAction::Raise(bet_amount);
      } else if bet_amount == 0 && info.value_to_call > 0 {
        break EasyBettingAction::Fold;
      } else {
        break EasyBettingAction::Call;
      }
    }
  }
}
