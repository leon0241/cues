//! # Cue
//!
//! Handler for individual cues of any type.
//! Individual Cue types are split out with an Enum called ``CueType``

pub mod audio_cue;
pub mod audio_file;
pub mod copied_cell;
pub mod cue_column;
pub mod cue_type;
pub mod follow_state;
pub mod play_state;
pub mod target_cue;

// Expose Submodules
pub use crate::model::cue::{
    audio_cue::AudioCue, copied_cell::CopiedCell, cue_column::CueColumn, cue_type::CueType,
    follow_state::FollowState, play_state::PlayState, target_cue::TargetCue,
};

use ratatui::widgets::Row;

/// Cue
///
/// This is an individual cue in a cuestack
#[derive(Debug, Clone)]
pub struct Cue {
    #[allow(clippy::struct_field_names)]
    cue_type: CueType,
    number: String,
    name: String,
    follow: FollowState,
    prewait: f32,
    duration: f32,
    postwait: f32,
    status: PlayState,
    // parent: Option<TargetCue>,
}

impl Default for Cue {
    fn default() -> Self {
        Self::new(
            String::new(),
            String::new(),
            0_f32,
            FollowState::None,
            CueType::Audio(AudioCue::default()),
        )
    }
}

/// Cue struct
/// Implement for all cues - cue types are differentiated
/// within `cue_type: CueType`
impl Cue {
    /// Creates a new cue with specified parameters
    ///
    /// Cue alternatively implements Default, as well as
    /// a `default_from_type()` function as Default will always
    /// create an audio cue.
    #[must_use]
    pub const fn new(
        number: String,
        name: String,
        duration: f32,
        follow: FollowState,
        cue_type: CueType,
    ) -> Self {
        Self {
            number,
            name,
            duration,
            follow,
            cue_type,
            prewait: 0_f32,
            postwait: 0_f32,
            status: PlayState::None,
            // parent: None,
        }
    }

    /// Creates a specified blank cue type
    ///
    /// Cue alternatively implements Default, which will
    /// create an audio file
    #[must_use]
    pub fn default_from_type(cue_type: CueType) -> Self {
        Self {
            cue_type,
            ..Self::default()
        }
    }

    /// Gets a Ratatui format `Row`
    ///
    /// All the fields in a `Row` are of type String
    #[must_use]
    pub fn get_row(&self) -> Row<'_> {
        Row::new(vec![
            // Currently playing status icon
            self.status.icon().to_string(),
            // Cue Type Icon
            self.cue_type.icon().to_string(),
            // Cue Number
            self.number.clone(),
            // Cue Name
            self.name.clone(),
            // Cue Target
            self.get_target_str(),
            // Cue Prewait
            self.prewait.to_string(),
            // Cue Duration
            self.duration.to_string(),
            // Cue Postwait
            self.postwait.to_string(),
            // Cue Follow State Icon
            self.follow.icon().to_string(),
        ])
    }

    /// Gets the Cue target
    ///
    /// This will either be an icon of a file for an Audio File
    /// or a target to another cue for any targetable cue types.
    #[must_use]
    pub fn get_target(&self) -> Option<String> {
        match self.cue_type {
            CueType::Audio(ref audio_cue) => audio_cue
                .file()
                .as_ref()
                .map_or(None, |f| Some(f.file_path().to_string())),
            CueType::Fade(ref t)
            | CueType::Stop(ref t)
            | CueType::Pause(ref t)
            | CueType::Start(ref t) => t.target().map_or(None, |f| Some(f)),
        }
    }

    /// Gets the Cue target
    ///
    /// This will either be an icon of a file for an Audio File
    /// or a target to another cue for any targetable cue types.
    #[must_use]
    pub fn get_target_str(&self) -> String {
        match self.cue_type {
            CueType::Audio(ref audio_cue) => audio_cue.as_icon().to_string(),
            CueType::Fade(ref t)
            | CueType::Stop(ref t)
            | CueType::Pause(ref t)
            | CueType::Start(ref t) => t.target().unwrap_or_else(String::new),
        }
    }

    /// Sets the Cue target
    ///
    /// This function only works for Targetable cues, setting
    /// the file path for an audio cue is set through `cue.file().target()`
    pub fn set_target(&mut self, value: String) {
        match self.cue_type {
            CueType::Audio(_) => {}
            CueType::Fade(ref mut t)
            | CueType::Stop(ref mut t)
            | CueType::Pause(ref mut t)
            | CueType::Start(ref mut t) => t.set_target(value),
        }
    }
}

