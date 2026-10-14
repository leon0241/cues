use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{model::{cuestack::CueStack, editor::ComboState}, update::keymaps_file::Message};


pub mod navigation;
pub mod modifiers;
pub mod playback;
pub mod editor;
pub mod modkey;


const fn handle_normal(key: KeyCode, modifier: KeyModifiers, combo_state: ComboState) -> Option<Message> {
    match combo_state {
        // No additional combo
        ComboState::None => {
            if let Some(i) = normal_mode_switch(key, modifier) {
                return Some(i);
            }
            if let Some(i) = navigation_key(key, modifier) {
                return Some(i);
            }
            if let Some(i) = modifier_key(key, modifier) {
                return Some(i);
            }
            if let Some(i) = playback_key(key, modifier) {
                return Some(i);
            }
            if let Some(i) = editor_key(key, modifier) {
                return Some(i);
            }
        }
        // Yank, Delete, Create, Change
        combo_state => {
            if let Some(i) = mod_key(key, modifier, combo_state) {
                return Some(i);
            }
        }
    }
    None
}


/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn update_normal(model: &mut CueStack, msg: &Message) -> color_eyre::Result<Option<Message>> {
    // TODO: check all these guys for errors
    
    let return_msg: Option<Message> = match msg {
        Message::Navigation(cmd) => {
            navigation_handler(model, cmd)?
        },
        Message::Modifier(cmd) => {
            modifier_handler(model, cmd)?
        },
        Message::Playback(cmd) => {
            playback_handler(model, cmd)?
        },
        Message::Editor(cmd) => {
            editor_handler(model, cmd)?
        }
        Message::ModKey(cmd) => {
            mod_key_handler(model, cmd)?
        }
    };

    return_msg.map_or_else(|| Ok(None), |i| Ok(Some(i)))
}
