use crate::app::App;
use crate::ui;
use anyhow::Result;
use ratatui::{backend::TermionBackend, Terminal};
use std::io::{self, Stdout};
use termion::{
    event::Event as TermionEvent,
    raw::{IntoRawMode, RawTerminal},
    screen::IntoAlternateScreen,
};

pub enum Event { Input(TermionEvent) }

pub struct Tui {
    terminal: Terminal<TermionBackend<termion::screen::AlternateScreen<RawTerminal<Stdout>>>>,
}

impl Tui {
    pub fn new() -> Result<Self> {
        let stdout = io::stdout().into_raw_mode()?.into_alternate_screen()?;
        let backend = TermionBackend::new(stdout);
        Ok(Self { terminal: Terminal::new(backend)? })
    }
    pub fn enter(&mut self) -> Result<()> {
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }
    pub fn exit(&mut self) -> Result<()> {
        self.terminal.show_cursor()?;
        Ok(())
    }
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(frame, app))?;
        Ok(())
    }
}
