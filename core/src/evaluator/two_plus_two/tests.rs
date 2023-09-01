use super::super::test_helpers::evaluator_correctness_tests;
use super::*;
use ntest::timeout;

#[test]
#[timeout(200)]
fn init_two_plus_two_table_should_be_spammable() {
  for _ in 0..5000 {
    init_two_plus_two_table();
  }
}

evaluator_correctness_tests!(evaluate_score, score_to_hand);
