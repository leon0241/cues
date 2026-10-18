pub mod audio_cue;
pub mod audio_file;
pub mod cue_base;
pub mod fade_cue;
// pub mod pause_cue;
// pub mod start_cue;
// pub mod stop_cue;

use crate::model::cues::{
    audio_cue::AudioCue,
    cue_base::{FollowState, PlayState, TargetCue},
};

use ratatui::widgets::Row;

#[derive(Debug, PartialEq, Eq)]
pub enum CueColumn {
    Playing,
    Name,
    Number,
    Target,
    PreWait,
    PostWait,
    Duration,
    Follow,
}

impl CueColumn {
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
}

pub enum CueType {
    Audio(AudioCue),
    Fade(TargetCue),
}

impl Default for CueType {
    fn default() -> Self {
        Self::Audio(AudioCue::default())
    }
}

impl CueType {
    #[must_use]
    pub const fn icon(&self) -> &'static str {
        match self {
            Self::Audio(_) => "",
            Self::Fade(_) => "",
            // Self::Stop => "",
            // Self::Pause => "",
            // Self::Start => "",
        }
    }
}

pub struct Cue {
    pub cue_type: CueType,
    pub number: String,
    pub name: String,
    pub follow: FollowState,
    pub prewait: f32,
    pub duration: f32,
    pub postwait: f32,
    pub status: PlayState,
    pub parent: Option<TargetCue>,
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
//
// pub trait CueBase {
//     /// Returns cue in table row format for Ratatui
//     fn get_row(&self) -> Row<'_>;
//     //
//     // /// Play functionality for a cue. Note this is not just
//     // /// "Play" but rather what happens when Space is pressed
//     // fn play_cue(&self) -> color_eyre::Result<()>;
//
//     /// Get Cue Type
//     fn cue_type(&self) -> &CueType;
//
//     /// Get Name (Label) of a cue
//     fn name(&self) -> String;
//
//     /// Set Name (Label) of a cue
//     fn set_name(&mut self, name: String);
//
//     /// Get Number of a cue
//     fn number(&self) -> String;
//
//     /// Set Number of a cue
//     fn set_number(&mut self, number: String);
//
//     /// Get Follow State of a cue
//     fn follow_state(&self) -> String;
//
//     /// Set Follow State of a cue
//     fn set_follow_state(&mut self, state: FollowState);
//
//     /// Get (Playing Status) of a cue
//     fn status(&self) -> PlayState;
//
//     /// Set (Playing Status) of a cue
//     fn set_status(&mut self, icon: PlayState);
//
//     /// # Errors
//     ///
//     /// Will return `Err` if cue cannot be played
//     fn play_cue(&self) -> color_eyre::Result<()> {
//         Ok(())
//     }
// }

// Implement for all types where T ==(has the trait)=> Cue
impl Cue {
    #[must_use]
    pub fn get_row(&self) -> Row<'_> {
        Row::new(vec![
            self.status.icon().to_string(),
            self.cue_type.icon().to_string(),
            self.number.clone(),
            self.name.clone(),
            self.prewait.to_string(),
            self.duration.to_string(),
            self.postwait.to_string(),
            self.follow.icon().to_string(),
        ])
    }

    #[must_use]
    pub const fn cue_type(&self) -> &CueType {
        &self.cue_type
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[must_use]
    pub fn number(&self) -> String {
        self.number.clone()
    }

    fn set_number(&mut self, number: String) {
        self.number = number;
    }

    fn follow_state(&self) -> String {
        match self.follow {
            FollowState::Follow => String::from("F"),
            FollowState::Continue => String::from("C"),
            FollowState::None => String::new(),
        }
    }

    const fn set_follow_state(&mut self, state: FollowState) {
        self.follow = state;
    }

    const fn status(&self) -> PlayState {
        self.status
    }

    pub const fn set_status(&mut self, status: PlayState) {
        self.status = status;
    }

    pub fn target(&self) -> String {
        match self.cue_type {
            CueType::Audio(_) => String::new(),
            CueType::Fade(ref target) => target.target().unwrap_or_else(String::new),
        }
    }

    /// # Errors
    ///
    /// Doesn't actually have any WIP TODO
    pub const fn play_cue(&self) -> color_eyre::Result<()> {
        Ok(())
    }
}

impl Cue {
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
            parent: None,
        }
    }

    #[must_use]
    pub fn get_target(&self) -> String {
        match self.cue_type {
            CueType::Fade(ref fade_cue) => fade_cue.target().unwrap_or_else(String::new),
            CueType::Audio(ref audio_cue) => audio_cue.as_icon().to_string(),
        }
    }

    pub fn set_target(&mut self, value: String) {
        match self.cue_type {
            CueType::Fade(ref mut fade_cue) => fade_cue.set_target(value),
            CueType::Audio(_) => {}
        }
    }

    #[must_use]
    pub fn default_from_type(cue_type: CueType) -> Self {
        Self {
            cue_type,
            ..Self::default()
        }
    }
}
