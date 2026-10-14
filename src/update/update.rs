use crate::{model::{
    cues::{
        audio_cue::AudioCue, cue::CueColumn, cue_base::{CueType, FollowState}, fade_cue::FadeCue
    }, cuestack::CueStack, editor::{Editor, Mode}
}, update::keymaps::{CueCmd, EditorCmd, ModKeyCmd, ModifierCmd, NavigationCmd, PlaybackCmd}};

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
pub fn update_normal(model: &mut CueStack, msg: &Message) -> color_eyre::Result<Option<Message>> {
    // TODO: check all these guys for errors
    
    let return_msg: Option<Message> = match msg {
        Message::Navigation(cmd) => {
            navigation_handler(model, cmd)?
        },
        Message::Modifier(cmd) => {
            modifier_handler(model, cmd)?
        },
        Message::Playback(cmd) => {
            playback_handler(model, cmd)?
        },
        Message::Editor(cmd) => {
            editor_handler(model, cmd)?
        }
        Message::ModKey(cmd) => {
            mod_key_handler(model, cmd)?
        }
    };

    return_msg.map_or_else(|| Ok(None), |i| Ok(Some(i)))
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn update_insert(model: &mut CueStack, msg: &Message) -> color_eyre::Result<Option<Message>> {

    let return_msg: Option<Message> = match msg {
       Message::Editor(cmd) => {
           editor_handler(model, cmd)?
       } 
       _ => {None}
    };

    return_msg.map_or_else(|| Ok(None), |i| Ok(Some(i)))
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub const fn update_visual(model: &mut CueStack, msg: &Message) -> color_eyre::Result<(Mode, Option<Message>)> {
    // TODO: check all these guys for errors
    // match msg {
    //     Message::Navigation(cmd) => navigation_handler(model, cmd)?,
    //     Message::Modifier(cmd) => modifier_handler(model, cmd)?,
    //     Message::Playback(cmd) => playback_handler(model, cmd)?,
    //     Message::Editor(cmd) => editor_handler(model, cmd)?,
    // }
    Ok((Mode::Insert, None))
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub const fn update_editor(editor: &mut Editor, msg: &EditorCmd) -> color_eyre::Result<Option<Message>> {
    match msg {
        EditorCmd::Quit => {
            editor.set_running_state(crate::model::editor::RunningState::Done);
        }
        EditorCmd::EnterInsertStart | EditorCmd::EnterInsertEnd => {
            editor.set_mode(Mode::Insert);
        }
        EditorCmd::EnterNormal => {
            editor.set_mode(Mode::Normal);
        }
        EditorCmd::EnterVisual => {
            editor.set_mode(Mode::Visual);
        }
    }
    Ok(None)
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

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn modifier_handler(model: &mut CueStack, cmd: &ModifierCmd) -> color_eyre::Result<Option<Message>> {
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
    Ok(None)
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn playback_handler(model: &mut CueStack, cmd: &PlaybackCmd) -> color_eyre::Result<Option<Message>> {
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
    Ok(None)
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn editor_handler(model: &mut CueStack, cmd: &EditorCmd) -> color_eyre::Result<Option<Message>> {
    match cmd {
        EditorCmd::Quit => {
            Ok(Some(Message::Editor(EditorCmd::Quit)))
        }
        EditorCmd::EnterInsertStart => {
            model.edit_current_cell_start();

            Ok(Some(Message::Editor(EditorCmd::EnterInsertEnd)))
        }
        EditorCmd::EnterInsertEnd => {
            model.edit_current_cell_end();

            Ok(Some(Message::Editor(EditorCmd::EnterInsertEnd)))
        }
        EditorCmd::EnterNormal => {
            model.save_current_cell();

            Ok(Some(Message::Editor(EditorCmd::EnterNormal)))
        }
        EditorCmd::EnterVisual => {
            // model.set_mode(Mode::Visual);
            Ok(Some(Message::Editor(EditorCmd::EnterVisual)))
        }
    }
}

pub fn mod_key_handler(model: &mut CueStack, cmd: &ModKeyCmd) -> color_eyre::Result<Option<Message>> {
    let inner = match cmd {
        x => {
            let a: CueCmd = x.into();
        }
    };

    let cmd2 = match cmd {
        ModKeyCmd::Yank(cue_cmd) => {
            cell_matcher(
                cue_cmd,
                &model.copy_cue,
                &model.copy_cell
            )
        },
        ModKeyCmd::Delete(cue_cmd) => {

        },
        ModKeyCmd::Create(cue_cmd) => {
            
        },
        ModKeyCmd::Change(cue_cmd) => {

        },
    };

    match cmd2 {
        CueCmd::Cue => {
            model.copy_cue();

            Ok(None)
        }
        CueCmd::CueNum => {
            model.copy_cell(CueColumn::Number);
            Ok(None)
        }
        CueCmd::Name => {
            model.copy_cell(CueColumn::Name);
            Ok(None)
        }
        CueCmd::PreWait => {
            model.copy_cell(CueColumn::PreWait);
            Ok(None)
        }
        CueCmd::PostWait => {
            model.copy_cell(CueColumn::PostWait);
            Ok(None)
        }
        CueCmd::Follow => {
            model.copy_cell(CueColumn::Follow);
            Ok(None)
        }
    }
}
