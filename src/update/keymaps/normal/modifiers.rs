use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{model::{cues::audio_cue::AudioCue, cuestack::CueStack}, update::keymaps_file::Message};

#[derive(Debug)]
pub enum ModifierCmd {
    ModifyCue,
    NewCue,
    DeleteCue,
}

const fn modifier_key(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Char('i') => Some(
            Message::Modifier(ModifierCmd::ModifyCue)
        ),
        KeyCode::Char('o') => Some(
            Message::Modifier(ModifierCmd::NewCue)
        ),
        KeyCode::Char('d') => Some(
            Message::Modifier(ModifierCmd::DeleteCue)
        ),
        _ => None,
    }
}

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

