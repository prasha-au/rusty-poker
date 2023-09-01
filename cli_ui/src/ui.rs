use std::{io, time::Duration};

use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusty_poker_core::{
  game::{Game, GameState},
  player::{CallingPlayer, Player},
};
use tui::{
  backend::{Backend, CrosstermBackend},
  layout::Rect,
  layout::{Constraint, Direction, Layout},
  Frame, Terminal,
};

mod actions;
mod render;

use actions::ActionsState;
use render::{draw_player_info, draw_table};

fn render<B: Backend>(f: &mut Frame<B>, game_state: &GameState, actions_state: &mut ActionsState) {
  let size = f.size();
  let table_area = Rect {
    x: 0,
    y: 0,
    width: size.width,
    height: size.height - 15,
  };

  draw_table(f, game_state, table_area);

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Length(80), Constraint::Min(0)].as_ref())
    .split(Rect {
      x: 0,
      y: size.height - 15,
      width: size.width,
      height: 15,
    });

  draw_player_info(f, game_state, chunks[1]);
  actions_state.render(f, chunks[1]);
}

struct DisableRawMode;
impl Drop for DisableRawMode {
  fn drop(&mut self) {
    disable_raw_mode().unwrap();
  }
}

pub fn run_tui() -> Result<(), io::Error> {
  let _disable_raw_mode = DisableRawMode;

  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut calling_players = vec![
    CallingPlayer { id: 1 },
    CallingPlayer { id: 2 },
    CallingPlayer { id: 3 },
    CallingPlayer { id: 4 },
    CallingPlayer { id: 5 },
    CallingPlayer { id: 6 },
    CallingPlayer { id: 7 },
    CallingPlayer { id: 8 },
  ];

  let players = calling_players
    .iter_mut()
    .map(|p| Box::new(p as &mut dyn Player))
    .collect::<Vec<Box<&mut dyn Player>>>();

  let mut game = Game::create(8, 1000);

  let mut actions_state = ActionsState::new();

  loop {
    let game_state = game.get_state(None);
    actions_state.update_game_state(&game_state);

    terminal.draw(|f| render(f, &game_state, &mut actions_state))?;

    if let Some(curr_index) = game.get_current_player_index() {
      match curr_index {
        0 => {
          if let Some(action) = actions_state.get_betting_action() {
            game.action_current_player(action).unwrap();
          }
        }
        _ => {
          let action = players[curr_index as usize].request_action(game.get_state(Some(curr_index)));
          game.action_current_player(action).unwrap();
        }
      }
    }
    let phase = game.next();
    if phase.is_none() {
      break;
    }

    if event::poll(Duration::from_millis(500))? {
      if let Event::Key(key) = event::read()? {
        match key.code {
          KeyCode::Char('q') => break,
          _ => actions_state.handle_keypress(key.code),
        }
      }
    }
  }

  execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
  terminal.show_cursor()?;

  Ok(())
}
