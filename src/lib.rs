
mod card;
mod deck;
mod hand;
mod game;

use card::*;
use deck::*;
use hand::*;

pub fn run_code() {
  let card = Card::new(Suit::Heart, FaceValue::Ace);

  println!("Hello, world! {}", card);



  let mut deck = Deck::new();
  deck.add_card(Card::new(Suit::Heart, FaceValue::Ace));
  deck.add_card(Card::new(Suit::Heart, FaceValue::Five));
  deck.add_card(Card::new(Suit::Club, FaceValue::Two));


  let mut deck2 = Deck::new();
  deck2.add_card(Card::new(Suit::Spade, FaceValue::Ace));
  deck2.add_card(Card::new(Suit::Diamond, FaceValue::Five));


  deck.remove_card(Card::new(Suit::Heart, FaceValue::Ace));

  let deck3 = deck + deck2;


  println!("The deck {} {}", deck3, deck3.has_card(Card::new(Suit::Heart, FaceValue::Ace)));


  evaluate_deck(&deck3);

}

