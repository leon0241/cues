use crate::model::{
    editor::{ComboState, Editor},
    keymaps::{ComboCmd, Message},
};

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub const fn combo_handler(
    editor: &mut Editor,
    cmd: &ComboCmd,
) -> color_eyre::Result<Option<Message>> {
    match cmd {
        ComboCmd::Return => {
            editor.set_combo_state(ComboState::None);
        }
        ComboCmd::EnterDelete => {
            editor.set_combo_state(ComboState::Delete);
        }
        ComboCmd::EnterCreate => {
            editor.set_combo_state(ComboState::Create);
        }
        ComboCmd::EnterChange => {
            editor.set_combo_state(ComboState::Change);
        }
        ComboCmd::EnterYank => {
            editor.set_combo_state(ComboState::Yank);
        }
    }
    Ok(None)
}
