//! # Insert Mode Keymaps
//!
//! Handles the mapping of keys to Messages within Insert Mode.
//!
//! This also handles typing of characters

use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{EditorCmd, InputCmd, Message};

/// Initialises Insert Keymaps.
///
/// These are Insert Mode commands, while typing commands are handled within a match statement
/// inside `insert_char_map()` as they will never need to be remapped.
///
/// # Examples
/// ```
/// use cues::model::keymaps::insert_keymaps::init_imaps;
///
/// let imaps = init_imaps();
/// ```
#[must_use]
pub fn init_imaps() -> HashMap<KeyCode, Message> {
    HashMap::from([
        // Esc discards changes and exits insert
        (
            KeyCode::Esc,
            Message::Editor(EditorCmd::CancelInsertEnterNormal),
        ),
        // Enter saves changes and exits insert
        (
            KeyCode::Enter,
            Message::Editor(EditorCmd::SaveInsertEnterNormal),
        ),
    ])
}

/// Initialises Typing charmaps.
///
/// These are typing commands such as typing keys or handling backspace, handled as a match
/// statement as they will never need to be remapped. On the other hand, any Insert mode commands
/// are handled from a `HashMap` within `init_imaps()`
///
/// # Examples
/// ```
/// use ratatui::crossterm::event::KeyCode;
/// use cues::model::keymaps::insert_keymaps::insert_char_map;
///
/// let i_char_maps = insert_char_map(KeyCode::Char('a'));
/// ```
#[must_use]
pub fn insert_char_map(key: KeyCode) -> Option<Message> {
    // Shorthand to turn InputCmd::X into Some(Message::Input(InputCmd::X))
    let wrapper = |cmd: InputCmd| Some(Message::Input(cmd));

    match key {
        // Backspace
        KeyCode::Backspace => wrapper(InputCmd::DeleteChar),
        // Any alphabet
        KeyCode::Char(c) => wrapper(InputCmd::InsertChar(c)),
        // Left Key
        KeyCode::Left => wrapper(InputCmd::MoveCursorLeft),
        // Right Key
        KeyCode::Right => wrapper(InputCmd::MoveCursorRight),
        // Up Key
        KeyCode::Up => wrapper(InputCmd::MoveCursorStart),
        // Down Key
        KeyCode::Down => wrapper(InputCmd::MoveCursorEnd),
        _ => None,
    }
}
