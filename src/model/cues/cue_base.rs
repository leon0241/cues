#[derive(Debug, Default, PartialEq, Eq)]
pub enum FollowState {
    #[default]
    None,
    Follow,
    Continue
}

pub enum PlayState {
    None,
    Playing,
    Stopped,
}
pub struct TargetCue {
    number: f32,
    cue_type: CueType,
    abs_number: i32
}

pub enum TargetType {
    None,
    File,
    TargetCue
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum CueType {
    #[default]
    Audio,
    Fade
}

pub struct CueBase {
    pub cue_type: CueType,
    pub number: f32,
    pub name: String,
    pub follow: FollowState,
    pub prewait: f32,
    pub duration: f32,
    pub postwait: f32,
    pub icon: String,
    pub status: PlayState,
    pub target: TargetType,
    pub parent: Option<TargetCue>
} 

impl CueBase {
    pub fn new(number: f32, name: String, duration: f32,
        follow: FollowState, cue_type: CueType) -> Self {

        CueBase {
            number,
            name,
            duration,
            follow,
            cue_type,
            prewait: 0_f32,
            postwait: 0_f32,
            icon: String::new(),
            status: PlayState::None,
            target: TargetType::File,
            parent: None,
        }
    }
}

/// Helper Trait for Blanket Impl.
pub trait HasCueBase {
    fn get_base(&self) -> &CueBase;

    fn set_base(&mut self) -> &mut CueBase;
}

