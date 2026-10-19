use crate::model::{
    cues::CueType,
    cuestack::CueStack,
    keymaps::{Message, PlaybackCmd},
};

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn playback_handler(
    model: &mut CueStack,
    cmd: &PlaybackCmd,
) -> color_eyre::Result<Option<Message>> {
    match cmd {
        PlaybackCmd::PlayCue => {
            let Some(cue_type) = model.current_type() else {
                return Ok(None);
            };

            match cue_type {
                CueType::Audio(_) => {
                    model.cue_action(None, &PlaybackCmd::PlayCue)?;
                }
                CueType::Fade(_) => {
                    let target: Option<String> = model.current_target();
                    if let Some(i) = target {
                        model.cue_action(Some(i), &PlaybackCmd::FadeStopCue)?;
                    }
                }
                CueType::Start(_) => {
                    let target: Option<String> = model.current_target();
                    if let Some(i) = target {
                        model.cue_action(Some(i), &PlaybackCmd::PlayCue)?;
                    }
                }
                CueType::Stop(_) => {
                    let target: Option<String> = model.current_target();
                    if let Some(i) = target {
                        model.cue_action(Some(i), &PlaybackCmd::StopCue)?;
                    }
                }
                CueType::Pause(_) => {
                    let target: Option<String> = model.current_target();
                    if let Some(i) = target {
                        model.cue_action(Some(i), &PlaybackCmd::PauseCue)?;
                    }
                }
            }
            model.go_next();
        }
        PlaybackCmd::PauseCue => {
            let cue_type = model.current_type();

            if let Some(CueType::Audio(_)) = cue_type {
                model.cue_action(None, &PlaybackCmd::PauseCue)?;
            }
            model.go_next();
        }
        PlaybackCmd::StopCue => {
            let cue_type = model.current_type();

            if let Some(CueType::Audio(_)) = cue_type {
                model.cue_action(None, &PlaybackCmd::StopCue)?;
            }
            model.go_next();
        }
        PlaybackCmd::FadeStopCue => {
            let cue_type = model.current_type();

            if let Some(CueType::Audio(_)) = cue_type {
                model.cue_action(None, &PlaybackCmd::FadeStopCue)?;
            }
            model.go_next();
        }
    }
    Ok(None)
}
