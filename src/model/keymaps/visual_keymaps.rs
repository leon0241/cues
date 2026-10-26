//! # Visual Mode Keymaps
//!
//! Handles the mapping of visual mode commands (without any combos). Currently not for use

use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{EditorCmd, Message};

/// Initialises Visual mode Keymaps.
///
/// These are all single press commands, and anything bigger goes into something else WIP.
///
/// # Examples
/// ```
/// use cues::model::keymaps::visual_keymaps::init_vmaps;
///
/// let vmaps = init_vmaps();
/// ```
#[must_use]
pub fn init_vmaps() -> HashMap<KeyCode, Message> {
    HashMap::from([(
        KeyCode::Esc,
        Message::Editor(EditorCmd::SaveInsertEnterNormal),
    )])
}
