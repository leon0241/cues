//! # Cue Type
//!
//! Defines all the different cue types
//! Currently can either be an Audio Cue, or
//! a target cue of some sort.

use crate::model::cue::{AudioCue, TargetCue};

/// Type of Cue
///
/// Separated into Audio cues and Target Cues
/// Target Cues can be fade/stop/start cues for example
#[derive(Debug, Clone)]
pub enum CueType {
    /// Audio Cue with an Audio File
    Audio(AudioCue),
    /// Fade Cue with a Cue Target
    Fade(TargetCue),
    /// Stop Cue with a Cue Target
    Stop(TargetCue),
    /// Pause Cue with a Cue Target
    Pause(TargetCue),
    /// Start Cue with a Cue Target
    Start(TargetCue),
}

impl Default for CueType {
    /// Create a Default Cue
    ///
    /// The default cue is a blank audio cue
    fn default() -> Self {
        Self::Audio(AudioCue::default())
    }
}

impl CueType {
    /// Get string representation of icon
    ///
    /// Returns a string icon depending on what the cue type is
    #[must_use]
    pub const fn icon(&self) -> &'static str {
        match self {
            Self::Audio(_) => "",
            Self::Fade(_) => "",
            Self::Stop(_) => "",
            Self::Pause(_) => "",
            Self::Start(_) => "",
        }
    }
}
