use crate::model::{cues::audio_cue::AudioCue, cuestack::CueStack, keymaps::{Message, ModifierCmd}};

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

