//! # Cuestack Modifiers
//!
//! Implementation for any cue based modification within the cuestack.

use crate::model::cue::{CopiedCell, Cue, CueColumn, CueType, FollowState};
use crate::model::cuestack::CueStack;

impl CueStack {
    /// Adds a new cue into the end of the `CueStack` with a specified `Cue`
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::cue::Cue;
    ///
    /// let mut cuestack = CueStack::default();
    /// let cue_to_add = Cue::default();
    /// cuestack.new_cue(cue_to_add);
    /// ```
    pub fn new_cue(&mut self, mut new_cue: Cue) {
        let last_cue_num: String = self
            .cues
            .last()
            .map_or(2_usize, |cue| {
                cue.number()
                    .parse::<usize>()
                    .unwrap_or(1_usize)
                    .wrapping_add(1)
            })
            .to_string();

        new_cue.set_number(last_cue_num);
        self.cues.push(new_cue);
    }

    /// Returns the current row index, or None if no cue is selected.
    ///
    /// See Ratatui `TableState::selected()` for more information.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// assert_eq!(cuestack.get_row_idx(), None);
    /// ```
    #[must_use]
    pub const fn get_row_idx(&self) -> Option<usize> {
        self.current_state.selected()
    }

    /// Returns the current column index, or None if no cue is selected.
    ///
    /// See Ratatui `TableState::selected_column()` for more information.
    const fn get_column_idx(&self) -> Option<usize> {
        self.current_state.selected_column()
    }

    /// Returns the current cue along with the cue's index, or None if no cue is selected.
    ///
    /// Cue is an immutable value, use `get_current_cue_mut()` for a mutable return value.
    ///
    /// The return value is a tuple, where the `Cue` is 0, and the index is 1.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// assert!(matches!(cuestack.get_current_cue(), None));
    /// ```
    #[must_use]
    pub fn get_current_cue(&self) -> Option<(&Cue, usize)> {
        let i = self.get_row_idx()?;

        self.cues.get(i).map_or_else(|| None, |cue| Some((cue, i)))
    }

    /// Returns the current cue along with the cue's index, or None if no cue is selected.
    ///
    /// Cue is a mutable value, use `get_current_cue()` for an immutable return value.
    ///
    /// The return value is a tuple, where the `Cue` is 0, and the index is 1.
    fn get_current_cue_mut(&mut self) -> Option<(&mut Cue, usize)> {
        let i = self.get_row_idx()?;

        self.cues
            .get_mut(i)
            .map_or_else(|| None, |cue| Some((cue, i)))
    }

    /// Returns the current column, or `None` if no column is selected.
    ///
    /// The value is clamped between 0 and 8 as that is the number of columns in the table.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// assert_eq!(cuestack.get_current_column(), None);
    /// ```
    pub fn get_current_column(&mut self) -> Option<CueColumn> {
        let col_idx = self.get_column_idx()?.clamp(0, 8);

        CueColumn::from_index(col_idx)
    }

    /// Gets a string value for the currently selected `Cue` and `CueColumn`
    ///
    /// Returns None if either a column or cue is not selected.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::cue::Cue;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.set_items(vec![Cue::default()]);
    ///
    /// // manually move to the number column of cue 1...
    /// cuestack.go_next(); cuestack.go_right(); cuestack.go_right(); cuestack.go_right();
    ///
    /// assert_eq!(cuestack.get_current_cell_str().unwrap(), String::new());
    /// ```
    pub fn get_current_cell_str(&mut self) -> Option<String> {
        let current_column: CueColumn = self.get_current_column()?;
        let current_cue: &Cue = self.get_current_cue()?.0;

        let contents = current_cue.get_cell_str(current_column);
        Some(contents)
    }

    /// Sets a string value for the currently selected `Cue` and `CueColumn`
    ///
    /// # Errors
    ///
    /// Errors if the column is a File target, and the file cannot be read
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::cue::Cue;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.set_items(vec![Cue::default()]);
    ///
    /// // manually move to the number column of cue 1...
    /// cuestack.go_next(); cuestack.go_right(); cuestack.go_right(); cuestack.go_right();
    ///
    /// cuestack.set_current_cell_str(String::from("1"));
    /// ```
    pub fn set_current_cell_str(&mut self, value: String) -> color_eyre::Result<()> {
        let Some(current_column) = self.get_current_column() else {
            return Ok(());
        };

        let Some(current_cue) = self.get_current_cue_mut() else {
            return Ok(());
        };

        current_cue.0.set_cell_str(&current_column, value)?;
        Ok(())
    }

    /// Sets a cell from the current cue, as well as a specified column
    ///
    /// # Errors
    ///
    /// Errors if the column is a File target, and the file cannot be read
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::cue::{Cue, CueColumn};
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.set_items(vec![Cue::default()]);
    ///
    /// cuestack.go_next();
    ///
    /// cuestack.set_cell_from_col_str(&CueColumn::Number, String::from("1"));
    /// ```
    pub fn set_cell_from_col_str(
        &mut self,
        column: &CueColumn,
        value: String,
    ) -> color_eyre::Result<()> {
        let Some(current_cue) = self.get_current_cue_mut() else {
            return Ok(());
        };

        current_cue.0.set_cell_str(column, value)?;
        Ok(())
    }

    /// Creates a Clone of the currently selected cue.
    ///
    /// Returns a blank audio cue if no cue is selected.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// let copied_cue = cuestack.copy_cue();
    /// ```
    #[must_use]
    pub fn copy_cue(&self) -> Cue {
        // Current Cue
        let Some(current_cue) = self.get_current_cue() else {
            return Cue::default();
        };

        current_cue.0.clone()
    }

    /// Copies the cell of the current cue, as well as a specified column.
    ///
    /// The copied value is of form `CopiedCell` to make conversions easier.
    ///
    /// Returns a `CopiedCell::None` if there is no cue selected.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::cue::CueColumn;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// let copied_cell = cuestack.copy_cell(&CueColumn::Number);
    /// ```
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

    /// Copies the cell of the current cue, as well as the current column.
    ///
    /// The copied value is of form `CopiedCell` to make conversions easier.
    ///
    /// Returns a `CopiedCell::None` if there is no column or cue selected.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// let copied_cell = cuestack.copy_current_cell();
    /// ```
    pub fn copy_current_cell(&mut self) -> CopiedCell {
        let Some(current_col) = self.get_current_column() else {
            return CopiedCell::None;
        };

        self.copy_cell(&current_col)
    }

    /// Clears the values of the current cue, or returns if there is no cue selected.
    ///
    /// This additionally sets the state to the cues Name column.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.clear_cue();
    /// ```
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

    /// Delete Current Cue, or returns if there are no cues or no cue is selected.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.delete_cue();
    /// ```
    pub fn delete_cue(&mut self) {
        if let Some(i) = self.get_row_idx() {
            self.cues.remove(i);
        }
    }

    /// Delete a Cell alongside a specified column
    ///
    /// Returns if there are no cues or no cue is selected.
    ///
    /// Additionally sets the table state to the recently deleted column.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::cue::CueColumn;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.delete_cell(&CueColumn::Number);
    /// ```
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

    /// Delete the current Cell, or returns if there are no cues, or no cue is selected.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.delete_current_cell();
    /// ```
    pub fn delete_current_cell(&mut self) {
        let Some(current_col) = self.get_current_column() else {
            return;
        };

        self.delete_cell(&current_col);
    }
}
