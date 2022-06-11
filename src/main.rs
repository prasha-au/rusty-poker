use rusty_poker::*;


fn main() {
  println!("Testing a game!");

  let mut players = vec![
    CallingPlayer { id: 5, wallet: 200 },
    CallingPlayer { id: 6, wallet: 200 },
    CallingPlayer { id: 3, wallet: 200 }
  ];



  let mutboxes = players.iter_mut().map(|p| -> Box<&mut dyn Player> { Box::new(p) }).collect();


  let mut game = Game::create(mutboxes);


  let mut rounds_played = 0;

  while let Some(phase) = game.next() {
    if phase == Phase::Init {
      rounds_played = rounds_played + 1;
      println!("{}", game);
    }
  }

  for p in players.iter() {
    println!("Player {} ends up with ${}", p.id, p.wallet);
  }
  println!("{} rounds played", rounds_played);

  println!("Ran successfully!");

}
