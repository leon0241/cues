pub mod combo;
pub mod create;
pub mod editor;
pub mod modkey;
pub mod navigation;
pub mod playback;

use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{
    model::{
        cuestack::CueStack,
        editor::{ComboState, Editor},
        full_model::Model,
        keymaps::Message,
    },
    update::keymaps::normal::{
        combo::combo_handler, create::create_handler, editor::n_editor_handler,
        modkey::modkey_handler, navigation::navigation_handler, playback::playback_handler,
    },
};

/// # Normal Mode
///
/// Takes a ``KeyCode`` and returns a ``Message``
/// If there is no map, then returns ``None``
#[must_use]
pub fn handle_normal(key: KeyCode, _modifier: KeyModifiers, editor: &Editor) -> Option<Message> {
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

/// # Normal Mode
///
/// Takes a ``Message`` and processes a functions.
/// If the action is finished, returns ``None``.
/// Otherwise, it returns another message to chain.
///
/// # Example
///
/// ``Message::ModKey(ModKeyCmd::Change(CueCmd::Name))``
/// -> ``Message::``
///
/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn update_normal(model: &mut Model, msg: &Message) -> color_eyre::Result<Option<Message>> {
    // TODO: check all these guys for errors

    let (editor, cuestack): (&mut Editor, &mut CueStack) = model.full_mut();

    let return_msg: Option<Message> = match msg {
        Message::Navigation(cmd) => navigation_handler(cuestack, cmd),
        Message::Playback(cmd) => playback_handler(cuestack, cmd)?,
        Message::Editor(cmd) => n_editor_handler(editor, cuestack, cmd),
        Message::ModKey(cmd) => modkey_handler(editor, cuestack, cmd)?,
        Message::Combo(cmd) => combo_handler(editor, cmd)?,
        // TODO: this
        Message::None => None,
        Message::Input(_cmd) => None, // only insert
        Message::Create(cmd) => create_handler(cuestack, editor, cmd)?,
    };

    return_msg.map_or_else(|| Ok(None), |i| Ok(Some(i)))
}
