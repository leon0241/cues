use crate::model::{
    cues::{
        audio_cue::AudioCue, fade_cue::FadeCue, pause_cue::PauseCue, start_cue::StartCue,
        stop_cue::StopCue,
    },
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
            cuestack.new_cue(Box::new(AudioCue::default()));
        }
        CreateCmd::CreateFade => {
            cuestack.new_cue(Box::new(FadeCue::default()));
        }
        CreateCmd::CreateStart => {
            cuestack.new_cue(Box::new(StartCue::default()));
        }
        CreateCmd::CreateStop => {
            cuestack.new_cue(Box::new(StopCue::default()));
        }
        CreateCmd::CreatePause => {
            cuestack.new_cue(Box::new(PauseCue::default()));
        }
    }
    editor.set_combo_state(ComboState::None);
    Ok(None)
}
