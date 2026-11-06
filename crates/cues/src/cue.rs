//! # Cue
//!
//! Handler for individual cues of any type.
//! Individual Cue types are split out with an Enum called ``CueType``

pub mod audio_cue;
pub mod copied_cell;
pub mod cue_column;
pub mod cue_type;
pub mod follow_state;
pub mod play_state;
pub mod target_cue;

use ratatui::widgets::Row;

// Expose Submodules
pub use crate::cue::{
    audio_cue::AudioCue, copied_cell::CopiedCell, cue_column::CueColumn, cue_type::CueType,
    follow_state::FollowState, play_state::PlayState, target_cue::TargetCue,
};

/// Time
///
/// Time struct featuring a time in both f32 format as well as str format
/// And probably more later
#[derive(Debug, Clone)]
pub struct Time {
    time: f32,
    time_str: String,
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.time_str)
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            time: Default::default(),
            time_str: String::from("0"),
        }
    }
}

impl Time {
    /// Creates a new Time
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Time;
    ///
    /// let time = Time::new(0.0);
    /// ```
    #[must_use]
    pub fn new(time: f32) -> Self {
        Self {
            time,
            time_str: time.to_string(),
        }
    }

    const fn set_from_f32(&mut self, time: f32) {
        self.time = time;
    }

    /// Returns Time in f32 format
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Time;
    ///
    /// let time = Time::new(0.0);
    ///
    /// assert_eq!(time.as_f32(), 0.0);
    /// ```
    #[must_use]
    pub const fn as_f32(&self) -> f32 {
        self.time
    }

    fn as_str(&self) -> &str {
        &self.time_str
    }
}

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
    prewait: Time,
    duration: Time,
    postwait: Time,
    status: PlayState,
    // parent: Option<TargetCue>,
}

impl Default for Cue {
    fn default() -> Self {
        Self::new(
            "",
            "",
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
    /// Cue alternatively implements Default, as well as a `default_from_type()` function as Default
    /// will always create an audio cue.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, FollowState, CueType, AudioCue};
    ///
    /// let cue = Cue::new("1", "Test", 3.0, FollowState::None, CueType::Audio(AudioCue::default()));
    /// ```
    #[must_use]
    pub fn new(
        number: impl Into<String>,
        name: impl Into<String>,
        duration: f32,
        follow: FollowState,
        cue_type: CueType,
    ) -> Self {
        Self {
            number: number.into(),
            name: name.into(),
            duration: Time::new(duration),
            follow,
            cue_type,
            prewait: Time::default(),
            postwait: Time::default(),
            status: PlayState::None,
            // parent: None,
        }
    }

    /// Creates a specified blank cue type
    ///
    /// Cue alternatively implements Default, which will create a blank audio file. Note that having
    /// a default type requires you to use `CueType` as well as `AudioCue` or `TargetCue`.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, CueType, AudioCue, TargetCue};
    ///
    /// let audio_cue = Cue::default_from_type(CueType::Targetable(TargetCue::default()));
    /// ```
    #[must_use]
    pub fn default_from_type(cue_type: CueType) -> Self {
        Self {
            cue_type,
            ..Self::default()
        }
    }

    /// Gets a Ratatui format `Row`
    ///
    /// All the fields in a `Row` are of type String. The fields are in the following order:
    /// Status | Type | Number | Name | Target | Pre | Duration | Post | Follow
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::Cue;
    ///
    /// let new_cue = Cue::default();
    ///
    /// let row = new_cue.get_row();
    /// ```
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
            self.get_target_str().to_string(),
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

    /// Gets the Cue target (for reference)
    ///
    /// - If the cue is a populated Audio Cue, it returns the file path
    /// - If the cue is an unpopulated Audio Cue, it returns None
    /// - If the cue is a populated Target Cue, it returns the target number
    /// - If the cue is an unpopulated Target cue, it returns None
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, CueType, AudioCue, TargetCue};
    ///
    /// let mut audio_cue = Cue::default_from_type(CueType::Audio(AudioCue::default()));
    ///
    /// assert_eq!(audio_cue.get_target(), None);
    /// ```
    #[must_use]
    pub fn get_target(&self) -> Option<&str> {
        match self.cue_type {
            CueType::Audio(AudioCue::Empty) => None,
            CueType::Audio(AudioCue::Populated(ref file)) => Some(file.file_path()),
            CueType::Targetable(ref t) => Some(t.target().unwrap_or_default()),
        }
    }

    /// Gets the Cue target (for formatting)
    ///
    /// - If the cue is a populated Audio Cue, it returns an icon of a file ()
    /// - If the cue is an unpopulated Audio Cue, it returns ""
    /// - If the cue is a populated Target Cue, it returns the target number
    /// - If the cue is an unpopulated Target cue, it returns ""
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, CueType, AudioCue, TargetCue};
    ///
    /// let mut audio_cue = Cue::default_from_type(CueType::Audio(AudioCue::default()));
    ///
    /// assert_eq!(audio_cue.get_target_str(), "");
    /// ```
    #[must_use]
    pub fn get_target_str(&self) -> &str {
        match self.cue_type {
            CueType::Audio(ref audio_cue) => audio_cue.as_icon(),
            CueType::Targetable(ref t) => t.target().unwrap_or_default(),
        }
    }

