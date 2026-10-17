use crate::model::{
    cuestack::CueStack,
    editor::{Editor, Mode},
    keymaps::{EditorCmd, Message},
};

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn n_editor_handler(
    editor: &mut Editor,
    cuestack: &mut CueStack,
    cmd: &EditorCmd,
) -> Option<Message> {
    match cmd {
        EditorCmd::Quit => {
            editor.set_running_state(crate::model::editor::RunningState::Done);
        }
        EditorCmd::EnterInsertStart => {
            // Get current cell value
            // Put that into the buffer
            // Enter Insert
            if let Some(str) = cuestack.get_current_cell_str() {
                let len = str.len();
                editor.set_input_buffer(str);
                editor.set_cursor_idx(len);
            }

            editor.set_mode(Mode::Insert);
        }
        EditorCmd::EnterInsertEnd => {
            // Get current cell value
            // Put that into the buffer
            // Enter Insert
            // TODO: get rid of this result
            if let Some(str) = cuestack.get_current_cell_str() {
                editor.set_input_buffer(str);
                editor.reset_cursor();
            }

            editor.set_mode(Mode::Insert);
        }
        EditorCmd::EnterNormal => {
            cuestack.save_current_cell();

            editor.set_mode(Mode::Normal);
        }
        EditorCmd::EnterVisual => {
            editor.set_mode(Mode::Visual);
        }
    }
    None
}
