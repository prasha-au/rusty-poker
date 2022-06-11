use rusty_poker::*;


fn main() {
  println!("Testing a game!");

  let mut players = vec![
    CallingPlayer { id: 1, wallet: 200 },
    CallingPlayer { id: 2, wallet: 200 },
    CallingPlayer { id: 3, wallet: 200 }
  ];

  let mut terminal_player = TerminalPlayer { wallet: 200 };


  let mut mutboxes: Vec<Box<&mut dyn Player>> = players.iter_mut().map(|p| -> Box<&mut dyn Player> { Box::new(p) }).collect();
  mutboxes.push(Box::new(&mut terminal_player));



  let mut game = Game::create(mutboxes);


  let mut rounds_played = 0;

  while let Some(phase) = game.next() {
    if phase == Phase::Init {
      rounds_played = rounds_played + 1;
      println!("{}", game);
    }
  }

  for p in players.iter() {
    println!("Player COM{} ends up with ${}", p.id, p.wallet);
  }
  println!("You ended up with ${}", terminal_player.wallet);
  println!("{} rounds played", rounds_played);

  println!("Ran successfully!");

}
