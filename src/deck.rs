use crate::card::*;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Deck {
  value: u64
}

impl std::fmt::Display for Deck {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    let mut card_strings = Vec::new();
    for i in 0..64 {
      if (self.value & (1 << i)) != 0 {
        card_strings.push(Card::try_from(i).unwrap().to_string());
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
    Deck { value: 0 }
  }

  pub fn full_deck() -> Deck {
    Deck { value: 0x000F_FFFF_FFFF_FFFF }
  }


  pub fn from_cards(cards: &Vec<Card>) -> Deck {
    let mut value = 0u64;
    for c in cards {
      value |= 1 << u8::from(*c);
    }
    Deck { value }
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

  pub fn get_cards(&self) -> Vec<Card> {
    let mut cards = Vec::new();
    for s in Suit::iter() {
      for fv in Rank::iter() {
        if self.has_card(Card::new(s, fv)) {
          cards.push(Card::new(s, fv));
        }
      }
    }
    cards
  }

  pub fn get_available_cards(&self) -> Vec<Card> {
    let mut cards = Vec::new();
    for s in Suit::iter() {
      for fv in Rank::iter() {
        if !self.has_card(Card::new(s, fv)) {
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

