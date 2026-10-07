#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Visual
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Editor {
    mode: Mode,
}

impl Editor {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn get_mode(&self) -> Mode {
        self.mode
    }

    #[must_use]
    pub fn is_normal(&self) -> bool {
        self.mode == Mode::Normal
    }
}
