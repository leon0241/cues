use crate::model::cues::{
    Cue,
    cue_base::{CueBase, CueType, FollowState, HasCueBase},
};

pub struct FadeCue {
    base: CueBase,
    target: Option<f32>,
}

//
// #[allow(dead_code)]
// trait ModifierFn: Cue { }

impl HasCueBase for FadeCue {
    fn base(&self) -> &CueBase {
        &self.base
    }

    fn set_base(&mut self) -> &mut CueBase {
        &mut self.base
    }
}

impl Cue for FadeCue {
    fn target(&self) -> Option<f32> {
        self.target
    }

    fn set_target(&mut self, target: f32) {
        self.target = Some(target);
    }
}

impl Default for FadeCue {
    fn default() -> Self {
        Self {
            base: CueBase {
                cue_type: CueType::Fade,
                ..CueBase::default()
            },
            target: None,
        }
    }
}

impl FadeCue {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fade_cue_props() {
        let new_fade_cue: FadeCue =
            FadeCue::new(1.0, String::from("Test"), 1.0, FollowState::None, Some(5.2));
        // assert_eq!(new_fade_cue.get_base().number, 1.0);
        assert_eq!(new_fade_cue.base().name, "Test");
        // assert_eq!(new_fade_cue.get_base().duration, 1.0);
        assert_eq!(new_fade_cue.base().follow, FollowState::None);
        // assert_eq!(new_fade_cue.get_target().unwrap(), 5.2);
    }
}
