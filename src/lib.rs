
mod card;
mod deck;
mod evaluator;
mod game;

use card::*;
use deck::*;
use evaluator::*;


pub fn run_code() {

  println!("Hello, world!");

  let table = Deck::from_cards(&vec!(
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Heart, Rank::Three),
    Card::new(Suit::Diamond, Rank::Ace),
  ));
  let player = Deck::from_cards(&vec!(
    Card::new(Suit::Diamond, Rank::Queen),
    Card::new(Suit::Diamond, Rank::King),
  ));


  let chance_to_win = chance_to_win(&table, &player);

  println!("My chance to win is {:.5}", chance_to_win);


}

