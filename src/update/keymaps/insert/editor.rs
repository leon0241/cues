use crate::model::{
    cuestack::CueStack,
    editor::{Editor, Mode},
    keymaps::{EditorCmd, InputCmd, Message},
};

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub const fn i_editor_handler(
    editor: &mut Editor,
    cuestack: &mut CueStack,
    cmd: &EditorCmd,
) -> color_eyre::Result<Option<Message>> {
    let return_msg = match cmd {
        EditorCmd::EnterNormal => {
            cuestack.save_current_cell();

            editor.set_mode(Mode::Normal);

            Some(Message::Input(InputCmd::FinishInput))
        }
        _ => None,
    };

    Ok(return_msg)
}
