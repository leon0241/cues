#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum FollowState {
    #[default]
    None,
    Follow,
    Continue,
}

impl FollowState {
    #[must_use]
    pub const fn icon(self) -> &'static str {
        match self {
            Self::None => " ",
            Self::Follow => "",
            Self::Continue => "",
        }
    }
}

#[derive(Clone, Copy)]
pub enum PlayState {
    None,
    Playing,
    Paused,
    Fading,
    Stopped,
}

impl PlayState {
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

#[allow(dead_code)]
pub struct TargetCue {
    number: f32,
    cue_type: CueType,
    abs_number: i32,
}

pub enum TargetType {
    None,
    File,
    TargetCue,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum CueType {
    #[default]
    Audio,
    Fade,
    Start,
    Stop,
    Pause,
}

impl CueType {
    #[must_use]
    pub const fn icon(&self) -> &'static str {
        match self {
            Self::Audio => "",
            Self::Fade => "",
            Self::Stop => "",
            Self::Pause => "",
            Self::Start => "",
        }
    }
}

pub struct CueBase {
    pub cue_type: CueType,
    pub number: f32,
    pub name: String,
    pub follow: FollowState,
    pub prewait: f32,
    pub duration: f32,
    pub postwait: f32,
    pub status: PlayState,
    pub parent: Option<TargetCue>,
}

impl Default for CueBase {
    fn default() -> Self {
        Self::new(
            0_f32,
            String::new(),
            0_f32,
            FollowState::None,
            CueType::Audio,
        )
    }
}

impl CueBase {
    #[must_use]
    pub const fn new(
        number: f32,
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
            target: TargetType::File,
            parent: None,
        }
    }
}

/// Helper Trait for Blanket Impl.
pub trait HasCueBase {
    fn base(&self) -> &CueBase;

    fn set_base(&mut self) -> &mut CueBase;
}
