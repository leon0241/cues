//! # Visual Mode Update Handler
//!
//! Everything you need to update in Visual Mode.

use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::model::{
    editor::{ComboState, Editor},
    full_model::Model,
    keymaps::Message,
};

/// # Visual Mode
///
/// Turns a `KeyCode` into a `Message`
///
/// Matches against any `KeyMaps`.
#[must_use]
pub fn handle_visual(key: KeyCode, _modifier: KeyModifiers, editor: &Editor) -> Option<Message> {
    let keymaps = editor.keymaps();

    match editor.combo_state() {
        // No additional combo
        ComboState::None => keymaps.normal_maps().get(&key).copied(),

        // Yank, Delete, Create, Change
        ComboState::Yank => keymaps.yank_maps().get(&key).copied(),
        ComboState::Delete => keymaps.delete_maps().get(&key).copied(),
        ComboState::Create => keymaps.create_maps().get(&key).copied(),
        ComboState::Change => keymaps.change_maps().get(&key).copied(),
        // Z -> folding?
    }
}

/// # Visual Mode
///
/// Reads a Message and updates the `CueStack` or `Editor` in response
///
/// # Errors
///
/// Will return `Err` if a file cell is set and the file path cannot be read.
pub const fn update_visual(
    _model: &mut Model,
    _msg: &Message,
) -> color_eyre::Result<Option<Message>> {
    // TODO: implement this
    // match msg {
    //     Message::Navigation(cmd) => navigation_handler(model, cmd)?,
    //     Message::Modifier(cmd) => modifier_handler(model, cmd)?,
    //     Message::Playback(cmd) => playback_handler(model, cmd)?,
    //     Message::Editor(cmd) => editor_handler(model, cmd)?,
    // }
    Ok(None)
}
