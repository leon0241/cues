use crate::model::{cue::cue_base::CueColumn, cuestack::CueStack};

impl CueStack {
    /// Select the previous column.
    /// This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn go_left(&mut self) {
        self.current_state.select_previous_column();
    }

    /// Select the next item.
    /// This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn go_next(&mut self) {
        self.current_state.select_next();
    }

    /// Select the previous item.
    /// This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn go_previous(&mut self) {
        self.current_state.select_previous();
    }

    /// Select the next column.
    /// This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn go_right(&mut self) {
        self.current_state.select_next_column();
    }

    // Unselect the currently selected item if any. The implementation of `TableState` makes
    // sure that the stored offset is also reset.
    pub const fn unselect(&mut self) {
        self.current_state.select(None);
    }

    // Unselect the currently selected item if any. The implementation of `TableState` makes
    // sure that the stored offset is also reset.
    pub const fn select_cell_state(&mut self, row: usize, column: CueColumn) {
        self.current_state.select(Some(row));

        let col_i = CueColumn::to_index(column);
        self.current_state.select_column(Some(col_i));
    }
}
