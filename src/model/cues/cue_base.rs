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

pub enum TargetType {
    None,
    File,
    TargetCue,
}
