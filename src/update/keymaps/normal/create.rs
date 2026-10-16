use crate::model::{
    cues::{
        audio_cue::AudioCue, fade_cue::FadeCue, pause_cue::PauseCue, start_cue::StartCue,
        stop_cue::StopCue,
    },
    cuestack::CueStack,
    keymaps::{CreateCmd, Message},
};

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn modifier_handler(
    model: &mut CueStack,
    cmd: &CreateCmd,
) -> color_eyre::Result<Option<Message>> {
    match cmd {
        CreateCmd::CreateAudio => {
            model.new_cue(Box::new(AudioCue::default()));
        }
        CreateCmd::CreateFade => {
            model.new_cue(Box::new(FadeCue::default()));
        }
        CreateCmd::CreateStart => {
            model.new_cue(Box::new(StartCue::default()));
        }
        CreateCmd::CreateStop => {
            model.new_cue(Box::new(StopCue::default()));
        }
        CreateCmd::CreatePause => {
            model.new_cue(Box::new(PauseCue::default()));
        }
    }
    Ok(None)
}
