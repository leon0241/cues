//! # Event Handler
//!
//! A magical machine that turns keyboard input into `Message`s

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

/// Init some cues
///
/// This shouldn't be here to be honest
///
/// # Errors
///
/// Will return `Err` if a file couldn't be read
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

/// Polls for events and passes it to `handle_key()` if there is anything notable
///
/// Passes an `Editor` and a `Mode`. Notably this should never touch `CueStack`
///
/// # Errors
///
/// Will return `Err` if something went wrong in `event::read()`
///
/// # Examples
/// ```
/// use cues::model::editor::{Editor, Mode};
/// use cues::update::event_handler::handle_event;
///
/// assert!(matches!(handle_event(&Editor::new(), Mode::Normal), Ok(None)));
/// ```
pub fn handle_event(editor: &Editor, mode: Mode) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))?
        && let event::Event::Key(key) = event::read()?
        && key.kind == event::KeyEventKind::Press
    {
        return Ok(handle_key(key, editor, mode));
    }
    Ok(None)
}

/// Matches against the current `Mode` to the mode's respective handler function.
///
/// Returns None if the handler has completed its function, but can also return a Message, in
/// which case the function could perform a second message and so forth
#[must_use]
pub fn handle_key(key: event::KeyEvent, editor: &Editor, mode: Mode) -> Option<Message> {
    let (code, modifiers) = (key.code, key.modifiers);

    match mode {
        Mode::Normal => handle_normal(code, modifiers, editor),
        Mode::Insert => handle_insert(code, modifiers, editor),
        Mode::Visual => handle_visual(code, modifiers, editor),
    }
}

/// Matches against the current `Message`, and continues matching until the `Message` is `None`
///
/// # Errors
///
/// Returns an Error if either `handle_event()` fails, or any of the update functions propagates up.
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
