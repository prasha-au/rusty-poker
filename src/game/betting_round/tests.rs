
use super::*;

#[test]
fn all_players_check() {
  let mut br = BettingRound::create_for_players(2);
  assert_eq!(false, br.is_complete);
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(false, br.is_complete);
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(true, br.is_complete);
  assert_eq!(0, br.current_bet);
}

#[test]
fn calling_a_raise() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(false, br.is_complete);
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(true, br.is_complete);
  assert_eq!(200, br.current_bet);
}


#[test]
fn raising_extends_betting() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Call).unwrap();
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(false, br.is_complete);
}



#[test]
fn skip_players_who_have_folded() {
  let mut br = BettingRound::create_for_players(4);
  br.action_current_player(BettingAction::Call).unwrap();
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::Fold).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(false, br.is_complete);
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(true, br.is_complete);
}


#[test]
fn skip_players_who_have_gone_all_in() {
  let mut br = BettingRound::create_for_players(4);
  br.action_current_player(BettingAction::Call).unwrap();
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::AllIn(100)).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(false, br.is_complete);
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(true, br.is_complete);
}


#[test]
fn player_money_on_table_should_update() {
  let mut br = BettingRound::create_for_players(3);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::AllIn(100)).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(200, br.player_bets[0].money_on_table);
  assert_eq!(100, br.player_bets[1].money_on_table);
  assert_eq!(200, br.player_bets[2].money_on_table);
}


#[test]
fn should_error_when_concluded() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Call).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(Err("Betting has concluded."), br.action_current_player(BettingAction::Call));
}

#[test]
fn correct_value_for_player_bets() {
  let mut br = BettingRound::create_for_players(3);
  br.action_current_player(BettingAction::Call).unwrap();
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::AllIn(100)).unwrap();
  assert_eq!(vec![0, 200, 100], br.get_player_bets());
}



#[test]
fn restarting_bets_resets_values() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::AllIn(100)).unwrap();
  br.reset_for_next_phase();
  assert_eq!(vec![0, 0], br.get_player_bets());
  assert_eq!(0, br.current_bet);
  assert_eq!(false, br.is_complete);
}

#[test]
fn restarting_bets_keeps_players_folded() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::Fold).unwrap();
  br.reset_for_next_phase();
  assert_eq!(true, br.player_bets[1].is_folded);
}

#[test]
fn restarting_bets_keeps_players_all_in() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::AllIn(200)).unwrap();
  br.reset_for_next_phase();
  assert_eq!(true, br.player_bets[1].is_all_in);
}

#[test]
fn setting_new_start_position_ignores_inactive_players() {
  let mut br = BettingRound::create_for_players(4);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::Fold).unwrap();
  br.action_current_player(BettingAction::Fold).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  br.reset_for_next_phase();
  br.set_new_start_position(1);
  assert_eq!(3, br.get_current_player_index());
}


#[test]
fn setting_new_start_position_picks_first_if_available() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  br.reset_for_next_phase();
  br.set_new_start_position(0);
  assert_eq!(0, br.get_current_player_index());
}


#[test]
fn setting_new_start_position_resolves_given_value_circularly() {
  let mut br = BettingRound::create_for_players(2);
  assert_eq!(0, br.get_current_player_index());
  br.reset_for_next_phase();
  br.set_new_start_position(5);
  assert_eq!(1, br.get_current_player_index());
}


#[test]
fn get_num_players_able_to_bets_returns_proper_values() {
  let mut br = BettingRound::create_for_players(4);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::AllIn(200)).unwrap();
  br.action_current_player(BettingAction::Fold).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(2, br.get_num_players_able_to_bets());
}


#[test]
fn get_player_money_to_call_returns_proper_value() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::Raise(400)).unwrap();
  assert_eq!(200, br.get_player_money_to_call(0));
  assert_eq!(0, br.get_player_money_to_call(1));
}


#[test]
fn get_num_players_to_act_returns_proper_values() {
  let mut br = BettingRound::create_for_players(3);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(2, br.get_num_players_to_act());
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(1, br.get_num_players_to_act());
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  assert_eq!(2, br.get_num_players_to_act());
  br.action_current_player(BettingAction::Call).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(0, br.get_num_players_to_act());
}


#[test]
fn if_the_last_opposing_player_folds_it_should_complete() {
  let mut br = BettingRound::create_for_players(2);
  br.set_new_start_position(1);
  br.action_current_player(BettingAction::Fold).unwrap();
  assert_eq!(true, br.is_complete);
}

#[test]
fn if_the_last_opposing_player_went_all_in_do_another_round_of_betting() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::AllIn(200)).unwrap();
  assert_eq!(false, br.is_complete);
}

#[test]
fn get_unfolded_player_indexes_returns_proper_value() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::AllIn(200)).unwrap();
  br.action_current_player(BettingAction::Fold).unwrap();
  assert_eq!(vec![0], br.get_unfolded_player_indexes());
}

#[test]
fn get_pot_returns_proper_value() {
  let mut br = BettingRound::create_for_players(3);
  br.player_bets[0].money_in_pot = 100;
  br.player_bets[1].money_in_pot = 500;
  br.player_bets[2].money_in_pot = 200;
  assert_eq!(800, br.get_pot());
}

#[test]
fn players_retain_money_in_pot_value() {
  let mut br = BettingRound::create_for_players(2);
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  br.reset_for_next_phase();
  br.action_current_player(BettingAction::Raise(200)).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(400, br.player_bets[0].money_in_pot);
  assert_eq!(400, br.player_bets[1].money_in_pot);
}

#[test]
fn should_split_pot_properly_with_all_in_player() {
  let mut br = BettingRound::create_for_players(3);
  br.action_current_player(BettingAction::Raise(400)).unwrap();
  br.action_current_player(BettingAction::AllIn(200)).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(vec![700, 300, 0], br.get_pot_split(vec![0, 1]));
}

#[test]
fn should_split_pot_evenly_between_winners() {
  let mut br = BettingRound::create_for_players(3);
  br.action_current_player(BettingAction::Raise(400)).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  br.action_current_player(BettingAction::Call).unwrap();
  assert_eq!(vec![400, 400, 400], br.get_pot_split(vec![0, 1, 2]));
}

