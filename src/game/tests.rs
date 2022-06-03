use crate::game::*;
use crate::card::*;


// #[test]
// fn should_simulate_a_game_from_start() {


//   for i in 0..100000 {
//     println!("Game number {} ", i);
//     let mut game = Game {
//       // phase: Phase::PreFlop,
//       used_cards: Deck::new(),
//       table: Deck::new(),
//       players: vec![Deck::new(), Deck::new()]
//     };

//     simulate_game(&mut game);

//     println!("Winner is {}", evaluate_winner(&game));

//   }


//   // assert_eq!(2, 1);

// }




#[test]
fn should_calculate_odds() {

  let table = Deck::from_cards(vec![
    Card::new(Suit::Heart, FaceValue::Two),
    Card::new(Suit::Heart, FaceValue::Three),
    Card::new(Suit::Heart, FaceValue::Four),
    // Card::new(Suit::Diamond, FaceValue::Four),
    // Card::new(Suit::Diamond, FaceValue::Six),
  ]);

  let player = Deck::from_cards(vec![
    Card::new(Suit::Heart, FaceValue::Seven),
    Card::new(Suit::Heart, FaceValue::Six),
  ]);


  calculate_odds(&table, &player);


  assert_eq!(2, 1);

}

