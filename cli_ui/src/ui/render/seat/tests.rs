use tui::{backend::{TestBackend}, layout::{Rect}, Terminal, buffer::Buffer, style::{Color, Style}};

use super::{TablePosition, render_seat, Seat};



#[test]
fn render_bottom() {
  let backend = TestBackend::new(20, 5);
  let mut terminal = Terminal::new(backend).unwrap();
  terminal.draw(|f| {
    render_seat(f, Rect::new(0, 0, 20, 5), Seat {
      position: TablePosition::Bottom,
      is_turn: false,
      name: "Fred Flinstone".to_string(),
      money_on_table: 20,
      money_in_wallet: 980,
      is_folded: false,
      is_dealer: false,
    });
  }).unwrap();


  let mut expected = Buffer::empty(Rect::new(0, 0, 20, 5));
  expected.set_string(0, 0, "         $20        ", Style::default());
  expected.set_string(0, 1, "                    ", Style::default());
  expected.set_string(0, 2, "        [][]        ", Style::default());
  expected.set_string(3, 3,    "Fred Flinstone"   , Style::default().fg(Color::White));
  expected.set_string(0, 4, "        $980        ", Style::default());

  terminal.backend().assert_buffer(&expected);
}



#[test]
fn render_top() {
  let backend = TestBackend::new(20, 6);
  let mut terminal = Terminal::new(backend).unwrap();
  terminal.draw(|f| {
    render_seat(f, Rect::new(0, 0, 20, 6), Seat {
      position: TablePosition::Top,
      is_turn: false,
      name: "Fred Flinstone".to_string(),
      money_on_table: 20,
      money_in_wallet: 980,
      is_folded: false,
      is_dealer: false,
    });
  }).unwrap();


  let mut expected = Buffer::empty(Rect::new(0, 0, 20, 6));
  expected.set_string(0, 0, "        $980        ", Style::default());
  expected.set_string(3, 1,    "Fred Flinstone"   , Style::default().fg(Color::White));
  expected.set_string(0, 2, "        [][]        ", Style::default());
  expected.set_string(0, 3, "                    ", Style::default());
  expected.set_string(0, 4, "                    ", Style::default());
  expected.set_string(0, 5, "         $20        ", Style::default());

  terminal.backend().assert_buffer(&expected);
}



#[test]
fn render_left() {
  let backend = TestBackend::new(30, 5);
  let mut terminal = Terminal::new(backend).unwrap();
  terminal.draw(|f| {
    render_seat(f, Rect::new(0, 0, 30, 5), Seat {
      position: TablePosition::Left,
      is_turn: false,
      name: "Fred Flinstone".to_string(),
      money_on_table: 20,
      money_in_wallet: 980,
      is_folded: false,
      is_dealer: false,
    });
  }).unwrap();


  let mut expected = Buffer::empty(Rect::new(0, 0, 30, 5));
  expected.set_string(0, 0, "                              ", Style::default());
  expected.set_string(0, 1, "[][]                          ", Style::default());
  expected.set_string(0, 2, "Fred Flinstone", Style::default().fg(Color::White));
  expected.set_string(27, 2, "$20", Style::default());
  expected.set_string(0, 3, "$980                          ", Style::default());
  expected.set_string(0, 4, "                              ", Style::default());
  terminal.backend().assert_buffer(&expected);
}



#[test]
fn render_right() {
  let backend = TestBackend::new(30, 5);
  let mut terminal = Terminal::new(backend).unwrap();
  terminal.draw(|f| {
    render_seat(f, Rect::new(0, 0, 30, 5), Seat {
      position: TablePosition::Right,
      is_turn: false,
      name: "Fred Flinstone".to_string(),
      money_on_table: 20,
      money_in_wallet: 980,
      is_folded: false,
      is_dealer: false,
    });
  }).unwrap();


  let mut expected = Buffer::empty(Rect::new(0, 0, 30, 5));
  expected.set_string(0, 0, "                              ", Style::default());
  expected.set_string(26, 1, "[][]", Style::default());
  expected.set_string(16, 2, "Fred Flinstone", Style::default().fg(Color::White));
  expected.set_string(0, 2, "$20", Style::default());
  expected.set_string(26, 3, "$980", Style::default());
  expected.set_string(0, 4, "                              ", Style::default());
  terminal.backend().assert_buffer(&expected);
}




fn get_standard_seat() -> Seat {
  Seat {
    position: TablePosition::Bottom,
    is_turn: false,
    name: "Fred Flinstone".to_string(),
    money_on_table: 20,
    money_in_wallet: 980,
    is_folded: false,
    is_dealer: false,
  }
}


fn get_standard_buffer() -> Buffer {
  let mut expected = Buffer::empty(Rect::new(0, 0, 20, 5));
  expected.set_string(0, 0, "         $20        ", Style::default());
  expected.set_string(0, 1, "                    ", Style::default());
  expected.set_string(0, 2, "        [][]        ", Style::default());
  expected.set_string(3, 3,    "Fred Flinstone"   , Style::default().fg(Color::White));
  expected.set_string(0, 4, "        $980        ", Style::default());
  expected
}



#[test]
fn render_dealer_button() {
  let backend = TestBackend::new(20, 5);
  let mut terminal = Terminal::new(backend).unwrap();
  terminal.draw(|f| {
    let mut seat = get_standard_seat();
    seat.is_dealer = true;
    render_seat(f, Rect::new(0, 0, 20, 5), seat);
  }).unwrap();
  let mut expected = get_standard_buffer();
  expected.set_string(7, 2, "[][]    ", Style::default());
  expected.set_string(12, 2, "D", Style::default().bg(Color::Yellow).fg(Color::Black));
  terminal.backend().assert_buffer(&expected);
}

#[test]
fn render_no_status_items() {
  let backend = TestBackend::new(20, 5);
  let mut terminal = Terminal::new(backend).unwrap();
  terminal.draw(|f| {
    let mut seat = get_standard_seat();
    seat.is_folded = true;
    render_seat(f, Rect::new(0, 0, 20, 5), seat);
  }).unwrap();
  let mut expected = get_standard_buffer();
  expected.set_string(0, 2, "                    ", Style::default());
  terminal.backend().assert_buffer(&expected);
}


#[test]
fn render_no_money() {
  let backend = TestBackend::new(20, 5);
  let mut terminal = Terminal::new(backend).unwrap();
  terminal.draw(|f| {
    let mut seat = get_standard_seat();
    seat.money_in_wallet = 0;
    render_seat(f, Rect::new(0, 0, 20, 5), seat);
  }).unwrap();
  let mut expected = get_standard_buffer();
  expected.set_string(0, 4, "         $0         ", Style::default());
  terminal.backend().assert_buffer(&expected);
}
