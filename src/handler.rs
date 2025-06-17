// src/handler.rs
use crate::app::{App, AppMode};
use crate::tui::Event;
use anyhow::Result;
//+ ADDED: termion's Key enum
use termion::event::{Event as TermionEvent, Key};

/// Handles events from the TUI and updates the application state.
pub fn handle_event(event: Event, app: &mut App) -> Result<()> {
    //+ CHANGED: Match on a termion Event, not crossterm
    if let Event::Input(TermionEvent::Key(key)) = event {
        handle_key_press(key, app)?;
    }
    Ok(())
}

fn handle_key_press(key: Key, app: &mut App) -> Result<()> {
    //+ CHANGED: The key codes are now from termion::event::Key
    if key == Key::Char('q') {
        app.should_quit = true;
        return Ok(());
    }

    match app.mode {
        AppMode::Normal => match key {
            Key::Up | Key::Char('k') => app.station_manager.previous_station(),
            Key::Down | Key::Char('j') => app.station_manager.next_station(),
            Key::Char('\n') => { /* Enter key for mute */ }
            Key::Char('s') => { /* Enter discovery mode */ }
            _ => {}
        },
        AppMode::Discovery => { /* Handle discovery mode keys */ }
        AppMode::Copy => { /* Handle copy mode keys */ }
    }
    Ok(())
}
