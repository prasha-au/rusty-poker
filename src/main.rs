use rusty_poker::*;


fn main() {
  println!("Testing a game!");

  let mut game = Game::create(4, 500);

  let mut rounds_played = 0;
  while let Some(phase) = game.next() {
    if phase == Phase::Showdown {
      rounds_played = rounds_played + 1;
      println!("{}", game);
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

  for p in game.get_all_players().iter() {
    println!("Player {} ends up with ${}", p.id, p.wallet);
  }
  println!("{} rounds played", rounds_played);

  println!("Ran successfully!");

}
