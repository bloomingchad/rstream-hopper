use anyhow::Result;
use flexi_logger::{FileSpec, Logger};
use rstream_hopper::app::App;
use rstream_hopper::tui::Tui;

fn main() -> Result<()> {
    Logger::try_with_str("info")?
        .log_to_file(FileSpec::default())
        .start()?;
    log::info!("Application starting...");

    let mut app = App::new()?;
    let mut tui = Tui::new()?;
    tui.enter()?;
    let result = app.run(&mut tui);
    tui.exit()?;

    log::info!("Application shutting down.");
    result
}
