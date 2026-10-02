#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Visual
}

#[derive(Debug, Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default, PartialEq)]
pub struct Editor {
    mode: Mode,
}

impl Editor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_mode(&self) -> Mode {
        self.mode
    }

    pub fn is_normal(&self) -> bool {
        self.mode == Mode::Normal
    }
}
