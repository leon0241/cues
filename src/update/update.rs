use crate::model::model::CueStack;

use std::time::Duration;
use ratatui::crossterm::event::{self, Event, KeyCode};

pub enum Message {
    ModifyCue,
    NewCue,
    DeleteCue
}

pub fn update(model: &CueStack, msg: Message) -> Option<Message> {
    match msg {
        Message::ModifyCue => {
            model.update_cue();
            return None
        }
        Message::NewCue => {
            model.new_cue();
            return None
        }
        Message::DeleteCue => {
            model.delete_cue();
            return None;
        }
    };
}

pub fn handle_event(_: &CueStack) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(key));
    }
    Ok(None)
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('i') => Some(Message::ModifyCue),
        KeyCode::Char('o') => Some(Message::NewCue),
        KeyCode::Char('d') => Some(Message::DeleteCue),
        _ => None,
    }
}
