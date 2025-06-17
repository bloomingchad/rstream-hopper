// src/ui.rs
use crate::app::{App, FocusedPanel};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Renders the entire user interface.
pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.size();

    // Main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Body
            Constraint::Length(1), // Footer
        ])
        .split(size);

    render_header(frame, main_layout[0]);
    render_body(frame, main_layout[1], app);
    render_footer(frame, main_layout[2]);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new(" rstream-hopper | LIVE ").style(Style::default().bg(Color::Blue));
    frame.render_widget(header, area);
}

fn render_body(frame: &mut Frame, area: Rect, app: &mut App) {
    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    render_stations_panel(frame, body_layout[0], app);

    let right_panel_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(body_layout[1]);

    render_now_playing_panel(frame, right_panel_layout[0], app);
    render_history_panel(frame, right_panel_layout[1], app);
}

fn render_stations_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    let is_focused = matches!(app.focused_panel, FocusedPanel::Stations);
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };

    let items: Vec<ListItem> = app
        .station_manager
        .stations
        .iter()
        .map(|s| ListItem::new(s.name.clone()))
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Stations ")
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::DarkGray),
        )
        .highlight_symbol("▶ ");

    // We need a mutable ListState to keep track of the selection
    let mut list_state = ratatui::widgets::ListState::default()
        .with_selected(Some(app.station_manager.active_station_index));

    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_now_playing_panel(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default().title(" Now Playing ").borders(Borders::ALL);
    frame.render_widget(block, area);
}

fn render_history_panel(frame: &mut Frame, area: Rect, app: &App) {
    let is_focused = matches!(app.focused_panel, FocusedPanel::History);
    let border_style = if is_focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    };
    let block = Block::default()
        .title(" Recent History ")
        .borders(Borders::ALL)
        .border_style(border_style);
    frame.render_widget(block, area);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("[Q] Quit | [↑↓] Navigate").style(Style::default().bg(Color::Blue));
    frame.render_widget(footer, area);
}
