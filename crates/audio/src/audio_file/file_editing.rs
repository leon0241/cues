//! # Audio File - Editing
//!
//! Methods for editing an audio file. Note that none of these methods will work for a Rodio Player
//! which is already generated.

use std::time::Duration;

use crate::audio_file::AudioFile;

/// Audio File Settings
///
/// Set the bounds and the volume and possibly other options later on such as matrix outputs
#[derive(Clone, Default)]
pub struct AudioFileSettings {
    /// Start time of the cue as a Duration
    start_time: Duration,
    /// End time of the cue as a Duration
    end_time: Duration,
    /// Volume of the track as an f32 between [0, 1]
    volume: f32,
}

impl AudioFile {
    // fn set_volume(&self, vol: f32) {
    //     self.settings.start_time
    // }
}
