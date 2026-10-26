//! # Cuestack
//!
//! Handles a collection of `Cue`s. The Ratatui `TableState` is also handled here as a way to get
//! cursor positions.

pub mod modifiers;
pub mod navigation;
pub mod playback;

use ratatui::widgets::TableState;

use crate::model::{
    cue::{Cue, CueType},
    keymaps::PlaybackCmd,
};

use crate::model::audio_config::AudioDevice;

/// Cuestack
///
/// This stores a `Vec` of `Cue`s which act as the main cuestack. Any logic that happens to a
/// `Cue` goes through `Cuestack` first, and then it will pick a cue to perform that action onto.
#[derive(Default)]
pub struct CueStack {
    /// The main Cuestack, a `Vec` of `Cue`s
    pub cues: Vec<Cue>,
    /// The ratatui `TableState` which also acts as a row/col identifier to locate the cell the
    /// cursor is currently on
    pub current_state: TableState,
    /// Rodio `AudioDevice` which acts as the main player for any cues getting played
    pub handler: AudioDevice,
}

impl CueStack {
    /// Creates a new `CueStack`
    ///
    /// This gets initialised with no cues in the stack.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let cuestack = CueStack::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the default sink for cues in this cuestack to play from.
    ///
    /// # Errors
    ///
    /// Will return `Err` if cannot find a default sink
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// cuestack.set_default_sink();
    /// ```
    pub fn set_default_sink(&mut self) -> color_eyre::Result<()> {
        self.handler.set_default_sink()
    }

    /// Creates a pre-made cuestack by manually passing in `Vec` of `Cue`s.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::cue::{Cue, FollowState, CueType, AudioCue};
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.set_items(vec![
    ///     Cue::new("1", "",1.0, FollowState::None,
    ///         CueType::Audio(AudioCue::default()),
    ///     )
    /// ]);
    /// ```
    pub fn set_items(&mut self, items: Vec<Cue>) {
        self.cues = items;
        // We reset the state as the associated items have changed. This effectively reset
        // the selection as well as the stored offset.
        self.current_state = TableState::default();
    }

    /// Gets the current Cue Type
    ///
    /// Will return `None` if no cue is selected
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// assert!(matches!(cuestack.current_type(), None));
    /// ```
    #[must_use]
    pub fn current_type(&self) -> Option<&CueType> {
        let current_cue: &Cue = self.get_current_cue()?.0;

        Some(current_cue.cue_type())
    }

    /// Gets the current Cue Target
    ///
    /// Will return `None` if no cue is selected
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    ///
    /// let mut cuestack = CueStack::default();
    /// assert!(matches!(cuestack.current_target(), None));
    /// ```
    #[must_use]
    pub fn current_target(&self) -> Option<String> {
        let current_cue: &Cue = self.get_current_cue()?.0;

        Some(current_cue.get_target_str())
    }

    /// Performs an action on a chosen cue.
    ///
    /// If `target` is a string, it performs the action on the specified target.
    ///
    /// If `target` is none, it is assumed that the action will be done to the current cue.
    /// If no cue is selected, the function does nothing.
    /// For example, when pressing `Space` to trigger an audio cue.
    ///
    /// # Errors
    ///
    /// Will return `Err` there are too many targets
    ///
    /// # Examples
    /// ```
    /// use cues::model::cuestack::CueStack;
    /// use cues::model::keymaps::PlaybackCmd;
    /// use cues::model::cue::{Cue, FollowState, CueType, AudioCue};
    ///
    /// let mut cuestack = CueStack::default();
    ///
    /// cuestack.set_items(vec![
    ///     Cue::new("1", "",1.0, FollowState::None,
    ///         CueType::Audio(AudioCue::default()),
    ///     )
    /// ]);
    ///
    /// cuestack.cue_action(Some("1"), &PlaybackCmd::PlayCue);
    /// ```
    pub fn cue_action(
        &mut self,
        target: Option<&str>,
        selection: &PlaybackCmd,
    ) -> color_eyre::Result<()> {
        let cue_num: Option<usize> = match target {
            None => self.current_state.selected(),

            Some(t) => Some(self.query_target(t)),
        };

        // If there is a number then play else just return
        if let Some(i) = cue_num {
            self.playback_action(i, *selection)?;
        }
        Ok(())
    }

    /// Tries to find a target in a cuestack
    ///
    /// If there are zero/over 1 target it returns zero otherwise it returns the target
    /// cue number. Maybe this should be different..
    fn query_target(&self, target: &str) -> usize {
        // Get all matches for a cue target
        let mut matches = self
            .cues
            .iter()
            .enumerate()
            .filter(|x| x.1.number() == target)
            .map(|(i, _)| i);

        // if there are matches or net
        match (matches.next(), matches.next()) {
            (None, _) => {
                // zero matches
                0
            }
            (Some(i), None) => {
                // exactly one match (ideal)
                i
            }
            (Some(_f), Some(_s)) => {
                // two or more matches (first and second are the first two indices)
                0
            }
        }
    }
}
