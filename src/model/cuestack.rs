pub mod modifiers;
pub mod navigation;
pub mod playback;

use ratatui::widgets::TableState;

use color_eyre::eyre::eyre;

use crate::model::{
    cue::{Cue, CueColumn, CueType},
    keymaps::PlaybackCmd,
};

use crate::model::audio_config::AudioDevice;

#[derive(Default)]
pub struct CueStack {
    pub cues: Vec<Cue>,
    pub current_state: TableState,
    pub handler: AudioDevice,
    pub current_cell_ref: (usize, CueColumn), // Test message output
}

impl CueStack {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// # Errors
    ///
    /// Will return `Err` if cannot find a default sink
    pub fn set_default_sink(&mut self) -> color_eyre::Result<()> {
        self.handler.set_default_sink()
    }

    pub fn set_items(&mut self, items: Vec<Cue>) {
        self.cues = items;
        // We reset the state as the associated items have changed. This effectively reset
        // the selection as well as the stored offset.
        self.current_state = TableState::default();
    }

    /// Get the current Cue Type
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    #[must_use]
    pub fn current_type(&self) -> Option<&CueType> {
        let current_cue: &Cue = self.get_current_cue()?.0;

        Some(current_cue.cue_type())
    }

    /// Get the current cue target
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    #[must_use]
    pub fn current_target(&self) -> Option<String> {
        let current_cue: &Cue = self.get_current_cue()?.0;

        Some(current_cue.get_target())
    }

    /// Get the action of a chosen target cue
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected or too
    /// many targets
    pub fn cue_action(
        &mut self,
        target: Option<String>,
        selection: &PlaybackCmd,
    ) -> color_eyre::Result<()> {
        let cue_num: usize = match target {
            None => self
                .current_state
                .selected()
                .ok_or_else(|| eyre!("no cue selected"))?,

            Some(t) => self.query_target(&t),
        };
        self.playback_action(cue_num, *selection)?;
        Ok(())
    }

    fn query_target(&self, target: &str) -> usize {
        // Get all matches for a cue target
        let mut matches = self
            .cues
            .iter()
            .enumerate()
            .filter(|x| x.1.number() == target)
            .map(|(i, _)| i);

        // if there are matches or net
        match (matches.next(), matches.next()) {
            (None, _) => {
                // zero matches
                0
            }
            (Some(i), None) => {
                // exactly one match (ideal)
                i
            }
            (Some(_f), Some(_s)) => {
                // two or more matches (first and second are the first two indices)
                0
            }
        }
    }

    #[must_use]
    pub const fn current_cell_ref(&self) -> (usize, CueColumn) {
        self.current_cell_ref
    }

    pub const fn set_current_cell_ref(&mut self, row: usize, column: CueColumn) {
        self.current_cell_ref = (row, column);
    }
}
