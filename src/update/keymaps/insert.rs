use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{model::cuestack::CueStack, update::{keymaps::normal::editor::{EditorCmd, editor_handler}, keymaps_file::Message}};

const fn handle_insert(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Esc => Some(
            Message::Editor(EditorCmd::EnterNormal)
        ),
        KeyCode::Char('q') => Some(
            Message::Editor(EditorCmd::Quit)
        ),
        // KeyCode::Char('j') => Some(
        //     Message::Navigation(NavigationCmd::NextCue)
        // ),
        _ => None,
    }
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn update_insert(model: &mut CueStack, msg: &Message) -> color_eyre::Result<Option<Message>> {

    let return_msg: Option<Message> = match msg {
       Message::Editor(cmd) => {
           editor_handler(model, cmd)?
       } 
       _ => {None}
    };

    return_msg.map_or_else(|| Ok(None), |i| Ok(Some(i)))
}
