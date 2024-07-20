use crate::app::app::App;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use ratatui::crossterm::ExecutableCommand;
use std::io::stdout;

impl App<'_> {
    pub fn prepare_terminal(&mut self) -> &mut Self {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();

        self
    }
}
