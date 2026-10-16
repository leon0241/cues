use crate::model::{
    cues::cue_base::CueType,
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
            let cue_type: CueType = model.current_type()?;

            match cue_type {
                CueType::Audio => {
                    model.cue_action(None, &PlaybackCmd::PlayCue)?;
                }
                CueType::Fade => {
                    let target: Option<f32> = model.current_target()?;
                    if let Some(i) = target {
                        model.cue_action(Some(i), &PlaybackCmd::StopCue)?;
                    }
                } // TODO: the rest of these cases
                  // _ => {  }
            }
            model.go_next();
        }
        PlaybackCmd::PauseCue => {
            let cue_type = model.current_type()?;

            if CueType::Audio == cue_type {
                model.cue_action(None, &PlaybackCmd::PauseCue)?;
            }
            model.go_next();
        }
        PlaybackCmd::StopCue => {
            let cue_type = model.current_type()?;

            if CueType::Audio == cue_type {
                model.cue_action(None, &PlaybackCmd::StopCue)?;
            }
            model.go_next();
        }
        PlaybackCmd::FadeStopCue => {
            let cue_type = model.current_type()?;

            if CueType::Audio == cue_type {
                model.cue_action(None, &PlaybackCmd::FadeStopCue)?;
            }
            model.go_next();
        }
    }
    Ok(None)
}
