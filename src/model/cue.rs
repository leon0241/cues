//! # Cue
//!
//! Handler for individual cues of any type.
//! Individual Cue types are split out with an Enum called ``CueType``

pub mod audio_cue;
pub mod audio_file;
pub mod cue_base;
pub mod target_cue;

use crate::model::cue::{
    audio_cue::AudioCue,
    cue_base::{FollowState, PlayState},
    target_cue::TargetCue,
};

use ratatui::widgets::Row;

pub enum CueType {
    Audio(AudioCue),
    Fade(TargetCue),
    Stop(TargetCue),
    Pause(TargetCue),
    Start(TargetCue),
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
            Self::Stop(_) => "",
            Self::Pause(_) => "",
            Self::Start(_) => "",
        }
    }
}

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

    #[must_use]
    pub fn default_from_type(num: usize, cue_type: CueType) -> Self {
        Self {
            number: num.to_string(),
            cue_type,
            ..Self::default()
        }
    }

    #[must_use]
    pub fn get_row(&self) -> Row<'_> {
        Row::new(vec![
            self.status.icon().to_string(),
            self.cue_type.icon().to_string(),
            self.number.clone(),
            self.name.clone(),
            self.get_target(),
            self.prewait.to_string(),
            self.duration.to_string(),
            self.postwait.to_string(),
            self.follow.icon().to_string(),
        ])
    }

    #[must_use]
    pub fn get_target(&self) -> String {
        match self.cue_type {
            CueType::Audio(ref audio_cue) => audio_cue.as_icon().to_string(),
            CueType::Fade(ref t)
            | CueType::Stop(ref t)
            | CueType::Pause(ref t)
            | CueType::Start(ref t) => t.target().unwrap_or_else(String::new),
        }
    }

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
    #[must_use]
    pub const fn cue_type(&self) -> &CueType {
        &self.cue_type
    }

    pub const fn cue_type_mut(&mut self) -> &mut CueType {
        &mut self.cue_type
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[must_use]
    pub fn number(&self) -> String {
        self.number.clone()
    }

    pub fn set_number(&mut self, number: String) {
        self.number = number;
    }

    const fn _set_follow_state(&mut self, state: FollowState) {
        self.follow = state;
    }

    const fn _status(&self) -> PlayState {
        self.status
    }

    pub const fn set_status(&mut self, status: PlayState) {
        self.status = status;
    }

    #[must_use]
    pub const fn prewait(&self) -> f32 {
        self.prewait
    }

    pub const fn set_prewait(&mut self, prewait: f32) {
        self.prewait = prewait;
    }

    #[must_use]
    pub const fn postwait(&self) -> f32 {
        self.postwait
    }

    pub const fn set_postwait(&mut self, postwait: f32) {
        self.postwait = postwait;
    }

    #[must_use]
    pub const fn follow(&self) -> FollowState {
        self.follow
    }

    pub const fn set_follow(&mut self, follow: FollowState) {
        self.follow = follow;
    }
}
