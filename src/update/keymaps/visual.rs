use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{model::{cuestack::CueStack, editor::{ComboState, Mode}}, update::{keymaps::normal::navigation::NavigationCmd, keymaps_file::Message}};

const fn handle_visual(key: KeyCode, _modifier: KeyModifiers, combo_state: ComboState) -> Option<Message> {
    match key {
        KeyCode::Char('k') => Some(
            Message::Navigation(NavigationCmd::PrevCue)
        ),
        KeyCode::Char('j') => Some(
            Message::Navigation(NavigationCmd::NextCue)
        ),
        _ => None,
    }
}


/// # Errors
///
/// Will return `Err` if cannot handle a key
pub const fn update_visual(model: &mut CueStack, msg: &Message) -> color_eyre::Result<(Mode, Option<Message>)> {
    // TODO: check all these guys for errors
    // match msg {
    //     Message::Navigation(cmd) => navigation_handler(model, cmd)?,
    //     Message::Modifier(cmd) => modifier_handler(model, cmd)?,
    //     Message::Playback(cmd) => playback_handler(model, cmd)?,
    //     Message::Editor(cmd) => editor_handler(model, cmd)?,
    // }
    Ok((Mode::Insert, None))
}
