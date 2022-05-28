use crate::card::*;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq)]
pub struct Deck {
  value: u64
}

impl std::fmt::Display for Deck {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    let mut card_strings = Vec::new();
    for i in 0..64 {
      if (self.value & (1 << i)) != 0 {
        card_strings.push(Card::from(i).to_string());
      }
    }
    if card_strings.len() > 0 {
      write!(f, "[ {} ]", card_strings.join(" "))
    } else {
      write!(f, "[ ]")
    }
  }
}


impl Deck {
  pub fn new() -> Deck {
    Deck {
      value: 0
    }
  }

  pub fn add_card(&mut self, card: Card) {
    self.value |= 1 << u8::from(card)
  }

  pub fn remove_card(&mut self, card: Card) {
    self.value &= !(1 << u8::from(card))
  }

  pub fn has_card(&self, card: Card) -> bool {
    self.value & (1 << u8::from(card)) > 0
  }

  pub fn get_suit(&self, suit: Suit) -> u16 {
    (self.value >> (suit as u8)) as u16
  }

  pub fn get_cards(&self) -> Vec<Card> {
    let mut cards = Vec::new();
    for s in Suit::iter() {
      for fv in FaceValue::iter() {
        if self.has_card(Card::new(s, fv)) {
          cards.push(Card::new(s, fv));
        }
      }
    }
    cards
  }

}


impl std::ops::Add for Deck {
  type Output = Deck;

  fn add(self, other: Deck) -> Deck {
    Deck {
      value: self.value | other.value
    }
  }
}

#[cfg(test)]
mod tests;

