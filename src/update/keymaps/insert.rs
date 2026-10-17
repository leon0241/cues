pub mod editor;
pub mod input;

use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    model::{
        cuestack::CueStack,
        editor::Editor,
        full_model::Model,
        keymaps::{Message, insert_keymaps::insert_char_map},
    },
    update::keymaps::insert::{editor::i_editor_handler, input::input_handler},
};

#[must_use]
pub fn handle_insert(key: KeyCode, _modifier: KeyModifiers, editor: &Editor) -> Option<Message> {
    let keymaps = editor.keymaps();

    // Look through regular insert maps first
    let imaps = keymaps.insert_maps().get(&key).copied();

    imaps.map_or_else(|| insert_char_map(key), Some)
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn update_insert(model: &mut Model, msg: &Message) -> color_eyre::Result<Option<Message>> {
    let (editor, cuestack): (&mut Editor, &mut CueStack) = model.full_mut();

    let return_msg = match msg {
        Message::Editor(cmd) => i_editor_handler(editor, cuestack, cmd)?,
        Message::Input(cmd) => input_handler(editor, cmd)?,
        _ => None,
    };

    return_msg.map_or_else(|| Ok(None), |i| Ok(Some(i)))
}
