use crate::handler::handle_event;
use crate::station::StationManager;
use crate::tui::{Event, Tui};
use anyhow::Result;
use crossbeam_channel::select;
use std::{io, thread, time::Duration};
use termion::input::TermRead;

pub enum AppMode { Normal, Discovery, Copy }
pub enum FocusedPanel { Stations, History }

pub struct App {
    pub should_quit: bool,
    pub mode: AppMode,
    pub focused_panel: FocusedPanel,
    pub station_manager: StationManager,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            mode: AppMode::Normal,
            focused_panel: FocusedPanel::Stations,
            station_manager: StationManager::new()?,
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
