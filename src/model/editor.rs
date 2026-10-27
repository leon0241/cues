//! # Editor
//!
//! Handler for the editor as a whole
//!
//! Includes stuff like normal mode.

pub mod input;
pub mod mode;

// Expose Mode
pub use crate::model::editor::mode::Mode;

use crate::model::{
    cue::{CopiedCell, Cue},
    keymaps::KeyMaps,
};

/// Current Combo State
///
/// This is for combo keys such as delete or yank which require two keys to form a keymap.
///
/// When a key is pressed, The key checker will match against the combo state and get a relevant
/// keymap from there.
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
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::ComboState;
    ///
    /// assert_eq!(ComboState::Yank.as_str(), "Yank");
    /// ```
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

/// Clipboard
///
/// This makes matching the different types of objects you can copy possible.
#[derive(Debug, Default, Clone)]
pub enum Clipboard {
    /// Nothing copied to the clipboard
    #[default]
    None,
    /// Plain text copied to the clipboard, and hopefully the system clipboard as well. TODO
    Text(String),
    /// An entire cue
    Cue(Cue),
    /// Multiple entire cues
    Cues(Vec<Cue>),
    /// A cell of a Cue, in the form of `CopiedCell`.
    Cell(CopiedCell),
    /// Multiple cells of a Cue, in the form of `CopiedCell`.
    Cells(Vec<CopiedCell>),
}

impl Clipboard {
    /// Creates a clipboard Enum from a cue
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Clipboard;
    /// use cues::model::cue::Cue;
    ///
    /// Clipboard::from_cue(Cue::default());
    /// ```
    #[must_use]
    pub const fn from_cue(cue: Cue) -> Self {
        Self::Cue(cue)
    }

    /// Creates a clipboard Enum from a cell
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Clipboard;
    /// use cues::model::cue::CopiedCell;
    ///
    /// Clipboard::from_cell(CopiedCell::default());
    /// ```
    #[must_use]
    pub const fn from_cell(cell: CopiedCell) -> Self {
        Self::Cell(cell)
    }
}

/// Main Editor Struct
///
/// This handles anything that isn't cue related, such as modes, keymaps, clipboards, command
/// line text, and more.
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
    /// Creates a new Editor
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Editor;
    ///
    /// let new_editor = Editor::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the current mode (Normal, Insert, Visual, Vline).
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::{Editor, Mode};
    ///
    /// let new_editor = Editor::new();
    ///
    /// assert!(matches!(new_editor.mode(), Mode::Normal));
    /// ```
    #[must_use]
    pub const fn mode(&self) -> Mode {
        self.mode
    }

    /// Sets the current mode (Normal, Insert, Visual, Vline).
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::{Editor, Mode};
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// new_editor.set_mode(Mode::Insert);
    /// ```
    pub const fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    /// Returns the current running state (Running, Done).
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::{Editor, RunningState};
    ///
    /// let new_editor = Editor::new();
    ///
    /// assert!(matches!(new_editor.running_state(), RunningState::Running));
    /// ```
    #[must_use]
    pub const fn running_state(&self) -> &RunningState {
        &self.running_state
    }

    /// Sets the current running state.
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::{Editor, RunningState};
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// new_editor.set_running_state(RunningState::Running);
    /// ```
    pub const fn set_running_state(&mut self, running_state: RunningState) {
        self.running_state = running_state;
    }

    /// Returns the current combo state (None, Delete, Yank, ..).
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::{Editor, ComboState};
    ///
    /// let new_editor = Editor::new();
    ///
    /// assert!(matches!(new_editor.combo_state(), ComboState::None));
    /// ```
    #[must_use]
    pub const fn combo_state(&self) -> ComboState {
        self.combo_state
    }

    /// Sets the current combo state.
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::{Editor, ComboState};
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// new_editor.set_combo_state(ComboState::Yank);
    /// ```
    pub const fn set_combo_state(&mut self, combo_state: ComboState) {
        self.combo_state = combo_state;
    }

    /// Returns the current `KeyMaps` mappings
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Editor;
    /// use cues::model::keymaps::KeyMaps;
    ///
    /// let new_editor = Editor::new();
    ///
    /// let default_maps = KeyMaps::new();
    ///
    /// assert!(matches!(new_editor.keymaps(), default_maps));
    /// ```
    #[must_use]
    pub const fn keymaps(&self) -> &KeyMaps {
        &self.keymaps
    }

    /// Clones the current input buffer.
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Editor;
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// assert_eq!(new_editor.input_buffer(), String::new());
    /// ```
    #[must_use]
    pub fn input_buffer(&self) -> String {
        self.input_buffer.clone()
    }

    /// Sets the current Input buffer. This can be &str or a String.
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Editor;
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// new_editor.set_input_buffer("Test");
    /// ```
    pub fn set_input_buffer(&mut self, str: impl Into<String>) {
        self.input_buffer = str.into();
    }

    /// Sets the current Input buffer to blank
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Editor;
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// new_editor.reset_input_buffer();
    /// ```
    pub fn reset_input_buffer(&mut self) {
        self.input_buffer = String::new();
    }

    /// Copies a specified Cue to the stored `Clipboard`
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Editor;
    /// use cues::model::cue::Cue;
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// new_editor.clipboard_copy_cue(Cue::default());
    /// ```
    pub fn clipboard_copy_cue(&mut self, cue: Cue) {
        self.clipboard = Clipboard::Cue(cue);
    }

    /// Copies a specified `CopiedCell` to the stored `Clipboard`
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Editor;
    /// use cues::model::cue::CopiedCell;
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// new_editor.clipboard_copy_cell(CopiedCell::None);
    /// ```
    pub fn clipboard_copy_cell(&mut self, cell: CopiedCell) {
        self.clipboard = Clipboard::Cell(cell);
    }

    /// Returns the current stored `Clipboard` value
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::{Editor, Clipboard};
    ///
    /// let mut new_editor = Editor::new();
    ///
    /// assert!(matches!(new_editor.clipboard_paste(), Clipboard::None));
    /// ```
    pub fn clipboard_paste(&mut self) -> Clipboard {
        self.clipboard.clone()
    }
}
