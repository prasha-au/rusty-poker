use rusty_poker::*;



fn iterate_with_player(game: &mut Game, players: &Vec<Box<&mut dyn Player>>) -> Option<Phase> {
  if let Some(curr_index) = game.get_current_player_index() {
    let action = players[curr_index as usize].request_action(game.get_state(curr_index));
    game.action_current_player(action).unwrap();
  }
  game.next()
}



fn main() {
  println!("Testing a game!");

  let mut calling_players = vec![
    CallingPlayer { id: 1 },
    CallingPlayer { id: 2 },
    CallingPlayer { id: 3 }
  ];

  let mut _com_player = BasicPlayer { id: 1 };
  let mut terminal_player = TerminalPlayer {  };



  let players: Vec<Box<&mut dyn Player>> = vec![
    Box::new(&mut calling_players[0]),
    Box::new(&mut terminal_player)
  ];


  let mut game = Game::create(2, 200);


  let mut rounds_played = 1;

  println!("==== Round {} ============================================================================================", rounds_played);
  while let Some(phase) = iterate_with_player(&mut game, &players) {
    if phase == Phase::Init {
      rounds_played = rounds_played + 1;
      println!("==== Round {} ============================================================================================", rounds_played);
    }
  }

  // println!("Player COM{} ends up with ${}", calling_players[0].id, calling_players[0].wallet);

  // for p in calling_players.iter() {
  //   println!("Player COM{} ends up with ${}", p.id, p.wallet);
  // }
  // println!("You ended up with ${}", terminal_player.wallet);
  println!("{} rounds played", rounds_played);

  println!("Ran successfully!");

}
