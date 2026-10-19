#[derive(Default)]
pub struct TargetCue {
    number: Option<String>,
    // cue_type: CueType,
    // abs_number: i32,
}

// is this useful?
// pub trait Modifier {
//     fn target(&self) -> Option<String> {
//         None
//     }
//
//     #[allow(unused_variables)]
//     fn set_target(&mut self, target: String) {}
// }

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
