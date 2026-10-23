//! # Copied Cell
//!
//! Clipboard reference handler for copied cells
//! Used so that 'paste' has shared behaviour depending
//! on which type of cell it is

use crate::model::cue::{FollowState, TargetCue};

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
    Time(f32),
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
    pub fn as_str(&self) -> String {
        match self {
            Self::None => String::new(),
            Self::CueNumber(c) | Self::Name(c) | Self::FilePath(c) => c.clone(),
            Self::Follow(c) => c.as_str().to_string(),
            Self::Time(c) => c.to_string(),
            Self::Target(c) => c.target().unwrap_or(String::new()),
        }
    }
}
