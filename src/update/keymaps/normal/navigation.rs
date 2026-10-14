use ratatui::crossterm::event::KeyCode;

use crate::{model::cuestack::CueStack, update::keymaps_file::{Message}};

#[derive(Debug)]
pub enum NavigationCmd {
    LeftCol,
    PrevCue,
    NextCue,
    RightCol,
}

const fn navigation_key(key: KeyCode, _modifier: ratatui::crossterm::event::KeyModifiers) -> Option<Message> {
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

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn navigation_handler(model: &mut CueStack, cmd: &NavigationCmd) -> color_eyre::Result<Option<Message>> {
    match cmd {
        NavigationCmd::LeftCol => {
            model.go_left()?;
        }
        NavigationCmd::PrevCue => {
            model.go_previous()?;
        }
        NavigationCmd::NextCue => {
            model.go_next()?;
        }
        NavigationCmd::RightCol => {
            model.go_right()?;
        }
    }
    Ok(None)
}
