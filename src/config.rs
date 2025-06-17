use crate::station::Station;

const STATIONS_DATA: &[(&str, &[&str])] = &[
    ("ILove2Dance", &["https://ilm.stream18.radiohost.de/ilm_ilove2dance_mp3-192", "https://streams.ilovemusic.de/iloveradio2-aac.mp3"]),
    ("RM Deutschrap", &["https://rautemusik.stream43.radiohost.de/rm-deutschrap-charts_mp3-192"]),
    ("bigFM Dance", &["https://streams.bigfm.de/bigfm-dance-128-mp3"]),
    // Add other stations here...
];

pub fn get_stations() -> Vec<Station> {
    STATIONS_DATA
        .iter()
        .enumerate()
        .map(|(id, (name, urls))| Station {
            id,
            name: name.to_string(),
            urls: urls.iter().map(|s| s.to_string()).collect(),
            current_title: "Connecting...".to_string(),
        })
        .collect()
}
