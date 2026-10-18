use crate::model::cues::Cue;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FadeCue {
    target: Option<f32>,
}

impl FadeCue {
    #[must_use]
    pub const fn new(target: Option<f32>) -> Self {
        Self { target }
    }

    #[must_use]
    pub const fn target(&self) -> Option<f32> {
        self.target
    }
}
