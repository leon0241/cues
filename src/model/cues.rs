pub mod audio_cue;
pub mod audio_file;
pub mod cue_base;
pub mod target_cue;

use crate::model::cues::{
    audio_cue::AudioCue,
    cue_base::{FollowState, PlayState},
    target_cue::TargetCue,
};

use ratatui::widgets::Row;

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum CueColumn {
    Playing,
    #[default]
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

    #[must_use]
    pub fn follow_state(&self) -> String {
        match self.follow {
            FollowState::Follow => String::from("F"),
            FollowState::Continue => String::from("C"),
            FollowState::None => String::new(),
        }
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

    pub fn target(&self) -> String {
        match self.cue_type {
            CueType::Audio(_) => String::new(),
            CueType::Fade(ref t)
            | CueType::Stop(ref t)
            | CueType::Pause(ref t)
            | CueType::Start(ref t) => t.target().unwrap_or_else(String::new),
        }
    }

    #[must_use]
    pub const fn prewait(&self) -> f32 {
        self.prewait
    }

    #[must_use]
    pub const fn postwait(&self) -> f32 {
        self.postwait
    }

    pub const fn set_prewait(&mut self, prewait: f32) {
        self.prewait = prewait;
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

    #[must_use]
    pub fn default_from_type(cue_type: CueType) -> Self {
        Self {
            cue_type,
            ..Self::default()
        }
    }
}
