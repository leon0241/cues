use crate::model::cues::{
    Cue,
    cue_base::{CueBase, CueType, FollowState, HasCueBase},
};

#[derive(Default)]
pub struct StartCue {
    base: CueBase,
    target: Option<f32>,
}

//
// #[allow(dead_code)]
// trait ModifierFn: Cue { }

impl HasCueBase for StartCue {
    fn base(&self) -> &CueBase {
        &self.base
    }

    fn set_base(&mut self) -> &mut CueBase {
        &mut self.base
    }
}

impl Cue for StartCue {
    fn target(&self) -> Option<f32> {
        self.target
    }

    fn set_target(&mut self, target: f32) {
        self.target = Some(target);
    }
}

impl StartCue {
    #[must_use]
    pub const fn new(
        number: f32,
        name: String,
        duration: f32,
        follow: FollowState,
        target: Option<f32>,
    ) -> Self {
        Self {
            base: CueBase::new(number, name, duration, follow, CueType::Fade),
            // colour: None,
            target,
        }
    }
}
