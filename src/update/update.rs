use crate::model::{
    editor::{Mode},
    cuestack::{CueStack},
    cue::{Cue, FollowState, CueType}
};

use std::time::Duration;
use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::model::cue::AudioFile;

#[derive(Debug)]
pub enum Message {
    Quit,
    PrevCue,
    NextCue,
    ModifyCue,
    NewCue,
    DeleteCue,
    PlayCue,
}

pub fn init_cues(model: &mut CueStack) {
    let file1 = AudioFile::new(String::from("funkytown"), String::from("project/audio/test.wav"));

    model.set_items(
        vec![
            Cue::new(1, String::from("test"), CueType::Audio, 1, FollowState::None, None),
            Cue::new(2, String::from("file"), CueType::Audio, 1, FollowState::None, Some(file1)),
            Cue::new(3, String::from("test2"), CueType::Audio, 1, FollowState::None, None),
        ]
    )
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
            model.new_cue(Cue::new(
                    1,
                    String::from("test"),
                    CueType::Audio,
                    1,
                    FollowState::None,
                    None
            ));
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
                    let audiofile = model.get_audio_file()?;
                    if let Some(i) = audiofile {
                        // play_audio(i);
                        model.handler.play_file(i);
                    }
                },
                CueType::Stop => {

                },
                // TODO: the rest of these cases
                _ => {  }
            };
            model.next();
        }
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
        _ => None,
    }
}
