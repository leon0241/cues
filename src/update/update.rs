use crate::model::{
    editor::{Mode},
    cuestack::{CueStack},
    cue::{Cue, FollowState}
};

use std::time::Duration;
use ratatui::crossterm::event::{self, Event, KeyCode};

#[derive(Debug)]
pub enum Message {
    Quit,
    PrevCue,
    NextCue,
    ModifyCue,
    NewCue,
    DeleteCue,
}

pub fn update(model: &mut CueStack, msg: Message) -> (Mode, Option<Message>) {
    match msg {
        Message::PrevCue => {
            model.previous();
        }
        Message::NextCue => {
            model.next();
        }
        Message::ModifyCue => {
            model.update_cue();
        }
        Message::NewCue => {
            model.new_cue(Cue::new(1, String::from("test"), 1, FollowState::None));
        }
        Message::DeleteCue => {
            model.delete_cue();
        }
        Message::Quit => {
            model.running_state = crate::model::editor::RunningState::Done
        }
    }

    (Mode::Insert, None)
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
        KeyCode::Char('k') => Some(Message::PrevCue),
        KeyCode::Char('j') => Some(Message::NextCue),
        KeyCode::Char('i') => Some(Message::ModifyCue),
        KeyCode::Char('o') => Some(Message::NewCue),
        KeyCode::Char('d') => Some(Message::DeleteCue),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}
