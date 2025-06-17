use crate::handler::handle_event;
use crate::station::StationManager;
use crate::tui::{Event, Tui};
use anyhow::Result;
use crossbeam_channel::select;
use ratatui::widgets::ListState; //+ ADDED
use std::{collections::HashMap, io, thread, time::Duration};
use termion::input::TermRead;

pub enum AppMode { Normal, Discovery, Copy }
pub enum FocusedPanel { Stations, History }

pub struct App {
    pub should_quit: bool,
    pub mode: AppMode,
    pub focused_panel: FocusedPanel,
    pub station_manager: StationManager,
    pub history: HashMap<String, Vec<(String, String)>>,
    //+ CHANGED: We now use ListState to manage the view.
    pub history_list_state: ListState,
}

impl App {
    pub fn new() -> Result<Self> {
        let mut history = HashMap::new();
        history.insert(
            "ILove2Dance".to_string(),
            vec![
                ("14:30:15".to_string(), "Artist - Song Title".to_string()),
                ("14:33:42".to_string(), "Another Artist - Another Song".to_string()),
                ("14:37:01".to_string(), "Third Song - By A Band".to_string()),
                ("14:40:22".to_string(), "A Classic Hit - Old Timer".to_string()),
                ("14:44:56".to_string(), "Electronic Beat - Producer".to_string()),
                ("14:48:19".to_string(), "Something New - Fresh Artist".to_string()),
            ],
        );
        history.insert(
            "RM Deutschrap".to_string(),
            vec![
                ("15:01:00".to_string(), "Rapper - Track One".to_string()),
                ("15:04:12".to_string(), "Rapper Two - Feature Track".to_string()),
            ],
        );

        Ok(Self {
            should_quit: false,
            mode: AppMode::Normal,
            focused_panel: FocusedPanel::Stations,
            station_manager: StationManager::new()?,
            history,
            //+ ADDED: Initialize the ListState.
            history_list_state: ListState::default(),
        })
    }

    pub fn run(&mut self, tui: &mut Tui) -> Result<()> {
        let (event_sender, event_receiver) = crossbeam_channel::unbounded();
        let input_sender = event_sender.clone();
        thread::spawn(move || {
            let stdin = io::stdin();
            for event in stdin.events() {
                if let Ok(event) = event {
                    if input_sender.send(Event::Input(event)).is_err() {
                        return;
                    }
                }
            }
        });
        while !self.should_quit {
            tui.draw(self)?;
            select! {
                recv(event_receiver) -> event => {
                    if let Ok(event) = event {
                        handle_event(event, self)?;
                    }
                },
                default(Duration::from_millis(100)) => {}
            }
        }
        Ok(())
    }
}
