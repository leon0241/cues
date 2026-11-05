//! # Audio Cue
//!
//! `CueType` structure for audio cues
//!
//! Uses Rodio as a backend

use crate::model::cue::audio_file::AudioFile;

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

/// Playable Cue Specific Functions.
///
/// This can possibly be extended to video in the future?
#[allow(dead_code)]
pub trait PlayableCue {
    /// Plays a Cue
    ///
    /// Plays a cue from the `AudioFile`'s provided file
    /// as bytes, using a Rodio Decoder.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the decoder cannot read the file
    fn play_cue(&self) -> color_eyre::Result<()> {
        Ok(())
    }

    /// Pauses a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(false)` when file is not currently playing
    /// - Returns `Some(true)` when file goes play -> pause
    /// - Returns `Some(false)` when file goes pause -> play
    fn pause_cue(&self) -> Option<bool> {
        None
    }

    /// Stops a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(true)` when file gets stopped
    /// - Returns `Some(false)` when file is not currently playing
    fn stop_cue(&self) -> Option<bool> {
        None
    }

    /// Fade and Stops a cue
    ///
    /// - Returns None if there is no file
    /// - Returns `Some(true)` when file gets stopped
    /// - Returns `Some(false)` when file is not currently playing
    #[allow(unused_variables)]
    fn fade_stop_cue(&self, time: f32) -> Option<bool> {
        None
    }
}
