use crate::{model::{
    cues::{
        audio_cue::AudioCue, cue_base::{CueType, FollowState}, fade_cue::FadeCue
    }, cuestack::CueStack, editor::Mode
}, update::keymaps::{EditorCmd, ModifierCmd, NavigationCmd, PlaybackCmd}};

use crate::model::cues::audio_file::AudioFile;

use crate::update::keymaps::{Message};

/// # Errors
///
/// Will return `Err` if something went wrong in the files
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
            Box::new(AudioCue::new(1_f32, String::from("test"), 1_f32, FollowState::None, None)),
            Box::new(AudioCue::new(2_f32, String::from("file"), 1_f32, FollowState::None, Some(file1))),
            Box::new(AudioCue::new(2.5_f32, String::from("boom"), 1_f32, FollowState::None, Some(file2))),
            Box::new(FadeCue::new(3_f32, String::from("fade"), 1_f32, FollowState::None, Some(2_f32))),
            Box::new(AudioCue::new(3_f32, String::from("test2"), 1_f32, FollowState::None, None)),
        ]
    );

    Ok(())
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn update(model: &mut CueStack, msg: &Message) -> color_eyre::Result<(Mode, Option<Message>)> {
    // TODO: check all these guys for errors
    match msg {
        Message::Navigation(cmd) => navigation_handler(model, cmd)?,
        Message::Modifier(cmd) => modifier_handler(model, cmd)?,
        Message::Playback(cmd) => playback_handler(model, cmd)?,
        Message::Editor(cmd) => editor_handler(model, cmd)?,
    }
    Ok((Mode::Normal, None))
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn navigation_handler(model: &mut CueStack, cmd: &NavigationCmd) -> color_eyre::Result<()> {
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
    Ok(())
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn modifier_handler(model: &mut CueStack, cmd: &ModifierCmd) -> color_eyre::Result<()> {
    match cmd {
        ModifierCmd::ModifyCue => {
            // model.update_cue();
        }
        ModifierCmd::NewCue => {
            model.new_cue(Box::new(AudioCue::default()));
        }
        ModifierCmd::DeleteCue => {
            model.delete_cue()?;
        }
    }
    Ok(())
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn playback_handler(model: &mut CueStack, cmd: &PlaybackCmd) -> color_eyre::Result<()> {
    match cmd {
        PlaybackCmd::PlayCue => {
            let cue_type: CueType = model.get_current_type()?;

            match cue_type {
                CueType::Audio => {
                    model.current_cue_action(&PlaybackCmd::PlayCue)?;
                },
                CueType::Fade => {
                    let target: Option<f32> = model.get_current_cue_target()?;
                    if let Some(i) = target {
                        model.choose_cue_action(i, &PlaybackCmd::StopCue)?;
                    }

                },
                // TODO: the rest of these cases
                // _ => {  }
            }
            model.go_next()?;
        }
        PlaybackCmd::PauseCue => {
            let cue_type = model.get_current_type()?;

                if CueType::Audio == cue_type {
                    model.current_cue_action(&PlaybackCmd::PauseCue)?;
                }
                model.go_next()?;
        }
        PlaybackCmd::StopCue => {
            let cue_type = model.get_current_type()?;

            if CueType::Audio == cue_type {
                model.current_cue_action(&PlaybackCmd::StopCue)?;
            }
            model.go_next()?;
        }
        PlaybackCmd::FadeStopCue => {
            let cue_type = model.get_current_type()?;

            if CueType::Audio == cue_type {
                model.current_cue_action(&PlaybackCmd::FadeStopCue)?;
            }
            model.go_next()?;
        }
    }
    Ok(())
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub const fn editor_handler(model: &mut CueStack, cmd: &EditorCmd) -> color_eyre::Result<()> {
    match cmd {
        EditorCmd::Quit => {
            model.running_state = crate::model::editor::RunningState::Done;
        }
        EditorCmd::EnterInsertStart => {
            unimplemented!()
        }
        EditorCmd::EnterInsertEnd => {
            unimplemented!()
        }
        EditorCmd::EnterNormal => {
            unimplemented!()
        }
        EditorCmd::EnterVisual => {
            unimplemented!()
        }
    }
    Ok(())
}
