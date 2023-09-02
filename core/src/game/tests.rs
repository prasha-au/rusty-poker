use crate::game::*;

fn call_and_next(game: &mut Game) {
  game.action_current_player(BettingAction::Call).unwrap();
  game.next();
}

#[test]
fn should_progress_phases() {
  let mut game = Game::create(2, 1000);

  assert_eq!(Phase::Init, game.phase);
  game.next();

  assert_eq!(Phase::PreFlop, game.phase);
  for _ in 0..2 {
    call_and_next(&mut game);
  }

  assert_eq!(Phase::Flop, game.phase);
  for _ in 0..2 {
    call_and_next(&mut game);
  }

  assert_eq!(Phase::Turn, game.phase);
  for _ in 0..2 {
    call_and_next(&mut game);
  }

  assert_eq!(Phase::River, game.phase);
  for _ in 0..2 {
    call_and_next(&mut game);
  }

  assert_eq!(Phase::Showdown, game.phase);

  game.next();
  assert_eq!(Phase::Init, game.phase);
}

fn play_a_game_of_calls(game: &mut Game) {
  game.next();
  while game.phase != Phase::Showdown {
    call_and_next(game);
  }
  game.next();
}

#[test]
fn should_reset_state_between_rounds() {
  let mut game = Game::create(2, 1000);

  play_a_game_of_calls(&mut game);

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
  let mut game = Game::create(2, 1000);
  assert_eq!(Phase::Init, game.phase);
  for _ in 0..4 {
    play_a_game_of_calls(&mut game);
    assert_eq!(Phase::Init, game.phase);
  }
}

#[test]
fn should_rotate_the_dealer_on_init() {
  let mut game = Game::create(3, 1000);
  game.dealer_index = 1;
  game.next();
  assert_eq!(2, game.dealer_index);
}

#[test]
fn should_skip_inactive_players_when_picking_dealer() {
  let mut game = Game::create(4, 1000);
  game.dealer_index = 1;
  game.active_seats[2].wallet = 0;
  game.active_seats[3].wallet = 0;
  game.next();
  assert_eq!(0, game.dealer_index);
}

#[test]
fn should_select_the_player_past_blind_to_start_on_preflop() {
  let mut game = Game::create(5, 1000);
  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);
  assert_eq!(0, game.dealer_index);
  assert_eq!(3, game.get_current_seat().unwrap().player_index);
}

#[test]
fn should_select_the_player_past_blind_to_start_on_preflop_circular() {
  let mut game = Game::create(2, 1000);
  assert_eq!(Phase::Init, game.phase);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);
  assert_eq!(0, game.dealer_index);
  assert_eq!(1, game.get_current_seat().unwrap().player_index);
}

#[test]
fn should_let_big_blind_bet() {
  let mut game = Game::create(3, 1000);
  game.next();
  assert_eq!(Phase::PreFlop, game.phase);
  for _ in 0..2 {
    call_and_next(&mut game);
  }
  assert_eq!(Some(Phase::PreFlop), game.next());
  assert_eq!(2, game.get_current_seat().unwrap().player_index);
}

#[test]
fn should_select_the_small_blind_player_to_start_on_other_phases() {
  let mut game = Game::create(5, 1000);
  assert_eq!(Some(Phase::PreFlop), game.next());
  assert_eq!(3, game.get_current_seat().unwrap().player_index);
  for _ in 0..5 {
    call_and_next(&mut game);
  }
  assert_eq!(Some(Phase::Flop), game.next());
  assert_eq!(1, game.get_current_seat().unwrap().player_index);
}

#[test]
fn should_only_split_pot_between_players_who_have_not_folded() {
  let mut game = Game::create(2, 1000);

  game.phase = Phase::PreFlop;
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
    Card::new(Suit::Heart, Rank::Four),
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
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(800, game.active_seats[0].wallet);
  assert_eq!(1000, game.active_seats[1].wallet);
}

#[test]
fn should_return_current_player_index_properly() {
  let mut game = Game::create(3, 1000);
  game.active_seats.remove(0);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  assert_eq!(Some(1), game.get_current_player_index());
}

#[test]
fn should_return_current_player_index_as_none_when_between_phases() {
  let mut game = Game::create(3, 1000);
  game.phase = Phase::Init;
  assert_eq!(None, game.get_current_player_index());
  game.phase = Phase::Showdown;
  assert_eq!(None, game.get_current_player_index());
}