impl Cue {
    /// Gets the Cue type reference (immutable)
    ///
    /// This returns a non-mutable type, use `cue_type_mut()` for a mutable reference.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, CueType, TargetCue};
    ///
    /// let cue = Cue::default_from_type(CueType::Fade(TargetCue::default()));
    ///
    /// assert!(matches!(cue.cue_type(), CueType::Fade(_)));
    /// ```
    #[must_use]
    pub const fn cue_type(&self) -> &CueType {
        &self.cue_type
    }

    /// Gets the Cue type reference (mutable)
    ///
    /// This returns a mutable type, use `cue_type()` for an immutable reference.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, CueType, TargetCue};
    ///
    /// let cue = Cue::default_from_type(CueType::Fade(TargetCue::default()));
    ///
    /// assert!(matches!(cue.cue_type(), CueType::Fade(_)));
    /// ```
    pub const fn cue_type_mut(&mut self) -> &mut CueType {
        &mut self.cue_type
    }

    /// Gets the name of a cue
    ///
    /// This is the label of the cue, and returns a string.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let cue = Cue::default();
    ///
    /// assert_eq!(cue.name(), String::new());
    /// ```
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Sets the name of a cue
    ///
    /// This is the label of the cue, and takes a string.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let mut cue = Cue::default();
    /// cue.set_name(String::from("label"));
    /// ```
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Gets the cue number of a cue
    ///
    /// This is the ID of the cue, and returns a string, even if most
    /// cue numbers will be some form of number.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let cue = Cue::default();
    ///
    /// assert_eq!(cue.number(), String::new());
    /// ```
    #[must_use]
    pub fn number(&self) -> String {
        self.number.clone()
    }

    /// Sets the cue number of a cue
    ///
    /// This is the ID of the cue, and takes a string, even if most
    /// cue numbers will be some form of number.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let mut cue = Cue::default();
    ///
    /// cue.set_number(String::from("3"));
    /// ```
    pub fn set_number(&mut self, number: String) {
        self.number = number;
    }

    /// Gets the playing status of a cue
    ///
    /// This will be of type `PlayState`, and is mainly read to render icons for currently.
    /// playing cues. The default state is `PlayState::None`.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, PlayState};
    ///
    /// let mut cue = Cue::default();
    ///
    /// assert!(matches!(cue.status(), PlayState::None));
    /// ```
    #[must_use]
    pub const fn status(&self) -> PlayState {
        self.status
    }

    /// Sets the playing status of a cue
    ///
    /// This will be of type `PlayState`, though this should never be manually changed. only
    /// through hooks in other functions such as `play_cue()`.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, PlayState};
    ///
    /// let mut cue = Cue::default();
    ///
    /// cue.set_status(PlayState::Playing);
    /// ```
    pub const fn set_status(&mut self, status: PlayState) {
        self.status = status;
    }

    /// Gets the post wait of a cue
    ///
    /// This will be an f32 though it is a time so maybe later on I will change this.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let cue = Cue::default();
    ///
    /// assert_eq!(cue.prewait(), 0.0);
    /// ```
    #[must_use]
    pub const fn prewait(&self) -> f32 {
        self.prewait
    }

    /// Sets the prewait of a cue
    ///
    /// This will be an f32 though it is a time so maybe later on I will change this.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let mut cue = Cue::default();
    ///
    /// cue.set_prewait(0.2);
    /// ```
    pub const fn set_prewait(&mut self, prewait: f32) {
        self.prewait = prewait;
    }

    /// Gets the post wait of a cue
    ///
    /// This will be an f32 though it is a time so maybe later on I will change this.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let cue = Cue::default();
    ///
    /// assert_eq!(cue.postwait(), 0.0);
    /// ```
    #[must_use]
    pub const fn postwait(&self) -> f32 {
        self.postwait
    }

