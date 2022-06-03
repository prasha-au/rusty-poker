
mod card;
mod deck;
// mod hand;
// mod game;
mod evaluator;

use card::*;
// use deck::*;
// use hand::*;
use evaluator::*;


pub fn run_code() {

  println!("Hello, world!");



  init_tables();


  // let cards = [
  //   Card::new(Suit::Diamond, Rank::Six),
  //   Card::new(Suit::Diamond, Rank::Five),
  //   Card::new(Suit::Club, Rank::Ace),
  //   Card::new(Suit::Club, Rank::King),
  //   Card::new(Suit::Club, Rank::Queen),
  //   Card::new(Suit::Club, Rank::Jack),
  //   Card::new(Suit::Club, Rank::Ten),
  // ];
  // evaluate_hand(&cards);


  let cards2 = [
    Card::new(Suit::Heart, Rank::Two),
    Card::new(Suit::Diamond, Rank::Two),
    Card::new(Suit::Diamond, Rank::Three),
    Card::new(Suit::Club, Rank::Four),
    Card::new(Suit::Club, Rank::Jack),
    Card::new(Suit::Spade, Rank::Four),
    Card::new(Suit::Club, Rank::Queen)
  ];
  // evaluate_hand(&cards2);


  // let mut deck = Deck::new();
  // deck.add_card(Card::new(Suit::Heart, Rank::Ace));
  // deck.add_card(Card::new(Suit::Heart, Rank::Five));
  // deck.add_card(Card::new(Suit::Club, Rank::Two));


  // let mut deck2 = Deck::new();
  // deck2.add_card(Card::new(Suit::Spade, Rank::Ace));
  // deck2.add_card(Card::new(Suit::Diamond, Rank::Five));


  // deck.remove_card(Card::new(Suit::Heart, Rank::Ace));

  // let deck3 = deck + deck2;


  // println!("The deck {} {}", deck3, deck3.has_card(Card::new(Suit::Heart, Rank::Ace)));


  // evaluate_deck(&deck3);

}

