use ratatui::widgets::TableState;

use color_eyre::eyre::eyre;


use crate::model::{
    cues::{
        cue::{Cue, CueColumn}, cue_base::CueType,
    }, editor::RunningState
};

use crate::model::audio_config::AudioDevice;

use crate::update::update::Message;

#[derive(Default)]
pub struct CueStack {
    pub cues: Vec<Box<dyn Cue>>,
    pub current_cue: TableState,
    pub running_state: RunningState,
    pub handler: AudioDevice
}

impl CueStack {

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_default_sink(&mut self) -> color_eyre::Result<()> {
        self.handler.set_default_sink()
    }

    pub fn set_items(&mut self, items: Vec<Box<dyn Cue>>) {
        self.cues = items;
        // We reset the state as the associated items have changed. This effectively reset
        // the selection as well as the stored offset.
        self.current_cue = TableState::default();
    }

    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    /// # Errors
    ///
    /// Will return `Err` if can't go next or prev
    pub fn go_next(&mut self) -> color_eyre::Result<()> {
        let len = self.cues.len()
            .checked_sub(1)
            .ok_or_else(|| eyre!("Integer Overflow"))?;

        let i = match self.current_cue.selected() {
            Some(i) => {
                if i >= len {
                    0
                } else {
                    i.checked_add(1)
                        .ok_or_else(|| eyre!("Integer Overflow"))?
                }
            }
            None => 0,
        };
        self.current_cue.select(Some(i));
        Ok(())
    }

    /// Select the previous item. This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    ///
    /// # Errors
    ///
    /// Will return `Err` if overflows
    pub fn go_previous(&mut self) -> color_eyre::Result<()>  {
        let i = match self.current_cue.selected() {
            Some(i) => {
                if i == 0 {
                    self.cues.len()
                        .checked_sub(1)
                        .ok_or_else(|| eyre!("Integer Overflow"))?
                } else {
                    i
                        .checked_sub(1)
                        .ok_or_else(|| eyre!("Integer Overflow"))?
                }
            }
            None => 0,
        };
        self.current_cue.select(Some(i));
        Ok(())
    }

    // Unselect the currently selected item if any. The implementation of `TableState` makes
    // sure that the stored offset is also reset.
    pub const fn unselect(&mut self) {
        self.current_cue.select(None);
    }
    //
    // pub fn update_cue(&self){
    //     unimplemented!();
    // }

    pub fn new_cue(&mut self, new_cue: Box<dyn Cue>){
        self.cues.push(new_cue);
    }

    /// Delete Current Cue
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    pub fn delete_cue(&mut self) -> color_eyre::Result<()> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        self.cues.remove(i);
        Ok(())
    }

    /// Get the current Cue Type
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    pub fn get_current_type(&self) -> color_eyre::Result<CueType> {
        let i: usize = self.current_cue.selected()
            .ok_or_else(|| eyre!("no cue selected"))?;

        let current_cue: &dyn Cue =
            self.cues.get(i)
            .ok_or_else( || eyre!("index out of range"))?
            .as_ref();

        Ok(current_cue.get_type())
    }

    fn cue_action(&mut self, i: usize, selection: &Message) -> color_eyre::Result<()> {

        let current_cue: &mut Box<dyn Cue> = 
            self.cues
            .get_mut(i)
            .ok_or_else( || eyre!("index out of range"))?;


        match selection {
            Message::PlayCue => {
                current_cue.play_cue()?;
                current_cue.edit_cell_value(CueColumn::Playing, String::from(""));
            }
            Message::PauseCue => {
                // If pause is implemented
                match current_cue.pause_cue() {
                    // Unpaused -> Paused
                    Some(true) => {
                    current_cue.edit_cell_value(CueColumn::Playing, String::from(""));
                    }
                    // Paused -> Unpaused
                    Some(false) => {
                        current_cue.edit_cell_value(CueColumn::Playing, String::from(""));
                    }
                    // Else (no file / not playing)
                    _ => {}
                }
            }
            Message::StopCue => {
                // If Stop is implemented
                if let Some(_i) = current_cue.stop_cue() {
                    current_cue.edit_cell_value(CueColumn::Playing, String::from(""));
                }
                
            }
            Message::FadeStopCue => {
                // If F+S is implemented
                if let Some(_i) = current_cue.fade_stop_cue(3_f32) {
                    current_cue.edit_cell_value(CueColumn::Playing, String::from(""));
                }
            }
            _ => { }
        }

        Ok(())
    }

    /// Do something with the current cue
    /// 
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    pub fn current_cue_action(&mut self, selection: &Message) -> color_eyre::Result<()> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        self.cue_action(i, selection)
    }

    /// Get the current cue target
    /// 
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    pub fn get_current_cue_target(&self) -> color_eyre::Result<Option<f32>> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        let current_cue: &dyn Cue =
            self.cues.get(i)
            .ok_or_else( || eyre!("index out of range"))?
            .as_ref();

        Ok(current_cue.get_target())
    }

    /// Get the action of a chosen target cue
    /// 
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    pub fn choose_cue_action(&mut self, target: f32, selection: &Message) -> color_eyre::Result<()> {
        // Get all matches for a cue target
        let mut matches = self.cues.iter()
            .enumerate()
            .filter(|x| x.1.get_number() - target < 0.001)
            .map(|(i, _)| i);

        let _ = match (matches.next(), matches.next()) {
            (None, _) => {
                // zero matches
                Ok(())
            }
            (Some(i), None) => {
                // exactly one match
                self.cue_action(i, selection)
            }
            (Some(_first), Some(_second)) => {
                // two or more matches (first and second are the first two indices)
                Ok(())
            }
        };
        Ok(())
    }

}
