//! # Audio Cue
//!
//! `CueType` structure for audio cues
//!
//! Uses Rodio as a backend

use cue_audio::audio_file::AudioFile;

/// Audio Cue
///
/// Can either be Empty or Populated with an audio file.
///
/// Note that this implementation is different from target cues as they always need to have the
/// type set from the beginning.
#[derive(Debug, Default, Clone)]
pub enum AudioCue {
    /// Empty Audio Cue
    #[default]
    Empty,
    /// Populated with an audio file
    Populated(AudioFile),
}

impl AudioCue {
    /// Returns a file's icon.
    ///
    /// This is solely for use in the Target field, where
    /// a populated audio file will have the "" icon while
    /// an unpopulated one will be blank.
    #[must_use]
    pub const fn as_icon(&self) -> &'static str {
        match self {
            Self::Empty => "",
            Self::Populated(_) => "",
        }
    }
}
