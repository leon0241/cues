pub mod navigation;
pub mod modifiers;
pub mod playback;
pub mod editor;
pub mod modkey;

use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    model::{
        cuestack::{CueStack},
        editor::{ComboState, Editor},
        full_model::Model, keymaps::Message
    },
    update::keymaps::normal::{
        editor::editor_handler,
        modifiers::modifier_handler,
        modkey::modkey_handler,
        navigation::navigation_handler,
        playback::playback_handler
    }
};

#[must_use]
pub fn handle_normal(key: KeyCode, _modifier: KeyModifiers, editor: &Editor) -> Option<Message> {
    let keymaps = editor.keymaps();

    match editor.combo_state() {
        // No additional combo
        ComboState::None => {
            keymaps.normal_maps().get(&key).copied()
        }

        // Yank, Delete, Create, Change
        ComboState::Yank => {
            keymaps.yank_maps().get(&key).copied()
        }
        ComboState::Delete => {
            keymaps.delete_maps().get(&key).copied()
        },
        ComboState::Create => {
            keymaps.create_maps().get(&key).copied()
        },
        ComboState::Change => {
            keymaps.change_maps().get(&key).copied()
        },
        // Z -> folding?
    }
}


/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn update_normal(model: &mut Model, msg: &Message) -> color_eyre::Result<Option<Message>> {
    // TODO: check all these guys for errors

    let (editor, cuestack): (&mut Editor, &mut CueStack) = model.full_mut();
    
    let return_msg: Option<Message> = match msg {
        Message::Navigation(cmd) => {
            navigation_handler(cuestack, cmd)?
        },
        Message::Modifier(cmd) => {
            modifier_handler(cuestack, cmd)?
        },
        Message::Playback(cmd) => {
            playback_handler(cuestack, cmd)?
        },
        Message::Editor(cmd) => {
            editor_handler(editor, cuestack, cmd)?
        }
        Message::ModKey(cmd) => {
            modkey_handler(cuestack, cmd)?
        }
        // TODO: this
        Message::None => None,
        Message::Create(_create_cmd) => None,
    };

    return_msg.map_or_else(|| Ok(None), |i| Ok(Some(i)))
}
