use rusty_poker::*;


fn main() {
  println!("Testing a game!");

  let mut game = Game::create(2);
  game.load_credit(0, 500);
  game.load_credit(1, 500);

  while let Some(phase) = game.next() {
    if phase == Phase::Showdown || phase == Phase::Init {
      continue;
    }
    while let Some(_) = game.get_current_player() {
      // This will go to error
      game.action_current_player(BettingAction::Call).unwrap();
    }

  }

  game.next();

  println!("Ran successfully!");

}
