use crate::model::{
    cuestack::CueStack,
    editor::{Editor, Mode},
    keymaps::{EditorCmd, Message},
};

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub const fn editor_handler(
    editor: &mut Editor,
    cuestack: &mut CueStack,
    cmd: &EditorCmd,
) -> color_eyre::Result<Option<Message>> {
    match cmd {
        EditorCmd::Quit => {
            editor.set_running_state(crate::model::editor::RunningState::Done);
        }
        EditorCmd::EnterInsertStart => {
            cuestack.edit_current_cell_start();

            editor.set_mode(Mode::Insert);
        }
        EditorCmd::EnterInsertEnd => {
            cuestack.edit_current_cell_end();

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
    Ok(None)
}
