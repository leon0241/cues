//! # Normal Mode Create Handler
//!
//! Handles the creation of new cues

use crate::model::{
    cue::{AudioCue, Cue, CueType, TargetCue},
    cuestack::CueStack,
    editor::{ComboState, Editor},
    keymaps::{CreateCmd, Message},
};

/// Creates cues matched to a specific message
///
/// Should always return None
pub fn create_handler(
    cuestack: &mut CueStack,
    editor: &mut Editor,
    cmd: &CreateCmd,
) -> Option<Message> {
    match cmd {
        CreateCmd::CreateAudio => {
            cuestack.new_cue(Cue::default_from_type(CueType::Audio(AudioCue::default())));
        }
        // TODO: add these back in
        CreateCmd::CreateFade => {
            cuestack.new_cue(Cue::default_from_type(CueType::Fade(TargetCue::default())));
        }
        CreateCmd::CreateStart => {
            cuestack.new_cue(Cue::default_from_type(CueType::Start(TargetCue::default())));
        }
        CreateCmd::CreateStop => {
            cuestack.new_cue(Cue::default_from_type(CueType::Stop(TargetCue::default())));
        }
        CreateCmd::CreatePause => {
            cuestack.new_cue(Cue::default_from_type(CueType::Pause(TargetCue::default())));
        }
    }
    editor.set_combo_state(ComboState::None);
    None
}
