//! # Insert Input Handler
//!
//! Handles key presses in the command line when in Insert mode.

use crate::model::{
    editor::Editor,
    keymaps::{InputCmd, Message},
};

/// Handler for Insert mode typing commands. Will always return None probably
pub fn input_handler(editor: &mut Editor, msg: &InputCmd) -> Option<Message> {
    match msg {
        InputCmd::DeleteChar => {
            editor.delete_char();
        }
        InputCmd::InsertChar(c) => {
            editor.input_char(*c);
        }
        InputCmd::MoveCursorLeft => {
            editor.move_cursor_left();
        }
        InputCmd::MoveCursorRight => {
            editor.move_cursor_right();
        }
        InputCmd::MoveCursorStart => {
            editor.reset_cursor();
        }
        InputCmd::MoveCursorEnd => {
            editor.move_cursor_end();
        }
        InputCmd::FinishInput => {
            editor.finish_input();
        }
    }
    None
}
