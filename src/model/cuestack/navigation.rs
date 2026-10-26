//! # Cuestack Navigation
//!
//! Implementation for any movement based functions within the cuestack

use crate::model::{cue::CueColumn, cuestack::CueStack};

impl CueStack {
    /// Selects the previous column or the last column if no column is selected.
    ///
    /// See Ratatui `TableState::select_previous_column()` for more information.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// cuestack.go_left();
    /// ```
    pub fn go_left(&mut self) {
        self.current_state.select_previous_column();
    }

    /// Selects the next cue, or the first cue if no column is selected.
    ///
    /// See Ratatui `TableState::select_next()` for more information.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// cuestack.go_next();
    /// ```
    pub fn go_next(&mut self) {
        self.current_state.select_next();
    }

    /// Selects the previous cue, or the first cue if no column is selected.
    ///
    /// See Ratatui `TableState::select_previous()` for more information.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// cuestack.go_previous();
    /// ```
    pub fn go_previous(&mut self) {
        self.current_state.select_previous();
    }

    /// Select the previous columnn, or the first column if no column is selected.
    ///
    /// See Ratatui `TableState::select_next_column()` for more information.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// cuestack.go_right();
    /// ```
    pub fn go_right(&mut self) {
        self.current_state.select_next_column();
    }

    /// Unselect the currently selected item if any.
    ///
    /// The implementation of `TableState` makes
    /// sure that the stored offset is also reset.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// cuestack.unselect();
    /// ```
    pub const fn unselect(&mut self) {
        self.current_state.select(None);
    }

    /// Selects a cell state from a row (`usize`) and a column (`CueColumn`).
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::cue::CueColumn;
    ///
    /// let mut cuestack = CueStack::default();
    /// cuestack.select_cell_state(0, CueColumn::Name);
    /// ```
    pub const fn select_cell_state(&mut self, row: usize, column: CueColumn) {
        // TODO: do we need to clamp these values?
        self.current_state.select(Some(row));

        let col_i = CueColumn::to_index(column);
        self.current_state.select_column(Some(col_i));
    }
}
