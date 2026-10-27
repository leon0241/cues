//! # Normal Mode Combo Handler
//!
//! Handles the changing of `ComboState` for combo presses

use crate::model::{
    editor::{ComboState, Editor},
    keymaps::{ComboCmd, Message},
};

/// Handler for setting Combo State
pub const fn combo_handler(editor: &mut Editor, cmd: &ComboCmd) -> Option<Message> {
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
    None
}
