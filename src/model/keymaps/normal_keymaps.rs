use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{ComboCmd, EditorCmd, Message, ModKeyCmd, NavigationCmd, PlaybackCmd};

#[must_use]
pub fn init_nmaps() -> HashMap<KeyCode, Message> {
    HashMap::from([
        (
            KeyCode::Char('h'),
            Message::Navigation(NavigationCmd::LeftCol),
        ),
        (
            KeyCode::Char('k'),
            Message::Navigation(NavigationCmd::PrevCue),
        ),
        (
            KeyCode::Char('j'),
            Message::Navigation(NavigationCmd::NextCue),
        ),
        (
            KeyCode::Char('l'),
            Message::Navigation(NavigationCmd::RightCol),
        ),
        (KeyCode::Char(' '), Message::Playback(PlaybackCmd::PlayCue)),
        (KeyCode::Char('p'), Message::ModKey(ModKeyCmd::Paste)),
        (
            KeyCode::Char('x'),
            Message::Playback(PlaybackCmd::FadeStopCue),
        ),
        (KeyCode::Char('X'), Message::Playback(PlaybackCmd::StopCue)),
        (KeyCode::Char('q'), Message::Editor(EditorCmd::Quit)),
        (
            KeyCode::Char('i'),
            Message::Editor(EditorCmd::EnterInsertStart),
        ),
        (
            KeyCode::Char('a'),
            Message::Editor(EditorCmd::EnterInsertEnd),
        ),
        (KeyCode::Char('v'), Message::Editor(EditorCmd::EnterVisual)),
        (KeyCode::Char('d'), Message::Combo(ComboCmd::EnterDelete)),
        (KeyCode::Char('y'), Message::Combo(ComboCmd::EnterYank)),
        (KeyCode::Char('c'), Message::Combo(ComboCmd::EnterChange)),
        (KeyCode::Char('o'), Message::Combo(ComboCmd::EnterCreate)),
    ])
}
