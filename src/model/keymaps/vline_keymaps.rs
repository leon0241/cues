//! # Visual Line Mode Keymaps
//!
//! Handles the mapping of visual mode commands (without any combos). Currently not for use

use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{EditorCmd, Message};

/// Initialises Visual Line mode Keymaps.
///
/// These are all single press commands, and anything bigger goes into something else WIP.
///
/// # Examples
/// ```
/// use cues::model::keymaps::vline_keymaps::init_vlmaps;
///
/// let vlmaps = init_vlmaps();
/// ```
#[must_use]
pub fn init_vlmaps() -> HashMap<KeyCode, Message> {
    HashMap::from([(
        KeyCode::Esc,
        Message::Editor(EditorCmd::SaveInsertEnterNormal),
    )])
}
