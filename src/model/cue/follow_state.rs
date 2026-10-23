//! # Follow state
//!
//! Setting and formatting of a cue's follow state

/// Follow State of a Cue
///
/// This can be either:
/// None - does nothing
/// Follow - Next cue starts alongside the current cue
/// Continue - Next cue starts after the current cue finishes.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum FollowState {
    #[default]
    /// None - does nothing
    None,
    /// Follow - Next cue starts alongside the current cue
    Follow,
    /// Continue - Next cue starts after the current cue finishes.
    Continue,
}

impl FollowState {
    /// Get Icon of Follow State
    ///
    /// Returns a `str` matched against the
    /// follow state, for use in rendering in the
    /// "Follow" column of the TUI.
    #[must_use]
    pub const fn icon(self) -> &'static str {
        match self {
            Self::None => " ",
            Self::Follow => "",
            Self::Continue => "",
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Follow => "Follow",
            Self::Continue => "Continue",
        }
    }
}
