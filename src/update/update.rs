use crate::model::{
    editor::{Mode},
    cuestack::{CueStack},
    cues::{
        cue::{Cue, FollowState, CueType},
        audio_cue::{AudioCue}
    }
};

use std::time::Duration;
use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::model::cues::audio_file::AudioFile;

#[derive(Debug)]
pub enum Message {
    Quit,
    PrevCue,
    NextCue,
    ModifyCue,
    NewCue,
    DeleteCue,
    PlayCue,
    PauseCue,
    StopCue,
    FadeStopCue,
}

pub fn init_cues(model: &mut CueStack) -> color_eyre::Result<()> {
    let file1: AudioFile = AudioFile::new(
        String::from("funkytown"),
        String::from("project/audio/test.wav"),
        &model.handler
    )?;
    let file2: AudioFile = AudioFile::new(
        String::from("boom"),
        String::from("project/audio/boom.wav"),
        &model.handler
    )?;

    model.set_items(
        vec![
            Box::new(AudioCue::new(1, String::from("test"), 1, FollowState::None, None)),
            Box::new(AudioCue::new(2, String::from("file"), 1, FollowState::None, Some(file1))),
            Box::new(AudioCue::new(2, String::from("boom"), 1, FollowState::None, Some(file2))),
            Box::new(AudioCue::new(3, String::from("test2"), 1, FollowState::None, None)),
        ]
    );

    Ok(())
}

pub fn update(model: &mut CueStack, msg: Message) -> color_eyre::Result<(Mode, Option<Message>)> {
    // TODO: check all these guys for errors
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
            model.new_cue(Box::new(AudioCue::new(
                    1,
                    String::from("test"),
                    1,
                    FollowState::None,
                    None
            )));
        }
        Message::DeleteCue => {
            model.delete_cue()?;
        }
        Message::Quit => {
            model.running_state = crate::model::editor::RunningState::Done
        }

        Message::PlayCue => {
            let cue_type = model.get_current_type()?;

            match cue_type {
                CueType::Audio => {
                    model.current_cue_action(Message::PlayCue)?;
                },
                CueType::Stop => {

                },
                // TODO: the rest of these cases
                _ => {  }
            };
            model.next();
        }
        Message::PauseCue => {
            let cue_type = model.get_current_type()?;

            if let CueType::Audio = cue_type {
                model.current_cue_action(Message::PauseCue)?;
            };
            model.next();
        }
        Message::StopCue => {
            let cue_type = model.get_current_type()?;

            if let CueType::Audio = cue_type {
                model.current_cue_action(Message::StopCue)?;
            };
            model.next();
        }
        Message::FadeStopCue => {
            let cue_type = model.get_current_type()?;

            if let CueType::Audio = cue_type {
                model.current_cue_action(Message::FadeStopCue)?;
            };
            model.next();
        }
        _ => { }
    }

    Ok((Mode::Insert, None))
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
        KeyCode::Char(' ') => Some(Message::PlayCue),
        KeyCode::Char('p') => Some(Message::PauseCue),
        KeyCode::Char('X') => Some(Message::StopCue),
        KeyCode::Char('x') => Some(Message::FadeStopCue),
        _ => None,
    }
}
