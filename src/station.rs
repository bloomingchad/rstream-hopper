use crate::config::get_stations;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    pub id: usize,
    pub name: String,
    pub urls: Vec<String>,
    #[serde(skip)]
    pub current_title: String,
}

pub struct StationManager {
    pub stations: Vec<Station>,
    pub active_station_index: usize,
}

impl StationManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            stations: get_stations(),
            active_station_index: 0,
        })
    }

    //+ ADDED: Helper to get the currently active station.
    pub fn active_station(&self) -> Option<&Station> {
        self.stations.get(self.active_station_index)
    }

    pub fn next_station(&mut self) {
        let next_index = self.active_station_index + 1;
        self.active_station_index = if next_index >= self.stations.len() { 0 } else { next_index };
    }

    pub fn previous_station(&mut self) {
        self.active_station_index = if self.active_station_index == 0 { self.stations.len() - 1 } else { self.active_station_index - 1 };
    }
}
