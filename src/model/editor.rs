//! # Editor
//!
//! Handler for the editor as a whole
//! Includes stuff like normal mode.

pub mod input;

use crate::model::{
    cue::{CopiedCell, Cue},
    keymaps::KeyMaps,
};

/// Current Editor Mode
///
/// These follow the same modes as Vim
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    /// Normal - Movements and such
    #[default]
    Normal,
    /// Insert - text input. For this application Insert mode
    /// locks input within the command line
    Insert,
    /// Visual - Selection of cells / cues
    Visual,
}

impl Mode {
    /// Returns a string variant of the mode
    ///
    /// Used for rendering to Ratatui
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Insert => "Insert",
            Self::Visual => "Visual",
        }
    }
}

/// Current Combo State
///
/// This is for combo keys such as delete or yank
/// which require two keys to form a keymap.
///
/// When a key is pressed, The key checker will match
/// against the combo state and get a relevant keymap from there
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum ComboState {
    /// None - Regular Keymaps
    #[default]
    None,
    /// Delete Mode (d_)
    Delete,
    /// Create/Open Mode (o_)
    Create,
    /// Change/Clear Mode (c_).
    Change,
    /// Yank/Copy Mode (y_).
    Yank,
}

impl ComboState {
    /// Returns a string variant of the mode
    ///
    /// Used for rendering to Ratatui
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

/// Current Running State
///
/// Should always be running, done quits the app
#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    /// App currently Running.
    #[default]
    Running,
    /// App finished or panicked.
    Done,
}

#[derive(Debug, Default, Clone)]
pub enum Clipboard {
    #[default]
    None,
    Text(String),
    Cue(Cue),
    Cues(Vec<Cue>),
    Cell(CopiedCell),
    Cells(Vec<CopiedCell>),
}

impl Clipboard {
    #[must_use]
    pub const fn from_cue(cue: Cue) -> Self {
        Self::Cue(cue)
    }

    #[must_use]
    pub const fn from_cell(cell: CopiedCell) -> Self {
        Self::Cell(cell)
    }
}

#[derive(Debug, Default)]
pub struct Editor {
    mode: Mode,
    running_state: RunningState,
    combo_state: ComboState,
    keymaps: KeyMaps,
    input_buffer: String,
    cursor_idx: usize,
    clipboard: Clipboard,
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

    pub fn reset_input_buffer(&mut self) {
        self.input_buffer = String::new();
    }

    pub fn clipboard_copy_cue(&mut self, cue: Cue) {
        self.clipboard = Clipboard::Cue(cue);
    }

    pub fn clipboard_copy_cell(&mut self, cell: CopiedCell) {
        self.clipboard = Clipboard::Cell(cell);
    }

    pub fn clipboard_paste(&mut self) -> Clipboard {
        self.clipboard.clone()
    }
}
