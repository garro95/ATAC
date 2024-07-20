use crate::app::app_states::{next_app_state, previous_app_state, AppState};

#[derive(Default)]
pub struct HelpPopup {
    pub selection: AppState,
}

impl HelpPopup {
    pub fn next_page(&mut self) {
        self.selection = next_app_state(&self.selection);
    }

    pub fn previous_page(&mut self) {
        self.selection = previous_app_state(&self.selection);
    }
}
