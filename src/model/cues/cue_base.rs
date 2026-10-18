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
#[derive(Default)]
pub struct TargetCue {
    number: Option<String>,
    // cue_type: CueType,
    // abs_number: i32,
}

impl TargetCue {
    #[must_use]
    pub const fn new(number: Option<String>) -> Self {
        Self { number }
    }

    #[must_use]
    pub fn target(&self) -> Option<String> {
        self.number.clone()
    }

    pub fn set_target(&mut self, value: String) {
        self.number = Some(value);
    }
}

pub enum TargetType {
    None,
    File,
    TargetCue,
}

pub trait PlayableCue {
    fn pause_cue(&self) -> Option<bool> {
        None
    }

    fn stop_cue(&self) -> Option<bool> {
        None
    }

    #[allow(unused_variables)]
    fn fade_stop_cue(&self, time: f32) -> Option<bool> {
        None
    }
}

pub trait ModifierCue {
    fn target(&self) -> Option<f32> {
        None
    }

    #[allow(unused_variables)]
    fn set_target(&mut self, target: f32) {}
}
