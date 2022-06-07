use rusty_poker::*;


fn main() {
  println!("Testing a game!");

  let mut game = Game::create(4);
  game.load_credit(0, 500);
  game.load_credit(1, 500);
  game.load_credit(2, 500);
  game.load_credit(3, 500);

  let mut rounds_played = 0;
  while let Some(phase) = game.next() {
    if phase == Phase::Showdown {
      rounds_played = rounds_played + 1;
    }
    if phase == Phase::Showdown || phase == Phase::Init {
      continue;
    }
    while let Some(player) = game.get_current_player() {
      let current_bet = game.get_current_bet();
      let action = if player.wallet > current_bet {
        BettingAction::Call
      } else {
        BettingAction::AllIn(player.wallet)
      };
      game.action_current_player(action).unwrap();
    }
  }

  for (i, p) in game.get_players().iter().enumerate() {
    println!("Player {} ends up with ${}", i, p.wallet);
  }
  println!("{} rounds played", rounds_played);

  println!("Ran successfully!");

}
