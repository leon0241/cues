//! # Cue Column
//!
//! Used to match columns in the TUI to fields in a cue.

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
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::cue::CueColumn;
    /// let column = CueColumn::from_index(5);
    /// assert_eq!(column, Some(CueColumn::PreWait));
    /// ```
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
    ///
    /// This can be any value 0 -> 8 except 1
    ///
    /// # Examples
    ///
    /// ```
    /// use cues::model::cue::CueColumn;
    /// let idx = CueColumn::to_index(CueColumn::PreWait);
    /// assert_eq!(idx, 5);
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_index() {
        assert_eq!(CueColumn::from_index(0), Some(CueColumn::Playing));
        assert_eq!(CueColumn::from_index(1), None);
        assert_eq!(CueColumn::from_index(2), Some(CueColumn::Number));
        assert_eq!(CueColumn::from_index(3), Some(CueColumn::Name));
        assert_eq!(CueColumn::from_index(4), Some(CueColumn::Target));
        assert_eq!(CueColumn::from_index(5), Some(CueColumn::PreWait));
        assert_eq!(CueColumn::from_index(6), Some(CueColumn::Duration));
        assert_eq!(CueColumn::from_index(7), Some(CueColumn::PostWait));
        assert_eq!(CueColumn::from_index(8), Some(CueColumn::Follow));
        assert_eq!(CueColumn::from_index(9), None);
        assert_eq!(CueColumn::from_index(10), None);
    }

    #[test]
    fn test_to_index() {
        assert_eq!(CueColumn::to_index(CueColumn::Playing), 0);
        assert_eq!(CueColumn::to_index(CueColumn::Number), 2);
        assert_eq!(CueColumn::to_index(CueColumn::Name), 3);
        assert_eq!(CueColumn::to_index(CueColumn::Target), 4);
        assert_eq!(CueColumn::to_index(CueColumn::PreWait), 5);
        assert_eq!(CueColumn::to_index(CueColumn::Duration), 6);
        assert_eq!(CueColumn::to_index(CueColumn::PostWait), 7);
        assert_eq!(CueColumn::to_index(CueColumn::Follow), 8);
    }

    #[test]
    fn test_implementations() {
        let col1 = CueColumn::Name;
        let col2 = CueColumn::PreWait;

        assert_eq!(Some(col1), CueColumn::from_index(3));
        assert_eq!(CueColumn::to_index(col1), 3);
        assert_eq!(Some(col2), CueColumn::from_index(5));
        assert_eq!(CueColumn::to_index(col2), 5);
    }
}
