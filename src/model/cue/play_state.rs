//! # Play State
//!
//! Use and formatting for a cue's currently playing status

/// Currently playing status of a cue
///
/// This is most commonly changed by playback cues
/// such as start/stop cues, and the status can be displayed
/// in the leftmost `Playing` column
#[derive(Debug, Clone, Copy)]
pub enum PlayState {
    /// Used to show a cue is not active
    None,
    /// Used to show a cue is currently playing
    Playing,
    /// Used to show a cue is paused
    Paused,
    /// Used to show a cue is currently fading.
    Fading,
    /// Used to show a cue is stopped. Note that
    /// this is different from None.
    Stopped,
}

impl PlayState {
    /// Get Icon of Play State
    ///
    /// Returns a `str` matched against the
    /// play state, for use in rendering in the
    /// leftmost `Playing` column of the TUI.
    #[must_use]
    pub const fn icon(&self) -> &'static str {
        match self {
            Self::None => " ",
            Self::Playing => "",
            Self::Paused => "",
            Self::Fading => "",
            Self::Stopped => "",
        }
    }
}
