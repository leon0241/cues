//! # Normal Mode Create Handler
//!
//! Handles the creation of new cues

use crate::{
    cues::{
        cue::{AudioCue, Cue, CueType, TargetCue},
        cuestack::CueStack,
    },
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
        CreateCmd::CreateTargetable(t) => {
            cuestack.new_cue(Cue::default_from_type(CueType::Targetable(
                TargetCue::default_from_type(*t),
            )));
        }
    }
    editor.set_combo_state(ComboState::None);
    None
}
