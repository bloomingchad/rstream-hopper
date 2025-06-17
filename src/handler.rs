use crate::app::{App, FocusedPanel};
use crate::tui::Event;
use anyhow::Result;
use termion::event::{Event as TermionEvent, Key};

pub fn handle_event(event: Event, app: &mut App) -> Result<()> {
    if let Event::Input(TermionEvent::Key(key)) = event {
        handle_key_press(key, app)?;
    }
    Ok(())
}

fn handle_key_press(key: Key, app: &mut App) -> Result<()> {
    match key {
        Key::Char('q') => {
            app.should_quit = true;
            return Ok(());
        }
        Key::Char('\t') => {
            app.focused_panel = match app.focused_panel {
                FocusedPanel::Stations => FocusedPanel::History,
                FocusedPanel::History => FocusedPanel::Stations,
            };
            app.history_list_state.select(None); // Deselect history when switching away
            return Ok(());
        }
        _ => {}
    }

    match app.focused_panel {
        FocusedPanel::Stations => handle_stations_panel_keys(key, app),
        FocusedPanel::History => handle_history_panel_keys(key, app),
    }

    Ok(())
}

fn handle_stations_panel_keys(key: Key, app: &mut App) {
    match key {
        Key::Up | Key::Char('k') => {
            app.station_manager.previous_station();
            app.history_list_state.select(None); // Reset history selection
        }
        Key::Down | Key::Char('j') => {
            app.station_manager.next_station();
            app.history_list_state.select(None); // Reset history selection
        }
        _ => {}
    }
}

//+ CHANGED: This now manipulates the ListState for scrolling.
fn handle_history_panel_keys(key: Key, app: &mut App) {
    let history_len = match app.station_manager.active_station() {
        Some(station) => app.history.get(&station.name).map_or(0, |h| h.len()),
        None => 0,
    };

    if history_len == 0 {
        return; // Nothing to scroll
    }

    let current_selection = app.history_list_state.selected();

    match key {
        Key::Up | Key::Char('k') => {
            let next = current_selection.map_or(0, |i| i.saturating_sub(1));
            app.history_list_state.select(Some(next));
        }
        Key::Down | Key::Char('j') => {
            let next = current_selection.map_or(0, |i| i.saturating_add(1).min(history_len - 1));
            app.history_list_state.select(Some(next));
        }
        _ => {}
    }
}
