use color_eyre::eyre::eyre;

use crate::model::cues::Cue;
use crate::model::cues::CueColumn;
use crate::model::cues::cue_base::FollowState;
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

    pub const fn edit_current_cell_start(&self) {}

    pub const fn edit_current_cell_end(&self) {}

    pub const fn save_current_cell(&self) {}

    pub const fn copy_cue(&self) {}

    /// # Errors
    ///
    /// TODO
    pub fn copy_cell(&mut self, _column: &CueColumn) -> color_eyre::Result<()> {
        // Current Cue
        let i: usize = self
            .current_cue
            .selected()
            .ok_or_else(|| eyre!("no cue selected"))?;

        let _current_cue: &mut dyn Cue = self
            .cues
            .get_mut(i)
            .ok_or_else(|| eyre!("index out of range"))?
            .as_mut();
        Ok(())
    }

    /// # Errors
    ///
    /// TODO
    pub fn change_cell(&mut self, column: &CueColumn) -> color_eyre::Result<()> {
        // Current Cue
        let i: usize = self
            .current_cue
            .selected()
            .ok_or_else(|| eyre!("no cue selected"))?;

        let current_cue: &mut dyn Cue = self
            .cues
            .get_mut(i)
            .ok_or_else(|| eyre!("index out of range"))?
            .as_mut();

        match column {
            CueColumn::Playing | CueColumn::Duration => {}
            CueColumn::Name => current_cue.set_base().name = String::from("test"),
            CueColumn::Number => current_cue.set_base().number = 6.7,
            CueColumn::PreWait => current_cue.set_base().prewait = 6.7,
            CueColumn::PostWait => current_cue.set_base().postwait = 6.7,
            CueColumn::Follow => current_cue.set_base().follow = FollowState::Continue,
        }
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
    pub fn delete_cell(&mut self, column: &CueColumn) -> color_eyre::Result<()> {
        // Current Cue
        let i: usize = self
            .current_cue
            .selected()
            .ok_or_else(|| eyre!("no cue selected"))?;

        let current_cue: &mut dyn Cue = self
            .cues
            .get_mut(i)
            .ok_or_else(|| eyre!("index out of range"))?
            .as_mut();

        match column {
            CueColumn::Playing | CueColumn::Duration => {}
            CueColumn::Name => current_cue.set_base().name = String::new(),
            CueColumn::Number => current_cue.set_base().number = 0.0,
            CueColumn::PreWait => current_cue.set_base().prewait = 0.0,
            CueColumn::PostWait => current_cue.set_base().postwait = 0.0,
            CueColumn::Follow => current_cue.set_base().follow = FollowState::None,
        }
        Ok(())
    }
}
