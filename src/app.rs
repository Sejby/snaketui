use crate::entities::SnakeDirection;
use crate::game::{GameState, POLL_RATE};
use crate::ui::draw_game;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

pub struct App {
    game_state: GameState,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.exit {
            self.handle_events()?;

            if self.game_state.should_tick() {
                self.game_state.tick();
            }

            terminal.draw(|frame| draw_game(frame, &self.game_state))?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(POLL_RATE)? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    self.handle_key_event(key_event);
                }
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('r') if self.game_state.is_game_over() => self.game_state.reset(),
            KeyCode::Left => self.game_state.set_direction(SnakeDirection::Left),
            KeyCode::Right => self.game_state.set_direction(SnakeDirection::Right),
            KeyCode::Up => self.game_state.set_direction(SnakeDirection::Up),
            KeyCode::Down => self.game_state.set_direction(SnakeDirection::Down),
            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
