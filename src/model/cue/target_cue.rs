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
    /// let string: String = String::from("1");
    /// let new_cue: TargetCue = TargetCue::new(Some(string));
    /// ```
    #[must_use]
    pub const fn new(number: Option<String>) -> Self {
        Self { number }
    }

    /// Gets the target cue. Note, this is non-mutable
    ///
    /// # Examples
    /// ```
    /// // Target Cue pointing to Cue 1
    /// # use cues::model::cue::target_cue::TargetCue;
    /// let string: String = String::from("1");
    /// let new_cue: TargetCue = TargetCue::new(Some(string));
    /// assert_eq!(new_cue.target().unwrap_or_default(), String::from("1"));
    /// ```
    #[must_use]
    pub fn target(&self) -> Option<String> {
        self.number.clone()
    }

    /// Set the target cue from a string.
    ///
    /// # Examples
    /// ```
    /// // Target Cue pointing to Cue 1
    /// # use cues::model::cue::target_cue::TargetCue;
    /// let string: String = String::from("1");
    /// let mut new_cue: TargetCue = TargetCue::new(Some(string));
    /// new_cue.set_target(String::from("2"));
    /// assert_eq!(new_cue.target().unwrap_or_default(), String::from("2"));
    /// ```
    pub fn set_target(&mut self, value: String) {
        self.number = Some(value);
    }
}
