#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Visual
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum ComboState {
    #[default]
    None,
    Delete, // d -> 
    Create, // o -> 
    Change, // c ->
    Yank,   // y ->
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
    running_state: RunningState,
    combo_state: ComboState,
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
    pub fn get_mode_str(&self) -> String {
        match self.mode {
            Mode::Normal => {
                String::from("Normal")
            }
            Mode::Insert => {
                String::from("Insert")
            }
            Mode::Visual => {
                String::from("Visual")
            }
        }
    }

    pub const fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    #[must_use]
    pub fn is_normal(&self) -> bool {
        self.mode == Mode::Normal
    }

    pub const fn set_running_state(&mut self, running_state: RunningState) {
        self.running_state = running_state;
    }

    #[must_use]
    pub const fn running_state(&self) -> &RunningState {
        &self.running_state
    }

    #[must_use]
    pub const fn combo_state(&self) -> ComboState {
        self.combo_state
    }

    pub const fn set_combo_state(&mut self, combo_state: ComboState) {
        self.combo_state = combo_state;
    }
}
