use crate::model::cuestack::CueStack;

impl CueStack {
    /// Select the previous column.
    /// This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn go_left(&mut self) {
        self.current_cue.select_previous_column();
    }

    /// Select the next item.
    /// This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn go_next(&mut self) {
        self.current_cue.select_next();
    }

    /// Select the previous item.
    /// This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn go_previous(&mut self) {
        self.current_cue.select_previous();
    }

    /// Select the next column.
    /// This will not be reflected until the widget is drawn in the
    /// `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn go_right(&mut self) {
        self.current_cue.select_next_column();
    }

    // Unselect the currently selected item if any. The implementation of `TableState` makes
    // sure that the stored offset is also reset.
    pub const fn unselect(&mut self) {
        self.current_cue.select(None);
    }
}
