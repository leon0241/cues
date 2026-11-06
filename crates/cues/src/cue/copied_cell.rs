//! # Copied Cell
//!
//! Clipboard reference handler for copied cells
//! Used so that 'paste' has shared behaviour depending
//! on which type of cell it is

use crate::cue::{FollowState, TargetCue, Time};

/// Copyable cell data for use in yanking
///
/// Match out in its methods for shared behaviour
#[derive(Debug, Default, Clone)]
pub enum CopiedCell {
    /// None - Default
    #[default]
    None,
    /// Cue Number - `String`
    CueNumber(String),
    /// Name - `String`
    Name(String),
    /// Follow - `FollowState`
    Follow(FollowState),
    /// Time (Pre-wait/Post-wait) - `f32`
    Time(Time),
    /// Target - `TargetCue`
    Target(TargetCue),
    /// File Path - `String`
    /// Notably This is not a File so that they have separate
    /// players when you copy a file to another cue
    FilePath(String),
}

impl CopiedCell {
    /// Return copied type as string
    ///
    /// Various methods matched against the type of cell it is.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "",
            Self::CueNumber(c) | Self::Name(c) | Self::FilePath(c) => c,
            Self::Follow(c) => c.as_str(),
            Self::Time(c) => c.as_str(),
            Self::Target(c) => c.target().unwrap_or_default(),
        }
    }
}
