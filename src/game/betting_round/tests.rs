
use super::*;

#[test]
fn all_players_check() {
  let mut br = BettingRound::create_for_players(2);
  assert_eq!(false, br.is_complete());
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(false, br.is_complete());
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(true, br.is_complete());
  assert_eq!(0, br.current_bet);
}

#[test]
fn calling_a_raise() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(CurrentPlayerAction::Raise(200)).unwrap();
  assert_eq!(false, br.is_complete());
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(true, br.is_complete());
  assert_eq!(200, br.current_bet);
}


#[test]
fn raising_extends_betting() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  br.action_current_player(CurrentPlayerAction::Raise(200)).unwrap();
  assert_eq!(false, br.is_complete());
}



#[test]
fn skip_players_who_have_folded() {
  let mut br = BettingRound::create_for_players(4);
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  br.action_current_player(CurrentPlayerAction::Raise(200)).unwrap();
  br.action_current_player(CurrentPlayerAction::Fold).unwrap();
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(false, br.is_complete());
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(true, br.is_complete());
}


#[test]
fn skip_players_who_have_gone_all_in() {
  let mut br = BettingRound::create_for_players(4);
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  br.action_current_player(CurrentPlayerAction::Raise(200)).unwrap();
  br.action_current_player(CurrentPlayerAction::AllIn(100)).unwrap();
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(false, br.is_complete());
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(true, br.is_complete());
}


#[test]
fn player_money_on_table_should_update() {
  let mut br = BettingRound::create_for_players(3);
  br.action_current_player(CurrentPlayerAction::Raise(200)).unwrap();
  br.action_current_player(CurrentPlayerAction::AllIn(100)).unwrap();
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(200, br.player_bets[0].money_on_table);
  assert_eq!(100, br.player_bets[1].money_on_table);
  assert_eq!(200, br.player_bets[2].money_on_table);
}


#[test]
fn should_error_when_concluded() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(Err("Betting has concluded."), br.action_current_player(CurrentPlayerAction::Call));
}

#[test]
fn start_players_folded() {
  let mut br = BettingRound::create_for_players(3);
  br.set_player_folded(1);
  br.set_player_folded(2);
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  assert_eq!(true, br.is_complete());
}

#[test]
fn correct_value_for_player_bets() {
  let mut br = BettingRound::create_for_players(3);
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  br.action_current_player(CurrentPlayerAction::Raise(200)).unwrap();
  br.action_current_player(CurrentPlayerAction::AllIn(100)).unwrap();
  assert_eq!(vec![0, 200, 100], br.get_player_bets());
}


#[test]
fn correct_value_for_active_player_indexes() {
  let mut br = BettingRound::create_for_players(3);
  br.action_current_player(CurrentPlayerAction::Fold).unwrap();
  br.action_current_player(CurrentPlayerAction::Fold).unwrap();
  br.action_current_player(CurrentPlayerAction::Call).unwrap();
  let active_indexes = br.get_active_player_indexes();
  assert_eq!(vec![2], active_indexes);
}

