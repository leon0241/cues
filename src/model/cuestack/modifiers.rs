use crate::model::cues::Cue;
use crate::model::cues::CueColumn;
use crate::model::cues::cue_base::FollowState;
use crate::model::cuestack::CueStack;

impl CueStack {
    pub fn new_cue(&mut self, new_cue: Cue) {
        self.cues.push(new_cue);
    }

    #[must_use]
    pub const fn get_row_idx(&self) -> Option<usize> {
        self.current_state.selected()
    }

    const fn get_column_idx(&self) -> Option<usize> {
        self.current_state.selected_column()
    }

    #[must_use]
    pub fn get_current_cue(&self) -> Option<&Cue> {
        let i = self.get_row_idx()?;

        // idk why clippy wants this
        self.cues.get(i)
    }

    fn get_current_cue_mut(&mut self) -> Option<&mut Cue> {
        let i = self.get_row_idx()?;

        self.cues.get_mut(i)
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

    #[must_use]
    pub fn get_cell_str(&self, cue: &Cue, column: &CueColumn) -> String {
        match column {
            CueColumn::Playing | CueColumn::Duration => String::new(),
            CueColumn::Name => cue.name.clone(),
            CueColumn::Number => cue.number.clone(),
            CueColumn::PreWait => cue.prewait.to_string(),
            CueColumn::PostWait => cue.postwait.to_string(),
            CueColumn::Follow => cue.follow.icon().to_string(),
            CueColumn::Target => String::from("erm target"),
        }
    }

    pub fn get_current_cell_str(&mut self) -> Option<String> {
        let current_column: CueColumn = self.get_current_column()?;
        let current_cue: &Cue = self.get_current_cue()?;

        let contents = self.get_cell_str(current_cue, &current_column);
        Some(contents)
    }

    pub fn set_cell_str(cue: &mut Cue, column: &CueColumn, value: String) {
        fn parser(value: &str) -> f32 {
            value.parse::<f32>().unwrap_or_default()
        }

        match column {
            CueColumn::Playing | CueColumn::Duration => {}
            CueColumn::Name => cue.name = value,
            CueColumn::Number => cue.number = value,
            CueColumn::PreWait => cue.prewait = parser(&value),
            CueColumn::PostWait => cue.postwait = parser(&value),
            CueColumn::Follow => cue.follow = FollowState::Follow,
            CueColumn::Target => cue.set_target(value),
        }
    }

    pub fn set_current_cell_str(&mut self, value: String) {
        let Some(current_column) = self.get_current_column() else {
            return;
        };

        let Some(current_cue) = self.get_current_cue_mut() else {
            return;
        };

        Self::set_cell_str(current_cue, &current_column, value);
    }

    pub fn set_cell_from_col_str(&mut self, column: &CueColumn, value: String) {
        let Some(current_cue) = self.get_current_cue_mut() else {
            return;
        };

        Self::set_cell_str(current_cue, column, value);
    }

    pub fn change_cell_from_col_str(&mut self, column: &CueColumn) {
        let Some(current_cue) = self.get_current_cue_mut() else {
            return;
        };

        Self::set_cell_str(current_cue, column, String::new());
    }

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
                CueColumn::Name => current_cue.name = String::new(),
                CueColumn::Number => current_cue.number = String::new(),
                CueColumn::Target => current_cue.set_target(String::new()),
                CueColumn::PreWait => current_cue.prewait = 0.0,
                CueColumn::PostWait => current_cue.postwait = 0.0,
                CueColumn::Follow => current_cue.follow = FollowState::None,
            }
        }
    }
}
