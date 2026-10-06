use ratatui::widgets::{Cell, TableState};

use color_eyre::eyre::eyre;


use crate::model::{
    cues::{
        audio_cue::AudioCue, cue::{Cue, CueColumn, CueType}
    }, editor::RunningState
};

use crate::model::audio_config::AudioDevice;

use crate::update::update::Message;

#[derive(Default)]
pub struct CueStack {
    // cue_count: i16,
    pub cues: Vec<Box<dyn Cue>>,
    pub current_cue: TableState,
    pub running_state: RunningState,
    pub handler: AudioDevice
}

impl CueStack {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_items(&mut self, items: Vec<Box<dyn Cue>>) {
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

    pub fn new_cue(&mut self, new_cue: Box<dyn Cue>){
        self.cues.push(new_cue)
    }

    // Delete Current Cue
    pub fn delete_cue(&mut self) -> color_eyre::Result<()> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        self.cues.remove(i);
        Ok(())
    }

    pub fn get_current_type(&self) -> color_eyre::Result<CueType> {
        let i: usize = self.current_cue.selected()
            .ok_or_else(|| eyre!("no cue selected"))?;

        let current_cue: &Box<dyn Cue> = self.cues.get(i)
            .ok_or_else( || eyre!("index out of range"))?;

        Ok(current_cue.get_type())
    }

    fn cue_action(&mut self, i: usize, selection: Message) -> color_eyre::Result<()> {

        let current_cue: &mut Box<dyn Cue> = &mut self.cues[i];


        match selection {
            Message::PlayCue => {
                current_cue.play_cue()?;
                current_cue.edit_cell_value(CueColumn::Playing, String::from(""))
            }
            Message::PauseCue => {
                // If pause is implemented
                if let Some(_i) = current_cue.pause_cue() {
                    current_cue.edit_cell_value(CueColumn::Playing, String::from(""))
                }
                else {
                    current_cue.edit_cell_value(CueColumn::Playing, String::from(""))
                }
            }
            Message::StopCue => {
                // If Stop is implemented
                if let Some(_i) = current_cue.stop_cue() {
                    current_cue.edit_cell_value(CueColumn::Playing, String::from(""))
                }
                
            }
            Message::FadeStopCue => {
                // If F+S is implemented
                if let Some(_i) = current_cue.fade_stop_cue(3_f32) {
                    current_cue.edit_cell_value(CueColumn::Playing, String::from(""))
                }
            }
            _ => { }
        }

        Ok(())
    }

    pub fn current_cue_action(&mut self, selection: Message) -> color_eyre::Result<()> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        self.cue_action(i, selection)
    }

    pub fn get_current_cue_target(&self) -> color_eyre::Result<Option<f32>> {
        let i: usize = self.current_cue.selected().ok_or_else(|| eyre!("no cue selected"))?;

        Ok(self.cues[i].get_target())
    }

    pub fn choose_cue_action(&mut self, target: f32, selection: Message) -> color_eyre::Result<()> {
        // Get all matches for a cue target
        let mut matches = self.cues.iter()
            .enumerate()
            .filter(|x| x.1.get_number() == target)
            .map(|(i, _)| i);

        match (matches.next(), matches.next()) {
            (None, _) => {
                println!("1");
                // zero matches
                Ok(())
            }
            (Some(i), None) => {
                // println!("2");
                println!("{}", i);
                // exactly one match
                self.cue_action(i, selection)
            }
            (Some(first), Some(second)) => {
                println!("3");
                // two or more matches (first and second are the first two indices)
                Ok(())
            }
        };
        Ok(())
    }

}