    /// Sets the post wait of a cue
    ///
    /// This will be an f32 though it is a time so maybe later on I will change this.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let mut cue = Cue::default();
    ///
    /// cue.set_postwait(0.2);
    /// ```
    pub const fn set_postwait(&mut self, postwait: f32) {
        self.postwait = postwait;
    }

    /// Gets the follow state of a cue
    ///
    /// This will either be `None`, `Continue`, or `Follow`, from the `FollowState` enum.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, FollowState};
    ///
    /// let cue = Cue::default();
    ///
    /// assert_eq!(cue.follow(), FollowState::None);
    /// ```
    #[must_use]
    pub const fn follow(&self) -> FollowState {
        self.follow
    }

    /// Sets the follow state of a cue
    ///
    /// This can either be `None`, `Continue`, or `Follow`, and takes a `FollowState` enum.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, FollowState};
    ///
    /// let mut cue = Cue::default();
    ///
    /// cue.set_follow(FollowState::Continue);
    /// ```
    pub const fn set_follow(&mut self, follow: FollowState) {
        self.follow = follow;
    }
}

mod tests {
    use super::*;

    #[test]
    fn new_default_cue() {
        assert!(matches!(Cue::default().cue_type(), CueType::Audio(_)));
        assert!(matches!(Cue::default().cue_type_mut(), CueType::Audio(_)));
    }

    #[test]
    fn new_default_from_type() {
        assert!(matches!(
            Cue::default_from_type(CueType::Fade(TargetCue::default())).cue_type(),
            CueType::Fade(_)
        ));
    }

    #[test]
    fn get_row() {
        let cue = Cue::default();
        let match_row = Row::new(vec![
            String::from(" "), // Currently playing
            String::from(""), // Cue Type
            String::new(),     // Number
            String::new(),     // Name
            String::new(),     // Target (not actually part of cue)
            String::from("0"), // Prewait
            String::from("0"), // Duration
            String::from("0"), // Postwait
            String::from(" "), // Follow
        ]);

        assert_eq!(cue.get_row(), match_row);
    }

    // TODO: come back to this!
    fn get_target_audio() {
        // Type is Audio
        let cue = Cue::default();

        assert!(matches!(cue.cue_type(), CueType::Audio(_)));
    }

    #[test]
    fn name() {
        assert_eq!(Cue::default().name(), String::new());
    }

    #[test]
    fn set_name() {
        let mut cue = Cue::default();
        cue.set_name(String::from("label"));
        assert_eq!(cue.name(), String::from("label"));
    }

    #[test]
    fn number() {
        assert_eq!(Cue::default().number(), String::new());
    }

    #[test]
    fn set_number() {
        let mut cue = Cue::default();
        cue.set_number(String::from("3"));
        assert_eq!(cue.number(), String::from("3"));
    }

    #[test]
    fn status() {
        assert!(matches!(Cue::default().status(), PlayState::None));
    }

    #[test]
    fn set_status() {
        let mut cue = Cue::default();
        cue.set_status(PlayState::Playing);
        assert!(matches!(cue.status(), PlayState::Playing));
    }

    #[test]
    fn prewait() {
        assert_eq!(Cue::default().prewait(), 0.0);
    }

    #[test]
    fn set_prewait() {
        let mut cue = Cue::default();
        cue.set_prewait(0.2);
        assert_eq!(cue.prewait(), 0.2);
    }

    #[test]
    fn postwait() {
        assert_eq!(Cue::default().postwait(), 0.0);
    }

    #[test]
    fn set_postwait() {
        let mut cue = Cue::default();
        cue.set_postwait(0.2);
        assert_eq!(cue.postwait(), 0.2);
    }

    #[test]
    fn follow() {
        assert!(matches!(Cue::default().follow(), FollowState::None));
    }

    #[test]
    fn set_follow() {
        let mut cue = Cue::default();
        cue.set_follow(FollowState::Continue);
        assert_eq!(cue.follow(), FollowState::Continue);
    }
}
