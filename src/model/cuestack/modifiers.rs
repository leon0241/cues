use crate::model::cue::{CopiedCell, Cue, CueColumn, CueType, FollowState};
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
    pub fn get_current_cue(&self) -> Option<(&Cue, usize)> {
        let i = self.get_row_idx()?;

        self.cues.get(i).map_or_else(|| None, |cue| Some((cue, i)))
    }

    fn get_current_cue_mut(&mut self) -> Option<(&mut Cue, usize)> {
        let i = self.get_row_idx()?;

        self.cues
            .get_mut(i)
            .map_or_else(|| None, |cue| Some((cue, i)))
    }

    /// Delete Current Cue
    pub fn get_current_column(&mut self) -> Option<CueColumn> {
        let col_idx = self.get_column_idx()?.clamp(0, 8);

        CueColumn::from_index(col_idx)
    }

    #[must_use]
    pub fn get_cell_str(&self, cue: &Cue, column: &CueColumn) -> String {
        match column {
            CueColumn::Playing | CueColumn::Duration => String::new(),
            CueColumn::Name => cue.name(),
            CueColumn::Number => cue.number(),
            CueColumn::PreWait => cue.prewait().to_string(),
            CueColumn::PostWait => cue.postwait().to_string(),
            CueColumn::Follow => cue.follow().icon().to_string(),
            CueColumn::Target => match cue.cue_type() {
                CueType::Audio(cue) => cue
                    .file()
                    .as_ref()
                    .map_or(String::new(), |file| file.file_path().to_string()),
                CueType::Fade(t) | CueType::Stop(t) | CueType::Pause(t) | CueType::Start(t) => {
                    t.target().unwrap_or(String::new())
                }
            },
        }
    }

    pub fn get_current_cell_str(&mut self) -> Option<String> {
        let current_column: CueColumn = self.get_current_column()?;
        let current_cue: &Cue = self.get_current_cue()?.0;

        let contents = self.get_cell_str(current_cue, &current_column);
        Some(contents)
    }

    /// # Errors
    ///
    /// Errors if the file cannot be read
    pub fn set_cell_str(
        cue: &mut Cue,
        column: &CueColumn,
        value: String,
    ) -> color_eyre::Result<()> {
        fn parser(value: &str) -> f32 {
            value.parse::<f32>().unwrap_or_default()
        }

        match column {
            CueColumn::Playing | CueColumn::Duration => {}
            CueColumn::Name => cue.set_name(value),
            CueColumn::Number => cue.set_number(value),
            CueColumn::PreWait => cue.set_prewait(parser(&value)),
            CueColumn::PostWait => cue.set_postwait(parser(&value)),
            CueColumn::Follow => cue.set_follow(FollowState::Follow),
            CueColumn::Target => match cue.cue_type_mut() {
                CueType::Audio(cue) => {
                    if let Some(i) = cue.file_mut().as_mut() {
                        i.set_file_path(&value);
                        i.try_read()?;
                    }
                }
                CueType::Fade(_) | CueType::Stop(_) | CueType::Pause(_) | CueType::Start(_) => {
                    cue.set_target(value);
                }
            },
        }
        Ok(())
    }

    /// # Errors
    ///
    /// Errors if a file that has been set cannot be read
    pub fn set_current_cell_str(&mut self, value: String) -> color_eyre::Result<()> {
        let Some(current_column) = self.get_current_column() else {
            return Ok(());
        };

        let Some(current_cue) = self.get_current_cue_mut() else {
            return Ok(());
        };

        Self::set_cell_str(current_cue.0, &current_column, value)?;
        Ok(())
    }

    /// # Errors
    ///
    /// Errors if a file that has been set cannot be read
    pub fn set_cell_from_col_str(
        &mut self,
        column: &CueColumn,
        value: String,
    ) -> color_eyre::Result<()> {
        let Some(current_cue) = self.get_current_cue_mut() else {
            return Ok(());
        };

        Self::set_cell_str(current_cue.0, column, value)?;
        Ok(())
    }

    #[must_use]
    pub fn copy_cue(&self) -> Cue {
        // Current Cue
        let Some(current_cue) = self.get_current_cue() else {
            return Cue::default();
        };

        current_cue.0.clone()
    }

    /// # Errors
    ///
    /// TODO
    pub fn copy_cell(&mut self, column: &CueColumn) -> CopiedCell {
        let Some(current_cue) = self.get_current_cue() else {
            return CopiedCell::None;
        };

        match column {
            CueColumn::Number => CopiedCell::CueNumber(current_cue.0.number()),
            CueColumn::Name => CopiedCell::Name(current_cue.0.name()),
            CueColumn::PreWait => CopiedCell::Time(current_cue.0.prewait()),
            CueColumn::Playing | CueColumn::Duration => CopiedCell::None,
            CueColumn::PostWait => CopiedCell::Time(current_cue.0.postwait()),
            CueColumn::Follow => CopiedCell::Follow(current_cue.0.follow()),
            CueColumn::Target => match current_cue.0.cue_type() {
                CueType::Audio(file) => file.file().as_ref().map_or(CopiedCell::None, |file| {
                    CopiedCell::FilePath(file.file_path().to_string())
                }),
                CueType::Fade(t) | CueType::Stop(t) | CueType::Pause(t) | CueType::Start(t) => {
                    CopiedCell::Target(t.clone())
                }
            },
        }
    }

    pub fn copy_current_cell(&mut self) -> CopiedCell {
        let Some(current_col) = self.get_current_column() else {
            return CopiedCell::None;
        };

        self.copy_cell(&current_col)
    }

    /// # Errors
    ///
    /// TODO
    pub fn clear_cue(&mut self) {
        // Current Cue
        let Some(current_cue) = self.get_current_cue_mut() else {
            return;
        };

        // Reference to free mutable ref after match
        let idx = current_cue.1;

        current_cue.0.set_name(String::new());
        current_cue.0.set_number(String::new());
        current_cue.0.set_target(String::new());
        current_cue.0.set_prewait(0.0);
        current_cue.0.set_postwait(0.0);
        current_cue.0.set_follow(FollowState::None);

        // cc > default start at name column
        self.select_cell_state(idx, CueColumn::Name);
    }

    /// Delete Current Cue
    pub fn delete_cue(&mut self) {
        if let Some(i) = self.get_row_idx() {
            self.cues.remove(i);
        }
    }

    /// # Errors
    ///
    /// TODO
    pub fn delete_cell(&mut self, column: &CueColumn) {
        // Current Cue
        let Some(current_cue) = self.get_current_cue_mut() else {
            return;
        };

        // Reference to free mutable ref after match
        let idx = current_cue.1;

        match column {
            CueColumn::Playing | CueColumn::Duration => {}
            CueColumn::Name => current_cue.0.set_name(String::new()),
            CueColumn::Number => current_cue.0.set_number(String::new()),
            CueColumn::Target => current_cue.0.set_target(String::new()),
            CueColumn::PreWait => current_cue.0.set_prewait(0.0),
            CueColumn::PostWait => current_cue.0.set_postwait(0.0),
            CueColumn::Follow => current_cue.0.set_follow(FollowState::None),
        }

        self.select_cell_state(idx, *column);
    }

    pub fn delete_current_cell(&mut self) {
        let Some(current_col) = self.get_current_column() else {
            return;
        };

        self.delete_cell(&current_col);
    }
}
