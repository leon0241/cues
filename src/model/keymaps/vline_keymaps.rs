use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{EditorCmd, Message};

#[must_use]
pub fn init_vlmaps() -> HashMap<KeyCode, Message> {
    HashMap::from([(
        KeyCode::Esc,
        Message::Editor(EditorCmd::SaveInsertEnterNormal),
    )])
}
