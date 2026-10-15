use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{model::{cuestack::CueStack, editor::Editor, full_model::Model, keymaps::Message}, update::keymaps::normal::editor::editor_handler};

#[must_use]
pub fn handle_insert(key: KeyCode, _modifier: KeyModifiers, editor: &Editor) -> Option<Message> {
    let keymaps = editor.keymaps();

    keymaps.insert_maps().get(&key).copied()
}


/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn update_insert(model: &mut Model, msg: &Message) -> color_eyre::Result<Option<Message>> {

    let (editor, cuestack): (&mut Editor, &mut CueStack) = model.full_mut();

    let return_msg: Option<Message> = match msg {
       Message::Editor(cmd) => {
           editor_handler(
               editor,
               cuestack,
               cmd
           )?
       } 
       _ => {None}
    };

    return_msg.map_or_else(|| Ok(None), |i| Ok(Some(i)))
}
