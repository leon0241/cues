pub mod modifiers;
pub mod navigation;
pub mod playback;

use ratatui::widgets::TableState;

use color_eyre::eyre::eyre;

use crate::model::{
    cues::{Cue, cue_base::CueType},
    keymaps::PlaybackCmd,
};

use crate::model::audio_config::AudioDevice;

#[derive(Default)]
pub struct CueStack {
    pub cues: Vec<Box<dyn Cue>>,
    pub current_cue: TableState,
    pub handler: AudioDevice,
    pub do_a_thing: String, // Test message output
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

    pub fn set_items(&mut self, items: Vec<Box<dyn Cue>>) {
        self.cues = items;
        // We reset the state as the associated items have changed. This effectively reset
        // the selection as well as the stored offset.
        self.current_cue = TableState::default();
    }

    /// Get the current Cue Type
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    pub fn current_type(&self) -> color_eyre::Result<CueType> {
        let i: usize = self
            .current_cue
            .selected()
            .ok_or_else(|| eyre!("no cue selected"))?;

        let current_cue: &dyn Cue = self
            .cues
            .get(i)
            .ok_or_else(|| eyre!("index out of range"))?
            .as_ref();

        Ok(current_cue.cue_type())
    }

    /// Get the current cue target
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    pub fn current_target(&self) -> color_eyre::Result<Option<f32>> {
        let i: usize = self
            .current_cue
            .selected()
            .ok_or_else(|| eyre!("no cue selected"))?;

        let current_cue: &dyn Cue = self
            .cues
            .get(i)
            .ok_or_else(|| eyre!("index out of range"))?
            .as_ref();

        Ok(current_cue.target())
    }

    /// Get the action of a chosen target cue
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected or too
    /// many targets
    pub fn cue_action(
        &mut self,
        target: Option<f32>,
        selection: &PlaybackCmd,
    ) -> color_eyre::Result<()> {
        let cue_num: usize = match target {
            None => self
                .current_cue
                .selected()
                .ok_or_else(|| eyre!("no cue selected"))?,
            Some(t) => self.query_target(t),
        };
        self.playback_action(cue_num, *selection)?;
        Ok(())
    }

    fn query_target(&self, target: f32) -> usize {
        // Get all matches for a cue target
        let mut matches = self
            .cues
            .iter()
            .enumerate()
            .filter(|x| (x.1.number() - target).abs() < 0.001)
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
}
