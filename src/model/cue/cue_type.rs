//! # Cue Type
//!
//! Defines all the different cue types
//! Currently can either be an Audio Cue, or
//! a target cue of some sort.

use crate::model::cue::{AudioCue, TargetCue, target_cue::TargetType};

/// Type of Cue
///
/// Separated into Audio cues and Target Cues
/// Target Cues can be fade/stop/start cues for example
#[derive(Debug, Clone)]
pub enum CueType {
    /// Audio Cue with an Audio File
    Audio(AudioCue),
    /// Fade Cue with a Cue Target
    Targetable(TargetCue),
}

impl Default for CueType {
    /// Create a Default Cue
    ///
    /// The default cue is a blank audio cue
    fn default() -> Self {
        Self::Audio(AudioCue::Empty)
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
            Self::Targetable(t) => match t.target_type() {
                TargetType::Fade => "",
                TargetType::Stop => "",
                TargetType::Pause => "",
                TargetType::Start => "",
            },
        }
    }
}
