use crate::game::*;
use crate::player::{CallingPlayer};


fn create_players(num_players: u8) -> Vec<CallingPlayer> {
  let mut players = Vec::new();
  for i in 0..num_players {
    players.push(CallingPlayer { id: i });
  }
  players
}

fn to_game_players<'a>(players: &'a mut Vec<CallingPlayer>) -> Vec<Box<&'a mut dyn Player>> {
  players.iter_mut().map(|p| -> Box<&mut dyn Player> { Box::new(p) }).collect()
}


#[test]
fn should_progress_phases() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);

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
  let mut game = Game::create(to_game_players(&mut players), 1000);

  play_round_of_calls(&mut game);

  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);

  assert_eq!(30, game.betting_round.get_pot());
  assert_eq!(0, game.table.get_cards().len());
  for p in &game.active_seats {
    assert_eq!(2, p.hand.get_cards().len());
  }
}


#[test]
fn should_iterate_multiple_rounds() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  for _ in 0..4 {
    assert_eq!(Phase::Init, game.phase);
    play_round_of_calls(&mut game);
  }
}

#[test]
fn should_rotate_the_dealer_on_init() {
  let mut players = create_players(3);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.dealer_index = 1;
  game.next();
  assert_eq!(2, game.dealer_index);
}


#[test]
fn should_skip_inactive_players_when_picking_dealer() {
  let mut players = create_players(4);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.dealer_index = 1;
  game.active_seats[2].wallet = 0;
  game.active_seats[3].wallet = 0;
  game.next();
  assert_eq!(0, game.dealer_index);
}


#[test]
fn should_select_the_player_past_blind_to_start_on_preflop() {
  let mut players = create_players(5);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);
  assert_eq!(0, game.dealer_index);
  assert_eq!(3, game.get_current_seat().unwrap().player_index);
}


#[test]
fn should_select_the_player_past_blind_to_start_on_preflop_circular() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);
  assert_eq!(0, game.dealer_index);
  assert_eq!(1, game.get_current_seat().unwrap().player_index);
}

#[test]
fn should_let_big_blind_bet() {
  let mut players = create_players(3);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  for _ in 0..2 { game.next(); }
  assert_eq!(Some(Phase::PreFlop), game.next());
  assert_eq!(2, game.get_current_seat().unwrap().player_index);
}


#[test]
fn should_select_the_small_blind_player_to_start_on_other_phases() {
  let mut players = create_players(5);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  assert_eq!(Some(Phase::PreFlop), game.next());
  assert_eq!(3, game.get_current_seat().unwrap().player_index);
  for _ in 0..5 { game.next(); }
  assert_eq!(Some(Phase::Flop), game.next());
  assert_eq!(1, game.get_current_seat().unwrap().player_index);
}



#[test]
fn should_only_split_pot_between_players_who_have_not_folded() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);

  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  game.action_current_player(BettingAction::Call).unwrap();

  game.betting_round.reset_for_next_phase();
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  game.action_current_player(BettingAction::Fold).unwrap();

  game.table = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::Ace),
    Card::new(Suit::Heart, Rank::King),
    Card::new(Suit::Heart, Rank::Queen),
    Card::new(Suit::Heart, Rank::Jack),
    Card::new(Suit::Heart, Rank::Four)
  ]);
  game.active_seats[0].hand = Deck::from_cards(&vec![
    Card::new(Suit::Diamond, Rank::Four),
    Card::new(Suit::Diamond, Rank::Three),
  ]);
  game.active_seats[1].hand = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::Ten),
    Card::new(Suit::Diamond, Rank::King),
  ]);

  game.finalize();
  assert_eq!(1200, game.active_seats[0].wallet);
  assert_eq!(800, game.active_seats[1].wallet);
}

#[test]
fn should_decrement_seat_wallet_on_bet() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(800, game.active_seats[0].wallet);
  assert_eq!(1000, game.active_seats[1].wallet);
}



#[test]
fn game_state_should_return_correct_hand() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.active_seats[0].hand = Deck::from_cards(&vec![
    Card::new(Suit::Diamond, Rank::Four),
    Card::new(Suit::Diamond, Rank::Three),
  ]);
  game.active_seats[1].hand = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::Ten),
    Card::new(Suit::Diamond, Rank::King),
  ]);
  let state = game.get_state(0);
  assert_eq!(game.active_seats[0].hand, state.hand);
  let state = game.get_state(1);
  assert_eq!(game.active_seats[1].hand, state.hand);
}

#[test]
fn game_state_should_return_correct_hand_for_inactive_player() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.dealer_index = 0;
  game.active_seats.remove(0);
  assert_eq!(Deck::new(), game.get_state(0).hand);
}


#[test]
fn game_state_should_return_correct_pot() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  assert_eq!(0, game.get_state(0).total_pot);
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(200, game.get_state(0).total_pot);
}

#[test]
fn game_state_should_return_correct_table() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.table = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::Ten),
    Card::new(Suit::Club, Rank::King),
    Card::new(Suit::Diamond, Rank::King),
  ]);
  assert_eq!(game.table, game.get_state(1).table);
}

#[test]
fn game_state_should_return_correct_phase() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.phase = Phase::River;
  assert_eq!(Phase::River, game.get_state(1).phase);
}

#[test]
fn game_state_should_return_correct_wallet() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(800, game.get_state(0).wallet);
}

#[test]
fn game_state_should_return_correct_wallet_for_inactive_player() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.dealer_index = 0;
  game.active_seats.remove(0);
  assert_eq!(0, game.get_state(0).wallet);
}

#[test]
fn game_state_should_return_correct_player_index_of_dealer() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.dealer_index = 0;
  game.active_seats.remove(0);
  assert_eq!(1, game.get_state(0).dealer_index);
}

#[test]
fn game_state_should_return_correct_value_to_call() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(0, game.get_state(0).value_to_call);
  assert_eq!(200, game.get_state(1).value_to_call);
}

#[test]
fn game_state_should_return_correct_player_info() {
  let mut players = create_players(2);
  let mut game = Game::create(to_game_players(&mut players), 1000);
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  game.action_current_player(BettingAction::Fold).unwrap();

  let player_state = game.get_state(0).players;

  let player = player_state[0].unwrap();
  assert_eq!(800, player.wallet);
  assert_eq!(200, player.money_on_table);
  assert_eq!(false, player.is_folded);

  let player = player_state[1].unwrap();
  assert_eq!(1000, player.wallet);
  assert_eq!(0, player.money_on_table);
  assert_eq!(true, player.is_folded);

  for i in 2..8 {
    assert_eq!(true, player_state[i].is_none());
  }
}
