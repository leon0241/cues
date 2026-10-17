pub mod input;

use crate::model::keymaps::KeyMaps;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Visual,
}

impl Mode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Insert => "Insert",
            Self::Visual => "Visual",
        }
    }
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

impl ComboState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Delete => "Delete",
            Self::Create => "Create",
            Self::Change => "Change",
            Self::Yank => "Yank",
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Debug, Default)]
pub struct Editor {
    mode: Mode,
    running_state: RunningState,
    combo_state: ComboState,
    keymaps: KeyMaps,
    input_buffer: String,
    cursor_idx: usize,
    // clipboard:
}

impl Editor {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    pub const fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
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

    #[must_use]
    pub const fn keymaps(&self) -> &KeyMaps {
        &self.keymaps
    }

    #[must_use]
    pub fn input_buffer(&self) -> String {
        self.input_buffer.clone()
    }

    pub fn set_input_buffer(&mut self, str: String) {
        self.input_buffer = str;
    }
}
