use crate::model::cues::Cue;
use crate::model::cues::CueColumn;
use crate::model::cues::cue_base::FollowState;
use crate::model::cuestack::CueStack;

impl CueStack {
    pub fn new_cue(&mut self, new_cue: Box<dyn Cue>) {
        self.cues.push(new_cue);
    }

    const fn get_row_idx(&self) -> Option<usize> {
        self.current_state.selected()
    }

    const fn get_column_idx(&self) -> Option<usize> {
        self.current_state.selected_column()
    }

    fn get_current_cue(&self) -> Option<&dyn Cue> {
        let i = self.get_row_idx()?;

        // idk why clippy wants this
        self.cues.get(i).map(std::convert::AsRef::as_ref)
    }

    fn get_current_cue_mut(&mut self) -> Option<&mut dyn Cue> {
        let i = self.get_row_idx()?;

        let c = self.cues.get_mut(i)?;
        Some(c.as_mut())
    }

    /// Delete Current Cue
    pub fn get_current_column(&mut self) -> Option<CueColumn> {
        let col_idx = self.get_column_idx()?.clamp(0, 8);

        CueColumn::from_index(col_idx)
    }

    /// Delete Current Cue
    pub fn delete_cue(&mut self) {
        if let Some(i) = self.get_row_idx() {
            self.cues.remove(i);
        }
    }

    pub fn get_cell_str(&self, cue: &dyn Cue, column: &CueColumn) -> String {
        match column {
            CueColumn::Playing | CueColumn::Duration => String::new(),
            CueColumn::Name => cue.base().name.clone(),
            CueColumn::Number => cue.base().number.to_string(),
            CueColumn::PreWait => cue.base().prewait.to_string(),
            CueColumn::PostWait => cue.base().postwait.to_string(),
            CueColumn::Follow => cue.base().follow.icon().to_string(),
        }
    }

    pub fn get_current_cell_str(&mut self) -> Option<String> {
        let current_column: CueColumn = self.get_current_column()?;
        let current_cue: &dyn Cue = self.get_current_cue()?;

        let contents = self.get_cell_str(current_cue, &current_column);
        Some(contents)
    }

    pub const fn save_current_cell(&self) {}

    pub const fn copy_cue(&self) {}

    /// # Errors
    ///
    /// TODO
    pub fn copy_cell(&mut self, _column: &CueColumn) -> String {
        if let Some(_i) = self.get_current_cue_mut() {}
        String::new()
    }

    /// # Errors
    ///
    /// TODO
    pub fn change_cell(&mut self, column: &CueColumn) {
        // Current Cue
        if let Some(current_cue) = self.get_current_cue_mut() {
            match column {
                CueColumn::Playing | CueColumn::Duration => {}
                CueColumn::Name => current_cue.set_base().name = String::from("test"),
                CueColumn::Number => current_cue.set_base().number = 6.7,
                CueColumn::PreWait => current_cue.set_base().prewait = 6.7,
                CueColumn::PostWait => current_cue.set_base().postwait = 6.7,
                CueColumn::Follow => current_cue.set_base().follow = FollowState::Continue,
            }
        }
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
    pub fn delete_cell(&mut self, column: &CueColumn) {
        // Current Cue
        if let Some(current_cue) = self.get_current_cue_mut() {
            match column {
                CueColumn::Playing | CueColumn::Duration => {}
                CueColumn::Name => current_cue.set_base().name = String::new(),
                CueColumn::Number => current_cue.set_base().number = 0.0,
                CueColumn::PreWait => current_cue.set_base().prewait = 0.0,
                CueColumn::PostWait => current_cue.set_base().postwait = 0.0,
                CueColumn::Follow => current_cue.set_base().follow = FollowState::None,
            }
        }
    }
}
