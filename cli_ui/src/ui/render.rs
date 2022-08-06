use rusty_poker_core::{game::{GameState}};
use tui::{
  backend::{Backend},
  style::{Color, Style},
  text::{Span, Spans},
  widgets::{
      canvas::{Canvas, Rectangle},
      Paragraph, Wrap
  }, layout::Rect,
  layout::{Alignment}, Frame
};

use self::{seat::{TablePosition, render_seat, Seat}, card::{render_card_row, card_to_span}};

mod seat;
mod card;


fn draw_player<B: Backend>(f: &mut Frame<B>, table_area: Rect, position: u8,  name: String, player_index: u8, game_state: &GameState) {
  let rect_w = 20;
  let rect_h = 6;

  let x_offset = 10;
  let y_offset = 3;

  let one_third_x = table_area.width / 3 - rect_w / 2;
  let two_third_x = table_area.width / 3 * 2 - rect_w / 2;
  let one_third_y = table_area.height / 3;
  let two_third_y = table_area.height / 3 * 2;

  let rect = match position {
    0 => Rect { x: one_third_x, y: table_area.height - rect_h - y_offset, width: rect_w, height: rect_h },
    7 => Rect { x: two_third_x, y: table_area.height - rect_h - y_offset, width: rect_w, height: rect_h },
    1 => Rect { x: x_offset, y: two_third_y, width: rect_w, height: rect_h },
    2 => Rect { x: x_offset, y: one_third_y, width: rect_w, height: rect_h },
    3 => Rect { x: one_third_x, y: y_offset - 1, width: rect_w, height: rect_h  },
    4 => Rect { x: two_third_x, y: y_offset - 1, width: rect_w, height: rect_h },
    5 => Rect { x: table_area.width - rect_w - x_offset, y: one_third_y, width: rect_w, height: rect_h },
    6 => Rect { x: table_area.width - rect_w - x_offset, y: two_third_y, width: rect_w, height: rect_h },
    _ => Rect { x: 0, y: 0, width: rect_w, height: rect_h },
  };

  let alignment = match position {
    0 | 8 => TablePosition::Bottom,
    1..=2 => TablePosition::Left,
    3..=4  => TablePosition::Top,
    5..=6 => TablePosition::Right,
    _ => TablePosition::Bottom
  };

  let player_state = game_state.players[player_index as usize].unwrap();

  render_seat(f, rect, Seat {
    position: alignment,
    is_turn: game_state.current_player_index == Some(player_index),
    name: name,
    money_on_table: player_state.money_on_table,
    money_in_wallet: player_state.wallet,
    is_folded: player_state.is_folded,
    is_dealer: game_state.dealer_index == player_index,
  });

}


pub fn draw_table<B: Backend>(f: &mut Frame<B>, game_state: &GameState, table_area: Rect) {
  let canvas = Canvas::default()
  .paint(|ctx| {
    ctx.draw(&Rectangle {
      x: 10.0,
      y: 10.0,
      width: 180.0,
      height: 160.0,
      color: Color::Green,
    });
  })
  .x_bounds([0.0, 200.0])
  .y_bounds([0.0, 180.0]);

  f.render_widget(canvas, table_area);


  render_card_row(
    f,
    Rect { x: table_area.width / 2 - 15, y: table_area.height / 2, width: 30, height: 2},
    &game_state.table.get_cards(),
    3
  );

  let pot_display = Paragraph::new(vec![
    Spans::from(Span::raw(format!("${}", game_state.total_pot))),
  ])
  .style(Style::default().fg(Color::White))
  .alignment(Alignment::Center);

  f.render_widget(pot_display, Rect { x: table_area.width / 2 - 15, y: table_area.height / 2 + 2, width: 30, height: 2});

  for (i, p) in game_state.players.iter().enumerate() {
    if p.is_some() {
      let name = if i == 0 { String::from("You") } else { format!("Computer {}", i + 1) };
      draw_player(f, table_area, i as u8, name, i as u8, game_state);
    }
  }
}


pub fn draw_player_info<B: Backend>(f: &mut Frame<B>, game: &GameState, area: Rect) {
  let hand_cards = game.hand.get_cards();

  let mut text_spans = hand_cards.iter().map(|card| card_to_span(card)).collect::<Vec<_>>();
  if text_spans.len() > 0 {
    text_spans.insert(1, Span::raw(" "));
  }
  text_spans.push(Span::raw("   "));
  text_spans.push(Span::styled(format!("${}", game.wallet), Style::default()));

  let text = vec![
    Spans::from(text_spans)
  ];

  let player_display = Paragraph::new(text)
  .style(Style::default().fg(Color::White))
  .alignment(Alignment::Left)
  .wrap(Wrap { trim: true });

  f.render_widget(player_display, area);
}
