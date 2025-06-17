use crate::app::{App, FocusedPanel};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph}, //+ ADDED ListState
    Frame,
};

pub fn render(frame: &mut Frame, app: &mut App) {
    let size = frame.size();
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0), Constraint::Length(1)])
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
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(body_layout[1]);
    render_now_playing_panel(frame, right_panel_layout[0], app);
    render_history_panel(frame, right_panel_layout[1], app);
}

fn render_stations_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    let is_focused = matches!(app.focused_panel, FocusedPanel::Stations);
    let border_style = if is_focused { Style::default().fg(Color::Cyan) } else { Style::default() };
    let items: Vec<ListItem> = app.station_manager.stations.iter().map(|s| ListItem::new(s.name.clone())).collect();
    let list = List::new(items)
        .block(Block::default().title(" Stations ").borders(Borders::ALL).border_style(border_style))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::DarkGray))
        .highlight_symbol("▶ ");
    let mut list_state = ListState::default().with_selected(Some(app.station_manager.active_station_index));
    frame.render_stateful_widget(list, area, &mut list_state);
}

fn render_now_playing_panel(frame: &mut Frame, area: Rect, app: &App) {
    let block = Block::default().title(" Now Playing ").borders(Borders::ALL);
    let content = if let Some(active_station) = app.station_manager.active_station() {
        Text::from(vec![
            Line::from(""),
            Line::from(Span::styled(
                active_station.current_title.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                active_station.name.clone(),
                Style::default().fg(Color::Gray),
            )),
        ])
    } else {
        Text::from("No station selected.")
    };
    let paragraph = Paragraph::new(content).block(block).style(Style::default().fg(Color::White));
    frame.render_widget(paragraph, area);
}

//+ CHANGED: This function now uses render_stateful_widget and the app's history_list_state.
fn render_history_panel(frame: &mut Frame, area: Rect, app: &mut App) {
    let is_focused = matches!(app.focused_panel, FocusedPanel::History);
    let border_style = if is_focused { Style::default().fg(Color::Cyan) } else { Style::default() };
    let block = Block::default().title(" Recent History ").borders(Borders::ALL).border_style(border_style);

    let history_items: Vec<ListItem> = if let Some(station) = app.station_manager.active_station() {
        app.history
            .get(&station.name)
            .map_or(vec![ListItem::new("No history for this station.")], |entries| {
                entries
                    .iter()
                    .rev()
                    .map(|(timestamp, title)| {
                        let line = Line::from(vec![
                            Span::styled(format!("{:<10}", timestamp), Style::default().fg(Color::Yellow)),
                            Span::raw(title.clone()),
                        ]);
                        ListItem::new(line)
                    })
                    .collect()
            })
    } else {
        vec![]
    };

    let list = List::new(history_items)
        .block(block)
        .highlight_style(Style::default().bg(Color::DarkGray)); // Add a highlight for selection

    // Use the mutable state from the app to render the list.
    // Ratatui will handle the view window based on the selected item.
    frame.render_stateful_widget(list, area, &mut app.history_list_state);
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let footer_text = "[Q] Quit | [↑↓] Navigate | [Tab] Switch Panel";
    let footer = Paragraph::new(footer_text).style(Style::default().bg(Color::Blue));
    frame.render_widget(footer, area);
}
