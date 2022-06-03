use crate::card::*;
use strum::IntoEnumIterator;
use rand::prelude::*;

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

  pub fn from_cards(cards: Vec<Card>) -> Deck {
    let mut value = 0u64;
    for c in cards {
      value |= 1 << u8::from(c);
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

  pub fn pick_available_card(&mut self) -> Card {
    let mut rng = thread_rng();
    // TODO: This isn't the most efficient logic as we have a large wasted range
    loop {
      let idx: u8 = rng.gen_range(0..64);

      if self.value & (1 << idx) > 0 {
        continue;
      }
      let card = Card::try_from(idx);
      if card.is_err() {
        continue;
      }

      let card = card.unwrap();
      self.add_card(card);
      return card;
    }
  }

  pub fn invert(&mut self) {
    self.value = !self.value;
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

