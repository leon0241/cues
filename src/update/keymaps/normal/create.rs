use crate::model::{
    cues::{Cue, CueType, cue_base::TargetCue},
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
            cuestack.new_cue(Cue::default());
        }
        // TODO: add these back in
        CreateCmd::CreateFade => {
            cuestack.new_cue(Cue::default_from_type(CueType::Fade(TargetCue::default())));
        }
        _ => {} // CreateCmd::CreateStart => {
                //     cuestack.new_cue(Box::new(StartCue::default()));
                // }
                // CreateCmd::CreateStop => {
                //     cuestack.new_cue(Box::new(StopCue::default()));
                // }
                // CreateCmd::CreatePause => {
                //     cuestack.new_cue(Box::new(PauseCue::default()));
                // }
    }
    editor.set_combo_state(ComboState::None);
    Ok(None)
}
