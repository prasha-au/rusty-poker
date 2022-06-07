use crate::game::*;


#[test]
fn should_play_out_a_game() {
  let mut game = Game::create(2);

  game.load_credit(0, 500);
  game.load_credit(1, 500);

  assert_eq!(Phase::Init, game.phase);

  game.next();
  assert_eq!(Phase::PreFlop, game.phase);
  for (i, p) in game.players.iter().enumerate() {
    println!("Player {} {}", i, p.hand);
  }

  game.action_current_player(BettingAction::Raise(50)).unwrap();
  game.action_current_player(BettingAction::Call).unwrap();


  game.next();
  assert_eq!(Phase::Flop, game.phase);
  println!("THE TABLE: {}", game.table);
  game.action_current_player(BettingAction::Call).unwrap();
  game.action_current_player(BettingAction::Raise(50)).unwrap();
  game.action_current_player(BettingAction::Call).unwrap();


  game.next();
  assert_eq!(Phase::Turn, game.phase);
  println!("THE TABLE: {}", game.table);
  game.action_current_player(BettingAction::Call).unwrap();
  game.action_current_player(BettingAction::Call).unwrap();

  game.next();
  assert_eq!(Phase::River, game.phase);
  println!("THE TABLE: {}", game.table);
  game.action_current_player(BettingAction::Raise(50)).unwrap();
  game.action_current_player(BettingAction::Call).unwrap();


  game.next();
  assert_eq!(Phase::Showdown, game.phase);

  for (i, p) in game.players.iter().enumerate() {
    let score = get_hand_score(&game.table, &p.hand);
    println!("Player {} {} {:?} ({:?})", i, p.hand, score, get_hand_for_score(score));
  }

  game.next();
  assert_eq!(Phase::Init, game.phase);


  for (i, p) in game.players.iter().enumerate() {
    println!("Final wallet values {} ${}", i, p.wallet);
  }

  if game.players[0].wallet > game.players[1].wallet {
    assert_eq!(650, game.players[0].wallet);
    assert_eq!(350, game.players[1].wallet);
  } else if game.players[0].wallet < game.players[1].wallet {
    assert_eq!(350, game.players[0].wallet);
    assert_eq!(650, game.players[1].wallet);
  } else {
    assert_eq!(500, game.players[0].wallet);
    assert_eq!(500, game.players[1].wallet);
  }
}


fn play_round_of_calls(game: &mut Game) {
  while game.phase != Phase::Showdown {
    game.next();
    while let Some(_) = game.get_current_player() {
      game.action_current_player(BettingAction::Call).unwrap();
    }
  }
  game.next();
}


#[test]
fn should_reset_state_between_rounds() {
  let mut game = Game::create(2);
  game.load_credit(0, 500);
  game.load_credit(1, 500);
  play_round_of_calls(&mut game);

  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(0, game.pot);
  assert_eq!(0, game.table.get_cards().len());
  for p in &game.players {
    assert_eq!(2, p.hand.get_cards().len());
  }
}



#[test]
fn should_iterate_multiple_rounds() {
  let mut game = Game::create(2);
  game.load_credit(0, 500);
  game.load_credit(1, 500);
  for _ in 0..4 {
    assert_eq!(Phase::Init, game.phase);

    play_round_of_calls(&mut game);

    for (i, p) in game.players.iter().enumerate() {
      println!("Final wallet values {} ${}", i, p.wallet);
    }
  }
}

