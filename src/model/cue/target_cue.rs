//! # Audio Cue
//!
//! `CueType` structure for targeted cues.
//! This includes Stops, Starts, Fades, Pauses, etc.

use crate::model::keymaps::PlaybackCmd;

/// Type of Target Cue. Used as a field within `TargetCue`.
#[derive(Debug, Default, Clone, Copy)]
pub enum TargetType {
    /// Fade Cue (Default)
    #[default]
    Fade,
    /// Stop Cue
    Stop,
    /// Pause Cue
    Pause,
    /// Start Cue
    Start,
}

impl TargetType {
    /// Maps a Target Type to a Playback CMD
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::TargetType;
    /// use cues::model::keymaps::PlaybackCmd;
    ///
    /// assert!(matches!(TargetType::Fade.action(), PlaybackCmd::FadeStopCue))
    /// ```
    #[must_use]
    pub const fn action(&self) -> PlaybackCmd {
        match self {
            Self::Fade => PlaybackCmd::FadeStopCue,
            Self::Stop => PlaybackCmd::StopCue,
            Self::Pause => PlaybackCmd::PauseCue,
            Self::Start => PlaybackCmd::PlayCue,
        }
    }
}

/// Target Cue
///
/// Contains a Target Type, as well as an optional Target which links to a certain cue's number.
/// Target cues in particular have no "play" functionality,
/// rather they tell the update handler which Audio cue to play.
#[derive(Debug, Default, Clone)]
pub struct TargetCue {
    target_type: TargetType,
    target: Option<String>,
}

impl TargetCue {
    /// Creates a new `TargetCue` from a `TargetType` as well as a target. Use
    /// `default_from_type()` if there is no target.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{TargetCue, TargetType};
    ///
    /// let new_cue = TargetCue::new(TargetType::Fade, "3");
    /// ```
    pub fn new(target_type: TargetType, target: impl Into<String>) -> Self {
        Self {
            target_type,
            target: Some(target.into()),
        }
    }

    /// Creates a new blank `TargetCue` from a `TargetType`. Use `new()` if a target should be
    /// set as well as the type.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{TargetCue, TargetType};
    ///
    /// let new_cue = TargetCue::default_from_type(TargetType::Fade);
    /// ```
    #[must_use]
    pub const fn default_from_type(target_type: TargetType) -> Self {
        Self {
            target_type,
            target: None,
        }
    }

    /// Gets the target cue. Note, this is non-mutable
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::target_cue::{TargetCue, TargetType};
    ///
    /// let new_cue: TargetCue = TargetCue::new(TargetType::Fade, "1");
    /// assert_eq!(new_cue.target().unwrap_or_default(), "1");
    /// ```
    #[must_use]
    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    /// Set the target cue from a string.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::target_cue::{TargetCue, TargetType};
    ///
    /// let mut new_cue: TargetCue = TargetCue::new(TargetType::Fade, "1");
    /// new_cue.set_target("2");
    /// assert_eq!(new_cue.target().unwrap_or_default(), "2");
    /// ```
    pub fn set_target(&mut self, value: impl Into<String>) {
        self.target = Some(value.into());
    }

    /// Get the target type.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::target_cue::{TargetCue, TargetType};
    ///
    /// let mut new_cue: TargetCue = TargetCue::new(TargetType::Fade, "1");
    /// assert!(matches!(new_cue.target_type(), TargetType::Fade));
    /// ```
    #[must_use]
    pub const fn target_type(&self) -> &TargetType {
        &self.target_type
    }
}
