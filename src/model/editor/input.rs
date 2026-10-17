use crate::model::editor::Editor;

impl Editor {
    #[must_use]
    pub const fn cursor_idx(&self) -> usize {
        self.cursor_idx
    }

    pub const fn reset_cursor(&mut self) {
        self.cursor_idx = 0;
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_buffer.chars().count())
    }

    pub fn move_cursor_left(&mut self) {
        let moved_left = self.cursor_idx.saturating_sub(1);
        self.cursor_idx = self.clamp_cursor(moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let moved_right = self.cursor_idx.saturating_add(1);
        self.cursor_idx = self.clamp_cursor(moved_right);
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input_buffer
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_idx)
            .unwrap_or(self.input_buffer.len())
    }

    pub fn move_cursor_end(&mut self) {
        let index = self.byte_index();
        self.cursor_idx = self.clamp_cursor(index);
    }

    pub fn input_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input_buffer.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_idx != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_idx;
            let from_left_to_current_index = current_index.saturating_sub(1);

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input_buffer.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input_buffer.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input_buffer = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    /// Clears the buffer and returns the string
    pub fn finish_input(&mut self) -> String {
        let msg: String = self.input_buffer.clone();
        self.input_buffer.clear();
        self.reset_cursor();
        msg
    }
}
