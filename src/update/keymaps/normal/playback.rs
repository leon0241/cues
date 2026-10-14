use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{model::{cues::cue_base::CueType, cuestack::CueStack}, update::keymaps_file::Message};

#[derive(Debug)]
pub enum PlaybackCmd {
    PlayCue,
    PauseCue,
    StopCue,
    FadeStopCue,
}

const fn playback_key(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Char(' ') => Some(
            Message::Playback(PlaybackCmd::PlayCue)
        ),
        KeyCode::Char('p') => Some(
            Message::Playback(PlaybackCmd::PauseCue)
        ),
        KeyCode::Char('x') => Some(
            Message::Playback(PlaybackCmd::FadeStopCue)
        ),
        KeyCode::Char('X') => Some(
            Message::Playback(PlaybackCmd::StopCue)
        ),
        _ => None,
    }
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

