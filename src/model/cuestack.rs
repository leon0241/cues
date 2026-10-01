use ratatui::{
    widgets::{TableState}
};

use color_eyre::eyre::eyre;

use crate::model::{
    cue::{Cue, CueType, AudioFile},
    editor::{RunningState}
};

#[derive(Debug, Default)]
pub struct CueStack {
    // cue_count: i16,
    pub cues: Vec<Cue>,
    pub current_cue: TableState,
    pub running_state: RunningState,
}

impl CueStack {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_items(&mut self, items: Vec<Cue>) {
        self.cues = items;
        // We reset the state as the associated items have changed. This effectively reset
        // the selection as well as the stored offset.
        self.current_cue = TableState::default();
    }

    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn next(&mut self) {
        let i = match self.current_cue.selected() {
            Some(i) => {
                if i >= self.cues.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.current_cue.select(Some(i));
    }

    // Select the previous item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn previous(&mut self) {
        let i = match self.current_cue.selected() {
            Some(i) => {
                if i == 0 {
                    self.cues.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.current_cue.select(Some(i));
    }

    // Unselect the currently selected item if any. The implementation of `TableState` makes
    // sure that the stored offset is also reset.
    pub fn unselect(&mut self) {
        self.current_cue.select(None);
    }

    pub fn update_cue(&self){
        unimplemented!();
    }

    pub fn new_cue(&mut self, new_cue: Cue){
        self.cues.push(new_cue)
    }

    // Delete Current Cue
    pub fn delete_cue(&mut self) -> color_eyre::Result<()> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        self.cues.remove(i);
        Ok(())
    }

    pub fn get_current_type(&self) -> color_eyre::Result<CueType> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        let cue: &Cue = self.cues.get(i).ok_or_else( || eyre!("index out of range"))?;

        Ok(cue.get_type())
    }

    pub fn get_audio_file(&self) -> color_eyre::Result<Option<AudioFile>> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        let cue: &Cue = self.cues.get(i).ok_or_else( || eyre!("index out of range"))?;

        Ok(cue.get_audio_file())
    }
}
