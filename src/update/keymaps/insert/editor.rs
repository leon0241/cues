//! # Insert Mode Editor Handler
//!
//! Handles `Editor` updates when in Insert mode.

use crate::model::{
    cuestack::CueStack,
    editor::{Editor, Mode},
    keymaps::{EditorCmd, InputCmd, Message},
};

/// Handler for Insert mode Editor commands
///
/// # Errors
///
/// Will return `Err` if a file cell is set and the file path cannot be read.
pub fn i_editor_handler(
    editor: &mut Editor,
    cuestack: &mut CueStack,
    cmd: &EditorCmd,
) -> color_eyre::Result<Option<Message>> {
    let return_msg = match cmd {
        EditorCmd::CancelInsertEnterNormal => {
            editor.reset_input_buffer();
            editor.reset_cursor();

            editor.set_mode(Mode::Normal);

            Some(Message::Input(InputCmd::FinishInput))
        }
        EditorCmd::SaveInsertEnterNormal => {
            let string: String = editor.input_buffer();
            cuestack.set_current_cell_str(string)?;

            editor.reset_input_buffer();
            editor.reset_cursor();

            editor.set_mode(Mode::Normal);

            Some(Message::Input(InputCmd::FinishInput))
        }
        _ => None,
    };

    Ok(return_msg)
}