    /// Sets the Cue target
    ///
    /// - If the cue is a populated Audio Cue, it sets the file path
    /// - If the cue is a populated Target Cue, it sets the target number
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, CueType, AudioCue, TargetCue};
    ///
    /// let mut audio_cue = Cue::default_from_type(CueType::Audio(AudioCue::default()));
    /// let mut fade_cue = Cue::default_from_type(CueType::Targetable(TargetCue::default()));
    ///
    /// audio_cue.set_target("/test/file/path");
    /// fade_cue.set_target("3");
    /// ```
    pub fn set_target(&mut self, value: impl Into<String>) {
        match self.cue_type {
            CueType::Audio(AudioCue::Empty) => {}
            CueType::Audio(AudioCue::Populated(ref mut f)) => {
                f.set_file_path(value.into());
            }
            CueType::Targetable(ref mut t) => t.set_target(value.into()),
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
    /// let cue = Cue::default_from_type(CueType::Targetable(TargetCue::default()));
    ///
    /// assert!(matches!(cue.cue_type(), CueType::Targetable(_)));
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
    /// let cue = Cue::default_from_type(CueType::Targetable(TargetCue::default()));
    ///
    /// assert!(matches!(cue.cue_type(), CueType::Targetable(_)));
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
    /// assert_eq!(cue.name(), "");
    /// ```
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
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
    /// cue.set_name("label");
    /// ```
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
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
    /// assert_eq!(cue.number(), "");
    /// ```
    #[must_use]
    pub fn number(&self) -> &str {
        &self.number
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
    /// cue.set_number("3");
    /// ```
    pub fn set_number(&mut self, number: impl Into<String>) {
        self.number = number.into();
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
    /// use cues::model::cue::{Cue, Time};
    ///
    /// let cue = Cue::default();
    ///
    /// assert_eq!(cue.prewait().as_f32(), 0.0);
    /// ```
    #[must_use]
    pub const fn prewait(&self) -> &Time {
        &self.prewait
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
        self.prewait.set_from_f32(prewait);
    }

    /// Gets the post wait of a cue
    ///
    /// This will be an f32 though it is a time so maybe later on I will change this.
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, Time};
    ///
    /// let cue = Cue::default();
    ///
    /// assert_eq!(cue.postwait().as_f32(), 0.0);
    /// ```
    #[must_use]
    pub const fn postwait(&self) -> &Time {
        &self.postwait
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
        self.postwait.set_from_f32(postwait);
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

    /// Returns a string value for a specified `CueColumn`.
    ///
    /// The function will call the required string function for whatever column is selected
    ///
    /// # Examples
    /// ```
    /// use cues::model::cue::{Cue, CueColumn};
    ///
    /// let mut cue = Cue::default();
    ///
    /// assert_eq!(cue.get_cell_str(CueColumn::PreWait), "0");
    /// ```
    #[must_use]
    pub fn get_cell_str(&self, column: CueColumn) -> &str {
        match column {
            CueColumn::Playing | CueColumn::Duration => "",
            CueColumn::Name => self.name(),
            CueColumn::Number => self.number(),
            CueColumn::PreWait => self.prewait().as_str(),
            CueColumn::PostWait => self.postwait().as_str(),
            CueColumn::Follow => self.follow().icon(),
            CueColumn::Target => match self.cue_type() {
                CueType::Audio(AudioCue::Empty) => "",
                CueType::Audio(AudioCue::Populated(f)) => f.file_path(),
                CueType::Targetable(t) => t.target().unwrap_or_default(),
            },
        }
    }

    /// Parses a string value and sets it for a specified `CueColumn`.
    ///
    /// The function will call the required parsing function for whatever column is selected
    ///
    /// # Errors
    ///
    /// Errors if the column is a File target, and the file cannot be read
    pub fn set_cell_str(
        &mut self,
        column: &CueColumn,
        value: impl Into<String>,
    ) -> color_eyre::Result<()> {
        fn parser(value: &str) -> f32 {
            value.parse::<f32>().unwrap_or_default()
        }

        match column {
            CueColumn::Playing | CueColumn::Duration => {}
            CueColumn::Name => self.set_name(value),
            CueColumn::Number => self.set_number(value),
            CueColumn::PreWait => self.set_prewait(parser(&value.into())),
            CueColumn::PostWait => self.set_postwait(parser(&value.into())),
            CueColumn::Follow => self.set_follow(FollowState::Follow),
            CueColumn::Target => match self.cue_type_mut() {
                CueType::Audio(AudioCue::Empty) => {}
                CueType::Audio(AudioCue::Populated(f)) => {
                    f.set_file_path(value.into());
                    f.try_read()?;
                }

                CueType::Targetable(t) => t.set_target(value.into()),
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cue_audio::audio_file::AudioFile;

    use super::*;

    #[test]
    fn new() {
        let new_cue = Cue::new(
            "1",
            "test",
            0.0,
            FollowState::Follow,
            CueType::Audio(AudioCue::default()),
        );

        assert_eq!(new_cue.number(), "1");
        assert_eq!(new_cue.name(), "test");
        assert_eq!(new_cue.follow(), FollowState::Follow);
        assert_eq!(new_cue.prewait().as_f32(), 0_f32);
        assert_eq!(new_cue.postwait().as_f32(), 0_f32);
        assert!(matches!(new_cue.status(), PlayState::None));
    }

    #[test]
    fn new_default_cue() {
        assert!(matches!(Cue::default().cue_type(), CueType::Audio(_)));
        assert!(matches!(Cue::default().cue_type_mut(), CueType::Audio(_)));
    }

    #[test]
    fn new_default_from_type() {
        assert!(matches!(
            Cue::default_from_type(CueType::Targetable(TargetCue::default())).cue_type(),
            CueType::Targetable(_)
        ));
    }

    #[test]
    fn get_row() {
        let cue = Cue::default();
        let match_row = Row::new(vec![
            " ", // Currently playing
            "", // Cue Type
            "",  // Number
            "",  // Name
            "",  // Target (not actually part of cue)
            "0", // Prewait
            "0", // Duration
            "0", // Postwait
            " ", // Follow
        ]);

        assert_eq!(cue.get_row(), match_row);
    }

    #[test]
    fn get_target_blank_audio() {
        // Type is Audio
        let cue = Cue::default();

        assert_eq!(cue.get_target(), None);
    }

    #[test]
    fn get_target_populated_audio() {
        // Type is Audio
        let mut cue = Cue::default();

        if let CueType::Audio(cue_type) = cue.cue_type_mut() {
            *cue_type = AudioCue::Populated(AudioFile::unchecked_new("name", "/file/path"));
        }

        assert_eq!(cue.get_target(), Some("/file/path"));
    }

    #[test]
    fn get_target_blank_fade() {
        // Type is Audio
        let cue = Cue::default_from_type(CueType::Targetable(TargetCue::default()));

        assert_eq!(cue.get_target(), Some(""));
    }

    #[test]
    fn get_target_populated_fade() {
        // Type is Audio
        let mut cue = Cue::default_from_type(CueType::Targetable(TargetCue::default()));

        cue.set_target("3");

        assert_eq!(cue.get_target(), Some("3"));
    }

    #[test]
    fn get_target_str_blank_audio() {
        // Type is Audio
        let cue = Cue::default();

        assert_eq!(cue.get_target_str(), "");
    }

    #[test]
    fn get_target_str_populated_audio() {
        // Type is Audio
        let mut cue = Cue::default();

        if let CueType::Audio(cue_type) = cue.cue_type_mut() {
            *cue_type = AudioCue::Populated(AudioFile::unchecked_new("name", "/file/path"));
        }

        assert_eq!(cue.get_target_str(), "");
    }

    #[test]
    fn get_target_str_blank_fade() {
        // Type is Audio
        let cue = Cue::default_from_type(CueType::Targetable(TargetCue::default()));

        assert_eq!(cue.get_target_str(), "");
    }

    #[test]
    fn get_target_str_populated_fade() {
        // Type is Audio
        let mut cue = Cue::default_from_type(CueType::Targetable(TargetCue::default()));

        cue.set_target("3");

        assert_eq!(cue.get_target_str(), "3");
    }

    #[test]
    fn name() {
        assert_eq!(Cue::default().name(), "");
    }

    #[test]
    fn set_name() {
        let mut cue = Cue::default();
        cue.set_name("label");
        assert_eq!(cue.name(), "label");
    }

    #[test]
    fn number() {
        assert_eq!(Cue::default().number(), "");
    }

    #[test]
    fn set_number() {
        let mut cue = Cue::default();
        cue.set_number("3");
        assert_eq!(cue.number(), "3");
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
        assert_eq!(Cue::default().prewait().as_f32(), 0.0);
    }

    #[test]
    fn set_prewait() {
        let mut cue = Cue::default();
        cue.set_prewait(0.2);
        assert_eq!(cue.prewait().as_f32(), 0.2);
    }

    #[test]
    fn postwait() {
        assert_eq!(Cue::default().postwait().as_f32(), 0.0);
    }

    #[test]
    fn set_postwait() {
        let mut cue = Cue::default();
        cue.set_postwait(0.2);
        assert_eq!(cue.postwait().as_f32(), 0.2);
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
