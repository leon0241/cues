use std::time::Duration;

use ratatui::crossterm::event;

use crate::{
    model::{
        cue::{AudioCue, Cue, CueType, FollowState, audio_file::AudioFile},
        cuestack::CueStack,
        editor::{Editor, Mode},
        full_model::Model,
        keymaps::Message,
    },
    update::keymaps::{
        insert::{handle_insert, update_insert},
        normal::{handle_normal, update_normal},
        visual::{handle_visual, update_visual},
    },
};

/// # Errors
///
/// Will return `Err` if something went wrong in the files
pub fn init_cues(model: &mut CueStack) -> color_eyre::Result<()> {
    let file1: AudioFile = AudioFile::new(
        String::from("funkytown"),
        String::from("project/audio/test.wav"),
        &model.handler,
    )?;
    let _file2: AudioFile = AudioFile::new(
        String::from("boom"),
        String::from("project/audio/boom.wav"),
        &model.handler,
    )?;

    model.set_items(vec![
        Cue::new(
            "1",
            "",
            1_f32,
            FollowState::None,
            CueType::Audio(AudioCue::default()),
        ),
        Cue::new(
            "2",
            "file",
            1_f32,
            FollowState::None,
            CueType::Audio(AudioCue::new(Some(file1))),
        ),
        // Cue::new(
        //     String::from("2.5"),
        //     String::from("boom"),
        //     1_f32,
        //     FollowState::None,
        //     CueType::Audio(AudioCue::new(Some(file2))),
        // ),
        // Cue::new(
        //     String::from("3"),
        //     String::from("fade"),
        //     1_f32,
        //     FollowState::None,
        //     CueType::Fade(TargetCue::new(Some(String::from("2")))),
        // ),
        // Cue::new(
        //     String::from("4"),
        //     String::from("test2"),
        //     1_f32,
        //     FollowState::None,
        //     CueType::Audio(AudioCue::default()),
        // ),
    ]);

    Ok(())
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
pub fn handle_key(key: event::KeyEvent, mode: Mode, editor: &Editor) -> Option<Message> {
    let (code, modifiers) = (key.code, key.modifiers);

    match mode {
        Mode::Normal => handle_normal(code, modifiers, editor),
        Mode::Insert => handle_insert(code, modifiers, editor),
        Mode::Visual => handle_visual(code, modifiers, editor),
    }
}

/// # Errors
///
/// TODO
pub fn event_matcher(model: &mut Model) -> color_eyre::Result<()> {
    // Start on Normal mode (default)
    let current_mode: Mode = model.editor().mode();
    // Handle events and map to a Message
    let mut current_msg: Option<Message> = handle_event(model.editor(), current_mode)?;

    while let Some(ref message) = current_msg {
        match current_mode {
            Mode::Normal => {
                current_msg = update_normal(model, message)?;
            }
            Mode::Insert => {
                current_msg = update_insert(model, message)?;
            }
            Mode::Visual => {
                current_msg = update_visual(model, message)?;
            }
        }
    }

    Ok(())
}
