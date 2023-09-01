use crossterm::event::KeyCode;
use rusty_poker_core::game::{BettingAction, GameState};
use tui::{
  backend::Backend,
  layout::Rect,
  style::{Color, Modifier, Style},
  text::Span,
  widgets::{List, ListItem, ListState},
  Frame,
};

pub struct ActionsState {
  money_in_wallet: u32,
  action_selection: ListState,
  raise_amount: u32,
  action_locked: bool,
}

impl ActionsState {
  pub fn new() -> ActionsState {
    let mut s = ActionsState {
      action_selection: ListState::default(),
      money_in_wallet: 0,
      raise_amount: 0,
      action_locked: false,
    };
    s.action_selection.select(Some(0));
    s
  }

  fn prev_action(&mut self) {
    let selected = self.action_selection.selected().unwrap_or(1);
    if selected > 0 {
      self.action_selection.select(Some(selected - 1));
    }
  }

  fn next_action(&mut self) {
    let selected = self.action_selection.selected().unwrap_or(0);
    if selected < 3 {
      self.action_selection.select(Some(selected + 1));
    }
  }

  pub fn handle_keypress(&mut self, code: KeyCode) {
    if self.action_locked {
      if code == KeyCode::Esc {
        self.action_locked = false;
      }
      return;
    }

    match code {
      KeyCode::Up => self.prev_action(),
      KeyCode::Down => self.next_action(),
      KeyCode::Left => self.raise_amount -= 10,
      KeyCode::Right => self.raise_amount += 10,
      KeyCode::Enter => self.action_locked = true,
      _ => {}
    }
  }

  pub fn update_game_state(&mut self, game_state: &GameState) {
    self.money_in_wallet = game_state.wallet;
    if self.raise_amount < game_state.value_to_call {
      self.raise_amount = game_state.value_to_call + 10;
    }
    if self.raise_amount > self.money_in_wallet {
      self.raise_amount = self.money_in_wallet;
    }
  }

  pub fn get_betting_action(&mut self) -> Option<BettingAction> {
    if !self.action_locked {
      return None;
    }
    let selected = self.action_selection.selected().unwrap_or(0);
    let action = match selected {
      0 => BettingAction::Fold,
      1 => BettingAction::Call,
      2 => BettingAction::Raise(self.raise_amount),
      3 => BettingAction::AllIn,
      _ => BettingAction::Call,
    };

    self.action_locked = false;
    self.raise_amount = 0;
    self.action_selection.select(Some(0));

    Some(action)
  }

  pub fn render<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
    let mut actions_rect = area.clone();
    actions_rect.y += 5;
    actions_rect.height -= 5;

    let items: Vec<ListItem> = vec![
      ListItem::new(Span::from("Fold")).style(Style::default()),
      ListItem::new(Span::from("Check/Call")).style(Style::default()),
      ListItem::new(Span::from(format!("Raise ${}", self.raise_amount))).style(Style::default()),
      ListItem::new(Span::from(format!("All In ${}", self.money_in_wallet))).style(Style::default()),
    ];

    let items = List::new(items)
      .highlight_style(
        Style::default()
          .bg(if self.action_locked { Color::Green } else { Color::White })
          .fg(Color::Black)
          .add_modifier(Modifier::BOLD),
      )
      .highlight_symbol(">> ");
    f.render_stateful_widget(items, actions_rect, &mut self.action_selection);
  }
}