#[test]
fn game_state_should_return_correct_hand() {
  let mut game = Game::create(2, 1000);
  game.active_seats[0].hand = Deck::from_cards(&vec![
    Card::new(Suit::Diamond, Rank::Four),
    Card::new(Suit::Diamond, Rank::Three),
  ]);
  game.active_seats[1].hand = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::Ten),
    Card::new(Suit::Diamond, Rank::King),
  ]);
  let state = game.get_state(Some(0));
  assert_eq!(game.active_seats[0].hand, state.hand);
  let state = game.get_state(Some(1));
  assert_eq!(game.active_seats[1].hand, state.hand);
}

#[test]
fn game_state_should_return_correct_hand_for_inactive_player() {
  let mut game = Game::create(2, 1000);
  game.dealer_index = 0;
  game.active_seats.remove(0);
  assert_eq!(Deck::new(), game.get_state(Some(0)).hand);
}

#[test]
fn game_state_should_return_correct_pot() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  assert_eq!(0, game.get_state(Some(0)).total_pot);
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(200, game.get_state(Some(0)).total_pot);
}

#[test]
fn game_state_should_return_correct_table() {
  let mut game = Game::create(2, 1000);
  game.table = Deck::from_cards(&vec![
    Card::new(Suit::Heart, Rank::Ten),
    Card::new(Suit::Club, Rank::King),
    Card::new(Suit::Diamond, Rank::King),
  ]);
  assert_eq!(game.table, game.get_state(Some(1)).table);
}

#[test]
fn game_state_should_return_correct_phase() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::River;
  assert_eq!(Phase::River, game.get_state(Some(1)).phase);
}

#[test]
fn game_state_should_return_correct_wallet() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(800, game.get_state(Some(0)).wallet);
}

#[test]
fn game_state_should_return_correct_wallet_for_inactive_player() {
  let mut game = Game::create(2, 1000);
  game.dealer_index = 0;
  game.active_seats.remove(0);
  assert_eq!(0, game.get_state(Some(0)).wallet);
}

#[test]
fn game_state_should_return_correct_player_index_of_dealer() {
  let mut game = Game::create(2, 1000);
  game.dealer_index = 0;
  game.active_seats.remove(0);
  assert_eq!(1, game.get_state(Some(0)).dealer_index);
}

#[test]
fn game_state_should_return_correct_value_to_call() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(0, game.get_state(Some(0)).value_to_call);
  assert_eq!(200, game.get_state(Some(1)).value_to_call);
}

#[test]
fn game_state_should_return_correct_player_info() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(200)).unwrap();
  game.action_current_player(BettingAction::Fold).unwrap();

  let player_state = game.get_state(Some(0)).players;

  let player = player_state[0];
  assert_eq!(800, player.wallet);
  assert_eq!(200, player.money_on_table);
  assert_eq!(false, player.is_folded);

  let player = player_state[1];
  assert_eq!(1000, player.wallet);
  assert_eq!(0, player.money_on_table);
  assert_eq!(true, player.is_folded);

  assert_eq!(2, player_state.len());
}

#[test]
fn game_when_actioning_player_should_use_whole_wallet_when_going_all_in() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(2000)).unwrap();
  assert_eq!(0, game.get_state(Some(0)).players[0].wallet);
}

#[test]
fn game_when_actioning_player_should_raise_by_amount_given() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(20)).unwrap();
  game.action_current_player(BettingAction::Raise(40)).unwrap();
  let game_state = game.get_state(None);
  assert_eq!(80, game_state.total_pot);
  assert_eq!(20, game_state.players[0].money_on_table);
  assert_eq!(60, game_state.players[1].money_on_table);
}

#[test]
fn game_when_actioning_player_should_convert_raise_to_call() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  assert!(game.action_current_player(BettingAction::Raise(0)).is_ok());
}

#[test]
fn game_when_actioning_player_should_convert_raise_to_all_in() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(2000)).unwrap();
  assert_eq!(0, game.get_state(Some(0)).players[0].wallet);
  game.action_current_player(BettingAction::Raise(5000)).unwrap();
  assert_eq!(0, game.get_state(Some(1)).players[1].wallet);
  game.next();
  assert_eq!(2000, game.get_state(None).total_pot);
  assert_eq!(Phase::Showdown, game.get_state(None).phase);
}

#[test]
fn game_when_actioning_player_should_convert_call_to_all_in() {
  let mut game = Game::create(2, 1000);
  game.phase = Phase::PreFlop;
  game.betting_round.set_new_start_position(0);
  game.action_current_player(BettingAction::Raise(1000)).unwrap();
  game.action_current_player(BettingAction::Call).unwrap();
  game.next();
  assert_eq!(Phase::Showdown, game.get_state(None).phase);
}
