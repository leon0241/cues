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
