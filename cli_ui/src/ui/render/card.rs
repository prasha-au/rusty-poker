use rusty_poker_core::card::{Card, Suit};
use tui::{
  backend::Backend,
  layout::Alignment,
  layout::Rect,
  style::{Color, Style},
  text::{Span, Spans},
  widgets::Paragraph,
  Frame,
};

pub fn card_to_span(card: &Card) -> Span {
  let color = match card.suit {
    Suit::Club => Color::Green,
    Suit::Diamond => Color::Magenta,
    Suit::Heart => Color::Yellow,
    Suit::Spade => Color::Cyan,
  };

  Span::styled(format!("{}{}", card.suit, card.rank), Style::default().fg(color))
}

pub fn render_card_row<B: Backend>(f: &mut Frame<B>, area: Rect, cards: &Vec<Card>, min_cards: usize) {
  let mut card_spans = cards.iter().map(|card| card_to_span(card)).collect::<Vec<_>>();

  if card_spans.len() < min_cards {
    card_spans.resize(min_cards, Span::raw("[]"));
  }
  let mut flattened = card_spans
    .into_iter()
    .map(|span| vec![span, Span::raw(" ")])
    .flatten()
    .collect::<Vec<_>>();
  flattened.pop();

  let table_cards = Paragraph::new(Spans::from(flattened))
    .style(Style::default().fg(Color::White))
    .alignment(Alignment::Center);

  f.render_widget(table_cards, area);
}
