use crate::game::*;
use crate::player::{CallingPlayer};


fn create_players(num_players: u8) -> Vec<CallingPlayer> {
  let mut players = Vec::new();
  for i in 0..num_players {
    players.push(CallingPlayer { id: i, wallet: 200 });
  }
  players
}

fn to_game_players<'a>(players: &'a mut Vec<CallingPlayer>) -> Vec<Box<&'a mut dyn Player>> {
  players.iter_mut().map(|p| -> Box<&mut dyn Player> { Box::new(p) }).collect()
}


#[test]
fn should_progress_phases() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players));

  assert_eq!(Phase::Init, game.phase);
  game.next();

  assert_eq!(Phase::PreFlop, game.phase);
  for _ in 0..3 { game.next(); }

  assert_eq!(Phase::Flop, game.phase);
  for _ in 0..3 { game.next(); }

  assert_eq!(Phase::Turn, game.phase);
  for _ in 0..3 { game.next(); }

  assert_eq!(Phase::River, game.phase);
  for _ in 0..3 { game.next(); }

  assert_eq!(Phase::Showdown, game.phase);

  game.next();
  assert_eq!(Phase::Init, game.phase);
}


fn play_round_of_calls(game: &mut Game) {
  while game.phase != Phase::Showdown {
    game.next();
  }
  game.next();
}


#[test]
fn should_reset_state_between_rounds() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players));

  play_round_of_calls(&mut game);

  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);

  assert_eq!(30, game.pot);
  assert_eq!(0, game.table.get_cards().len());
  for p in &game.active_seats {
    assert_eq!(2, p.hand.get_cards().len());
  }
}


#[test]
fn should_iterate_multiple_rounds() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players));
  for _ in 0..4 {
    assert_eq!(Phase::Init, game.phase);
    play_round_of_calls(&mut game);
  }
}


#[test]
fn should_select_the_player_past_blind_to_start_on_preflop() {
  let mut players = create_players(5);
  let mut game = Game::create(to_game_players(&mut players));
  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);
  assert_eq!(0, game.dealer_index);
  assert_eq!(3, game.get_current_seat().unwrap().player_index);
}


#[test]
fn should_select_the_player_past_blind_to_start_on_preflop_circular() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players));
  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);
  assert_eq!(0, game.dealer_index);
  assert_eq!(1, game.get_current_seat().unwrap().player_index);
}

#[test]
fn should_let_big_blind_bet() {
  let mut players = create_players(3);
  let mut game = Game::create(to_game_players(&mut players));
  for _ in 0..2 { game.next(); }
  assert_eq!(Some(Phase::PreFlop), game.next());
  assert_eq!(2, game.get_current_seat().unwrap().player_index);
}


#[test]
fn should_select_the_small_blind_player_to_start_on_other_phases() {
  let mut players = create_players(5);
  let mut game = Game::create(to_game_players(&mut players));
  assert_eq!(Some(Phase::PreFlop), game.next());
  assert_eq!(3, game.get_current_seat().unwrap().player_index);
  for _ in 0..5 { game.next(); }
  assert_eq!(Some(Phase::Flop), game.next());
  assert_eq!(1, game.get_current_seat().unwrap().player_index);
}



#[test]
fn should_only_split_pot_between_players_who_have_not_folded() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players));

  game.phase = Phase::Showdown;
  game.betting_round.set_new_start_position(0);

  game.pot = 200;

  game.betting_round.action_current_player(BettingAction::Raise(200)).unwrap();
  game.betting_round.action_current_player(BettingAction::Fold).unwrap();

  game.table = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::Ace),
    Card::new(Suit::Heart, Rank::King),
    Card::new(Suit::Heart, Rank::Queen),
    Card::new(Suit::Heart, Rank::Jack),
    Card::new(Suit::Heart, Rank::Ten)
  ]);
  game.active_seats[0].hand = Deck::from_cards(&vec![
    Card::new(Suit::Diamond, Rank::Four),
    Card::new(Suit::Diamond, Rank::Three),
  ]);
  game.active_seats[1].hand = Deck::from_cards(&vec![
    Card::new(Suit::Diamond, Rank::Ace),
    Card::new(Suit::Diamond, Rank::King),
  ]);


  game.next();
  assert_eq!(Some(Phase::PreFlop), game.next());


  assert_eq!(380, game.players[0].get_wallet());
  assert_eq!(190, game.players[1].get_wallet());
}

