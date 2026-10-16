use color_eyre::eyre::eyre;

use crate::model::cues::Cue;
use crate::model::cues::CueColumn;
use crate::model::cuestack::CueStack;

impl CueStack {
    pub fn new_cue(&mut self, new_cue: Box<dyn Cue>) {
        self.cues.push(new_cue);
    }

    /// Delete Current Cue
    ///
    /// # Errors
    ///
    /// Will return `Err` if no cue is selected
    pub fn delete_cue(&mut self) -> color_eyre::Result<()> {
        let i: usize = self
            .current_cue
            .selected()
            .ok_or_else(|| eyre!("no cue selected"))?;

        self.cues.remove(i);
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
        println!("copying cue");
    }

    pub fn copy_cell(&self, column: &CueColumn) {
        match column {
            CueColumn::Name => {
                println!("copying name");
            }
            CueColumn::Number => {
                println!("copying number");
            }
            CueColumn::Follow => {
                println!("copying follow");
            }
            CueColumn::PreWait => {
                println!("copying prewait");
            }
            CueColumn::PostWait => {
                println!("copying postwait");
            }
            _ => {
                println!("do nothing");
            }
        }
    }

    /// # Errors
    ///
    /// TODO
    pub const fn change_cell(&self, _i: &CueColumn) -> color_eyre::Result<()> {
        Ok(())
    }

    /// # Errors
    ///
    /// TODO
    pub const fn clear_cue(&self) -> color_eyre::Result<()> {
        Ok(())
    }

    /// # Errors
    ///
    /// TODO
    pub const fn delete_cell(&self, _i: &CueColumn) -> color_eyre::Result<()> {
        Ok(())
    }
}
