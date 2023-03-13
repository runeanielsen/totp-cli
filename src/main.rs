#![warn(clippy::all, clippy::pedantic)]

use std::{
    env,
    error::Error,
    io::stdout,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use arboard::Clipboard;
use totp::Totp;
use tui::{TotpLineParagraph, TotpListView};

mod config;
mod totp;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    const INTERVAL: u64 = 30;
    const DIGITS: u32 = 6;
    const POLL_INTERVAL: u64 = 1000;

    // Getting the home directory works fine on Unix systems,
    // and this project only supports UNIX based systems.
    #[allow(deprecated)]
    let home_dir = env::home_dir().expect("Could not load user home directory.");

    let clipboard = Arc::new(Mutex::new(
        Clipboard::new().expect("Could not get access to the clipboard."),
    ));

    let default_config_path = [home_dir, PathBuf::from(".config/totp-tui")]
        .iter()
        .collect();

    let totps: Vec<Totp> = config::load_totps(default_config_path, DIGITS, INTERVAL)?;

    let mut stdout = stdout();

    tui::start(
        &mut stdout,
        vec![
            Box::new(TotpLineParagraph::new()),
            Box::new(TotpListView::new(
                SystemTime::now(),
                INTERVAL,
                totps,
                Arc::clone(&clipboard),
            )),
        ],
        POLL_INTERVAL,
    )?;

    Ok(())
}
