use crate::model::{
    cue::{AudioCue, Cue, CueType, TargetCue},
    cuestack::CueStack,
    editor::{ComboState, Editor},
    keymaps::{CreateCmd, Message},
};

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn create_handler(
    cuestack: &mut CueStack,
    editor: &mut Editor,
    cmd: &CreateCmd,
) -> color_eyre::Result<Option<Message>> {
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
    Ok(None)
}
