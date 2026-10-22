//! # Cue Base
//!
//! Methods and Enums for use in cues
//! This does not include the `CueType` Enum, which is
//! provided within [`model/cue.rs`]

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

/// Cue Column
///
/// This is a correspondence to the editable columns in the TUI, and
/// **not** the variables in the struct `Cue`, though many of
/// the enum variants are called the same thing.
///
/// In particular, this is also not all of the columns in the TUI, as
/// fields such as the icon corresponding to the cue type is not editable,
/// and therefore not included in `CueColumn`
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum CueColumn {
    /// Playing - This is populated by the currently playing status.
    Playing,
    /// Number - This is populated by the cue ID, and is not limited to just a number.
    Number,
    /// Name - This is populated by the label of the cue.
    ///
    /// This is also the default value so that `cc` will clear cue into editing the name.
    #[default]
    Name,
    /// Target - This is populated by the cues target. If the cue is an audio cue, it will
    /// instead be populated by an icon
    Target,
    /// Pre-wait - This is populated by the cue's prewait
    PreWait,
    /// Duration - This is populated by the cue's duration. Maybe remove this as it shouldn't
    /// be editable? TODO
    Duration,
    /// Post-wait - This is populated by the cue's postwait
    PostWait,
    /// Follow - This is populated by the cue's follow state (`FollowState`)
    Follow,
}

impl CueColumn {
    /// Gets a cue column from an index
    ///
    /// Column 1 is the cue type, and is not included.
    /// Likewise with any usize larger than 8
    #[must_use]
    pub const fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::Playing),
            // 1 is the cuetype icon which will never be edited
            2 => Some(Self::Number),
            3 => Some(Self::Name),
            4 => Some(Self::Target),
            5 => Some(Self::PreWait),
            6 => Some(Self::Duration),
            7 => Some(Self::PostWait),
            8 => Some(Self::Follow),
            // There are no columns above 8
            1_usize | 9_usize.. => None,
        }
    }

    /// Gets an index from a cue column
    #[must_use]
    pub const fn to_index(column: Self) -> usize {
        match column {
            Self::Playing => 0,
            Self::Number => 2,
            Self::Name => 3,
            Self::Target => 4,
            Self::PreWait => 5,
            Self::Duration => 6,
            Self::PostWait => 7,
            Self::Follow => 8,
        }
    }
}
