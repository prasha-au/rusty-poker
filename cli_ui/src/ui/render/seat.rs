use tui::{backend::{Backend}, layout::{Rect, Alignment}, Frame, widgets::Paragraph, style::{Color, Style}, text::{Span, Spans}};


#[derive(PartialEq)]
pub enum TablePosition {
  Top,
  Bottom,
  Left,
  Right
}


pub struct Seat {
  pub position: TablePosition,
  pub is_turn: bool,
  pub name: String,
  pub money_on_table: u32,
  pub money_in_wallet: u32,
  pub is_folded: bool,
  pub is_dealer: bool,
}


pub fn render_seat<B: Backend>(f: &mut Frame<B>, area: Rect, seat: Seat) {
  let name = Span::styled(seat.name, Style::default().fg(if seat.is_turn { Color::Yellow } else { Color::White }));

  let money_in_wallet = Span::raw(format!("${}", seat.money_in_wallet));

  let mut status_spans = vec![];
  if !seat.is_folded {
    status_spans.push(Span::raw("[][]"));
  }
  if seat.is_dealer {
    if status_spans.len() > 0 {
      status_spans.push(Span::raw(" "));
    }
    status_spans.push(Span::styled("D", Style::default().bg(Color::Yellow).fg(Color::Black)));
  }

  let money_on_table = if seat.money_on_table > 0 {
    Some(Span::raw(format!("${}", seat.money_on_table)))
  } else {
    None
  };


  match seat.position {
    TablePosition::Top | TablePosition::Bottom => {
      let space_between = area.height - 3 - 1;
      let vertical_spacer_spans = (0..space_between).map(|_| Spans::from(Span::raw(""))).collect::<Vec<_>>();

      let mut spans = vec![
        Spans::from(money_on_table.unwrap_or(Span::raw(""))),
        Spans::from(status_spans),
        Spans::from(name),
        Spans::from(money_in_wallet),
      ];

      if seat.position == TablePosition::Top {
        spans.reverse();
        spans.splice(3..3, vertical_spacer_spans);
      } else {
        spans.splice(1..1, vertical_spacer_spans);
      }
      f.render_widget(Paragraph::new(spans).alignment(Alignment::Center),Rect { x: area.x, y: area.y, width: area.width, height: area.height });
    }
    TablePosition::Left | TablePosition::Right => {
      let y_offset = area.y + (area.height - 3) / 2;
      f.render_widget(
        Paragraph::new(vec![
          Spans::from(status_spans),
          Spans::from(name),
          Spans::from(money_in_wallet),
        ]).alignment(if seat.position == TablePosition::Left { Alignment::Left } else { Alignment::Right }),
        Rect { x: area.x, y: y_offset, width: area.width, height: 3 }
      );
      if money_on_table.is_some() {
        f.render_widget(
          Paragraph::new( money_on_table.unwrap())
            .alignment(if seat.position == TablePosition::Left { Alignment::Right } else { Alignment::Left }),
          Rect { x: area.x, y: y_offset + 1, width: area.width, height: 1 }
        );
      }
    }
  }
}


#[cfg(test)]
mod tests;


