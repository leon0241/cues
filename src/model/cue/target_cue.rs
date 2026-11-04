//! # Audio Cue
//!
//! `CueType` structure for targeted cues.
//! This includes Stops, Starts, Fades, Pauses, etc.

/// # Target Cue
///
/// Currently just has a "cue number" which is what links
/// to an audio cue. Target cues in particular have no "play" functionality,
/// rather they tell the update handler which Audio cue to play.
#[derive(Debug, Default, Clone)]
pub struct TargetCue {
    number: Option<String>,
    // cue_type: CueType, // Don't know if this will ever be useful
}

impl TargetCue {
    /// Creates a new `TargetCue` from a Number. Though it is
    /// called a number, it is actually a string. This lets
    /// you get cues with a number such as `1.1.1` or `H1`
    ///
    /// # Examples
    /// ```
    /// // Target Cue pointing to Cue 1
    /// # use cues::model::cue::target_cue::TargetCue;
    /// let new_cue: TargetCue = TargetCue::new(Some("1"));
    /// ```
    #[must_use]
    pub fn new(number: Option<impl Into<String>>) -> Self {
        let expanded: Option<String> = number.map(std::convert::Into::into);

        Self { number: expanded }
    }

    /// Gets the target cue. Note, this is non-mutable
    ///
    /// # Examples
    /// ```
    /// // Target Cue pointing to Cue 1
    /// # use cues::model::cue::target_cue::TargetCue;
    ///
    /// let new_cue: TargetCue = TargetCue::new(Some("1"));
    /// assert_eq!(new_cue.target().unwrap_or_default(), "1");
    /// ```
    #[must_use]
    pub fn target(&self) -> Option<&str> {
        self.number.as_deref()
    }

    /// Set the target cue from a string.
    ///
    /// # Examples
    /// ```
    /// // Target Cue pointing to Cue 1
    /// # use cues::model::cue::target_cue::TargetCue;
    /// let mut new_cue: TargetCue = TargetCue::new(Some("1"));
    /// new_cue.set_target("2");
    /// assert_eq!(new_cue.target().unwrap_or_default(), "2");
    /// ```
    pub fn set_target(&mut self, value: impl Into<String>) {
        self.number = Some(value.into());
    }
}
