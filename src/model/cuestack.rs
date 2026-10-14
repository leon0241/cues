use ratatui::widgets::TableState;

use color_eyre::eyre::eyre;


use crate::{model::
    cues::{
        cue::{Cue, CueColumn}, cue_base::CueType,
    }
, update::keymaps::PlaybackCmd};

use crate::model::audio_config::AudioDevice;


#[derive(Default)]
pub struct CueStack {
    pub cues: Vec<Box<dyn Cue>>,
    pub current_cue: TableState,
    pub handler: AudioDevice,
    pub do_a_thing: String // Test message output
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


    /// # Errors
    ///
    /// Will return `Err` if it can't i guess
    pub fn go_left(&mut self) -> color_eyre::Result<()>  {
        self.current_cue.select_previous_column();
        Ok(())
    }

    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    /// # Errors
    ///
    /// Will return `Err` if can't go next or prev
    pub fn go_next(&mut self) -> color_eyre::Result<()> {
        self.current_cue.select_next();
        Ok(())
    }

    /// Select the previous item. This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    ///
    /// # Errors
    ///
    /// Will return `Err` if overflows
    pub fn go_previous(&mut self) -> color_eyre::Result<()>  {
        self.current_cue.select_previous();
        Ok(())
    }


    /// # Errors
    ///
    /// Will return `Err` if it can't i guess
    pub fn go_right(&mut self) -> color_eyre::Result<()>  {
        self.current_cue.select_next_column();
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

    fn cue_action(&mut self, i: usize, selection: &PlaybackCmd) -> color_eyre::Result<()> {

        let current_cue: &mut Box<dyn Cue> = 
            self.cues
            .get_mut(i)
            .ok_or_else( || eyre!("index out of range"))?;


        match selection {
            PlaybackCmd::PlayCue => {
                current_cue.play_cue()?;
                current_cue.edit_cell_value(CueColumn::Playing, String::from(""));
            }
            PlaybackCmd::PauseCue => {
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
            PlaybackCmd::StopCue => {
                // If Stop is implemented
                if let Some(_i) = current_cue.stop_cue() {
                    current_cue.edit_cell_value(CueColumn::Playing, String::from(""));
                }
                
            }
            PlaybackCmd::FadeStopCue => {
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
    pub fn current_cue_action(&mut self, selection: &PlaybackCmd) -> color_eyre::Result<()> {
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
    pub fn choose_cue_action(&mut self, target: f32, selection: &PlaybackCmd) -> color_eyre::Result<()> {
        // Get all matches for a cue target
        let mut matches = self.cues.iter()
            .enumerate()
            .filter(|x| (x.1.get_number() - target).abs() < 0.001)
            .map(|(i, _)| i);

        match (matches.next(), matches.next()) {
            (None, _) => {
                // zero matches
                println!("a");
           }
            (Some(i), None) => {
                // exactly one match
                self.cue_action(i, selection)?;
                println!("b");
            }
            (Some(_first), Some(_second)) => {
                // two or more matches (first and second are the first two indices)
                println!("c");
            }
        }
        Ok(())
    }

    pub fn edit_current_cell_start(&self) {
        println!("enter insert at start");
    }

    pub fn edit_current_cell_end(&self) {
        println!("enter insert at end");
    }

    pub fn save_current_cell(&self) {
        println!("saving a cell returning to normal");
    }

    pub fn copy_cue(&self) {
        println!("copying cue")
    }

    pub fn copy_cell(&self, column: CueColumn) {
        match column {
            CueColumn::Name => {
                println!("copying name")
            },
            CueColumn::Number => {
                println!("copying number")
            },
            CueColumn::Follow => {
                println!("copying follow")
            },
            CueColumn::PreWait => {
                println!("copying duration")
            },
            CueColumn::PostWait => {
                println!("copying duration")
            },
            _ => {
                println!("do nothing")
            }
        }
    }
}
