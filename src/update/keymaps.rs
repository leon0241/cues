use std::time::Duration;

use ratatui::crossterm::event::{self, KeyCode, KeyModifiers};

use crate::model::{cuestack::CueStack, editor::{ComboState, Editor, Mode}};

#[derive(Debug)]
pub enum Message {
    Navigation(NavigationCmd),
    Modifier(ModifierCmd),
    Playback(PlaybackCmd),
    Editor(EditorCmd),
    ModKey(ModKeyCmd),
}

#[derive(Debug)]

pub enum ModKeyCmd {
    Yank(CueCmd),
    Delete(CueCmd),
    Create(CueCmd),
    Change(CueCmd),
}

#[derive(Debug)]
pub enum NavigationCmd {
    LeftCol,
    PrevCue,
    NextCue,
    RightCol,
}

#[derive(Debug)]
pub enum ModifierCmd {
    ModifyCue,
    NewCue,
    DeleteCue,
}

#[derive(Debug)]
pub enum PlaybackCmd {
    PlayCue,
    PauseCue,
    StopCue,
    FadeStopCue,
}

#[derive(Debug)]
pub enum EditorCmd {
    Quit,
    EnterInsertStart,
    EnterInsertEnd,
    EnterNormal,
    EnterVisual,
}

#[derive(Debug)]
/// Enum for all Cue column editing
pub enum CueCmd {
    Cue,
    CueNum,
    Name,
    PreWait,
    PostWait,
    Follow
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn handle_event(editor: &Editor, mode: Mode) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let event::Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(key, mode, editor));
    }
    Ok(None)
}

#[must_use]
pub const fn handle_key(key: event::KeyEvent, mode: Mode, editor: &Editor) -> Option<Message> {
    let (code, modifiers) = (key.code, key.modifiers);

    match mode {
        Mode::Normal => {
            handle_normal(code, modifiers, editor.combo_state())
        }
        Mode::Insert => {
            handle_insert(code, modifiers)
        }
        Mode::Visual => {
            handle_visual(code, modifiers, editor.combo_state())
        }
    }
}

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

/// Handles double key modifiers such as Yank, Delete
const fn mod_key(key: KeyCode, _modifier: KeyModifiers, combo_state: ComboState) -> Option<Message> {
    // Match the key press
    let cue_type: Option<CueCmd> = match key {
        KeyCode::Char('q') => Some(CueCmd::CueNum),
        KeyCode::Char('n') => Some(CueCmd::Name),
        KeyCode::Char('f') => Some(CueCmd::PreWait),
        KeyCode::Char('F') => Some(CueCmd::PostWait),
        KeyCode::Char('w') => Some(CueCmd::Follow),
        // Double presses e.g. yy => yank line
        _ => {
            match (key, combo_state) {
                (KeyCode::Char('y'), ComboState::Yank) => Some(CueCmd::Cue),
                (KeyCode::Char('d'), ComboState::Delete) => Some(CueCmd::Cue),
                (KeyCode::Char('o'), ComboState::Create) => Some(CueCmd::Cue),
                (KeyCode::Char('c'), ComboState::Change) => Some(CueCmd::Cue),
                _ => None
            }
        },
    };

    // If there was a compatible command, map to the associated command
    if let Some(i) = cue_type {
        let mod_key = match combo_state {
            ComboState::Yank => Some(ModKeyCmd::Yank(i)),
            ComboState::Delete => Some(ModKeyCmd::Delete(i)),
            ComboState::Change => Some(ModKeyCmd::Change(i)),
            ComboState::Create => Some(ModKeyCmd::Create(i)),
            ComboState::None => None
        };
        if let Some(i) = mod_key {
            return Some(Message::ModKey(i))
        }
    }
    // Else return none and reset
    None
}

const fn normal_mode_switch(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Char('i') => Some(
            Message::Editor(EditorCmd::EnterInsertStart)
        ),
        KeyCode::Char('a') => Some(
            Message::Editor(EditorCmd::EnterInsertEnd)
        ),
        KeyCode::Char('v') => Some(
            Message::Editor(EditorCmd::EnterVisual)
        ),
        _ => None
    }
}

const fn navigation_key(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Char('h') => Some(
            Message::Navigation(NavigationCmd::LeftCol)
        ),
        KeyCode::Char('k') => Some(
            Message::Navigation(NavigationCmd::PrevCue)
        ),
        KeyCode::Char('j') => Some(
            Message::Navigation(NavigationCmd::NextCue)
        ),
        KeyCode::Char('l') => Some(
            Message::Navigation(NavigationCmd::RightCol)
        ),
        _ => None,
    }
}

const fn modifier_key(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Char('i') => Some(
            Message::Modifier(ModifierCmd::ModifyCue)
        ),
        KeyCode::Char('o') => Some(
            Message::Modifier(ModifierCmd::NewCue)
        ),
        KeyCode::Char('d') => Some(
            Message::Modifier(ModifierCmd::DeleteCue)
        ),
        _ => None,
    }
}

const fn playback_key(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Char(' ') => Some(
            Message::Playback(PlaybackCmd::PlayCue)
        ),
        KeyCode::Char('p') => Some(
            Message::Playback(PlaybackCmd::PauseCue)
        ),
        KeyCode::Char('x') => Some(
            Message::Playback(PlaybackCmd::FadeStopCue)
        ),
        KeyCode::Char('X') => Some(
            Message::Playback(PlaybackCmd::StopCue)
        ),
        _ => None,
    }
}

const fn editor_key(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Char('q') => Some(
            Message::Editor(EditorCmd::Quit)
        ),
        _ => None,
    }
}

const fn handle_insert(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Esc => Some(
            Message::Editor(EditorCmd::EnterNormal)
        ),
        KeyCode::Char('q') => Some(
            Message::Editor(EditorCmd::Quit)
        ),
        // KeyCode::Char('j') => Some(
        //     Message::Navigation(NavigationCmd::NextCue)
        // ),
        _ => None,
    }
}

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
