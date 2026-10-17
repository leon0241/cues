use crate::model::keymaps::KeyMaps;

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Visual,
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

#[derive(Debug, Default)]
pub struct Editor {
    mode: Mode,
    running_state: RunningState,
    combo_state: ComboState,
    keymaps: KeyMaps,
    _input_buffer: String,
    character_index: usize,
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

    #[must_use]
    pub fn mode_str(&self) -> String {
        match self.mode {
            Mode::Normal => String::from("Normal"),
            Mode::Insert => String::from("Insert"),
            Mode::Visual => String::from("Visual"),
        }
    }

    #[must_use]
    pub fn combo_str(&self) -> String {
        match self.combo_state {
            ComboState::None => String::from("None"),
            ComboState::Delete => String::from("Delete"),
            ComboState::Create => String::from("Create"),
            ComboState::Change => String::from("Change"),
            ComboState::Yank => String::from("Yank"),
        }
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
    pub const fn character_index(&self) -> usize {
        self.character_index
    }
}
