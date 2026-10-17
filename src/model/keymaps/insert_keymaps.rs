use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{EditorCmd, InputCmd, Message};

#[must_use]
pub fn init_imaps() -> HashMap<KeyCode, Message> {
    HashMap::from([
        (KeyCode::Esc, Message::Editor(EditorCmd::EnterNormal)),
        (KeyCode::Enter, Message::Editor(EditorCmd::EnterNormal)),
    ])
}

#[must_use]
pub fn insert_char_map(key: KeyCode) -> Option<Message> {
    let wrapper = |cmd: InputCmd| Some(Message::Input(cmd));

    match key {
        KeyCode::Backspace => wrapper(InputCmd::DeleteChar),
        KeyCode::Char(c) => wrapper(InputCmd::InsertChar(c)),
        KeyCode::Left => wrapper(InputCmd::MoveCursorLeft),
        KeyCode::Right => wrapper(InputCmd::MoveCursorRight),
        KeyCode::Up => wrapper(InputCmd::MoveCursorStart),
        KeyCode::Down => wrapper(InputCmd::MoveCursorEnd),
        _ => None,
    }
}
