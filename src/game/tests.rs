use crate::game::*;


#[test]
fn should_simulate_a_game_from_start() {


  for i in 0..100000 {
    let mut game = Game {
      // phase: Phase::PreFlop,
      used_cards: Deck::new(),
      table: Deck::new(),
      players: vec![Deck::new(), Deck::new()]
    };

    simulate_game(&mut game);

    println!("Winner is {}", evaluate_winner(&game));

  }


  assert_eq!(2, 1);

}

