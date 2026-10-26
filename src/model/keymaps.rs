//! # Keymaps
//!
//! Handles the processing of Messages which act as the middle layer between Event handling and
//! Updating. This also stores a `KeyMaps` struct containing default keymaps for all events, stored
//! as different sets of `HashMap`s.
//!
//! The individual `HashMap`s are contained within the `keymaps/` folder.

use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{
    insert_keymaps::init_imaps,
    modkey_keymaps::{init_change_maps, init_create_maps, init_delete_maps, init_yank_maps},
    normal_keymaps::init_nmaps,
};

pub mod insert_keymaps;
pub mod modkey_keymaps;
pub mod normal_keymaps;
pub mod visual_keymaps;
pub mod vline_keymaps;

/// Messages that can be sent from Event handler to Updater
///
/// These can be chained one after the other so that an update function returns a message which then
/// triggers another update function, and so forth.
#[derive(Debug, Default, Clone, Copy)]
pub enum Message {
    /// No message, or "finish event" command
    #[default]
    None,
    /// Navigation such as hjkl movement
    Navigation(NavigationCmd),
    /// Playback such as Play Pause Stop
    Playback(PlaybackCmd),
    /// Editor commands such as switching modes
    Editor(EditorCmd),
    /// Input commands for typing in the command line within Insert mode
    Input(InputCmd),
    /// Mod Keys which require multiple hits such as yank, delete
    ModKey(ModKeyCmd),
    /// Create Keys which require multiple hits but operate on different keybinds to Mod Keys
    Create(CreateCmd),
    /// Combo Keys which change the `ComboState` to switch to a different set of maps within a combo
    Combo(ComboCmd),
}

/// Sends input commands for typing in the command line within Insert mode
#[derive(Debug, Clone, Copy)]
pub enum InputCmd {
    /// Move the cursor 1 char to the left
    MoveCursorLeft,
    /// Move the cursor 1 char to the right
    MoveCursorRight,
    /// Move the cursor 1 char to the start of the input buffer
    MoveCursorStart,
    /// Move the cursor 1 char to the final char of the input buffer
    MoveCursorEnd,
    /// Inserts a keyboard character
    InsertChar(char),
    /// Deletes the character at the current position
    DeleteChar,
    /// Triggers the return to Normal Mode, and Save/Exit Insert mode
    FinishInput,
}

/// Sends navigation commands within Normal mode
#[derive(Debug, Clone, Copy)]
pub enum NavigationCmd {
    /// Move the cursor 1 char to the left
    LeftCol,
    /// Move the cursor 1 char up
    PrevCue,
    /// Move the cursor 1 char down
    NextCue,
    /// Move the cursor 1 char to the right
    RightCol,
}

/// Sends playback commands for Audio cues
#[derive(Debug, Clone, Copy)]
pub enum PlaybackCmd {
    /// Plays the selected Cue
    PlayCue,
    /// Pauses the selected Cue
    PauseCue,
    /// Stops the selected Cue
    StopCue,
    /// Fade + Stops the selected Cue
    FadeStopCue,
}

/// Sends Mode changes for the Editor
#[derive(Debug, Clone, Copy)]
pub enum EditorCmd {
    /// Exits the Program
    Quit,
    /// Enters Insert mode at the start of the buffer
    EnterInsertStart,
    /// Enters Insert mode at the last character of the buffer
    EnterInsertEnd,
    /// Discards any changes and returns to Normal mode
    CancelInsertEnterNormal,
    /// Saves any changes and returns to Normal mode
    SaveInsertEnterNormal,
    /// Enters Visual Mode (Currently non-functional TODO)
    EnterVisual,
}

/// Sends Combo States to switch keymap sets in Normal mode
///
/// Note that this differs from `ModKeyCmd` by handling the keymap switch, while `ModKeyCmd`
/// handles the combo command itself
#[derive(Debug, Clone, Copy)]
pub enum ComboCmd {
    /// Returns to Normal Keymaps
    Return,
    /// Enters Delete (d) Keymaps
    EnterDelete,
    /// Enters Create (o) Keymaps
    EnterCreate,
    /// Enters Change (c) Keymaps
    EnterChange,
    /// Enters Yank (y) Keymaps
    EnterYank,
}

/// Sends creation commands for making new cue types
#[derive(Debug, Clone, Copy)]
pub enum CreateCmd {
    /// Create an Audio Cue
    CreateAudio,
    /// Create a Fade Cue
    CreateFade,
    /// Create a Start Cue
    CreateStart,
    /// Create a Stop Cue
    CreateStop,
    /// Create a Pause Cue
    CreatePause,
}

/// Sends Mod Key combo commands to process.
///
/// Note that this differs from `ComboCmd` by handling the final result command, while `ComboCmd`
/// handles the keymap changes
#[derive(Debug, Clone, Copy)]

pub enum ModKeyCmd {
    /// Sends a Yank command alongside a column/cue to yank
    Yank(CueCmd),
    /// Sends a Delete command alongside a column/cue to delete
    Delete(CueCmd),
    /// Sends a Change command alongside a column/cue to change
    Change(CueCmd),
    /// Sends a Paste command, and will paste either a new cue or the column currently under the
    /// cursor.
    Paste,
}

/// Identifier for different columns to process a Mod Key with
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CueCmd {
    /// Entire Cue
    Cue,
    /// Cue Name
    Name,
    /// Cue Number
    Number,
    /// Cue Target
    Target,
    /// Cue Pre Wait
    PreWait,
    /// Cue Post Wait
    PostWait,
    /// Cue Follow
    Follow,
    /// Whatever column the cursor is currently under
    Current,
    // Note that the duration depends on the cue so can not be modified by the end user
}

/// Handles Different sets of keymaps
///
/// These are stored as a `HashMap` for ease of remaps if necessary.
#[derive(Debug)]
pub struct KeyMaps {
    normal: HashMap<KeyCode, Message>,
    delete: HashMap<KeyCode, Message>,
    yank: HashMap<KeyCode, Message>,
    change: HashMap<KeyCode, Message>,
    create: HashMap<KeyCode, Message>,
    visual: HashMap<KeyCode, Message>,
    vline: HashMap<KeyCode, Message>,
    insert: HashMap<KeyCode, Message>,
}

impl Default for KeyMaps {
    fn default() -> Self {
        Self {
            normal: init_nmaps(),
            delete: init_delete_maps(),
            yank: init_yank_maps(),
            change: init_change_maps(),
            create: init_create_maps(),
            visual: HashMap::new(),
            vline: HashMap::new(),
            insert: init_imaps(),
        }
    }
}

impl KeyMaps {
    /// Creates a new default `KeyMaps` handler.
    ///
    /// This is created with hardcoded defaults, and custom keymaps may be implemented in a later
    /// version whether in an .rc file or otherwise.
    #[must_use]
    pub fn new() -> Self {
        Self {
            normal: init_nmaps(),
            delete: init_delete_maps(),
            yank: init_yank_maps(),
            change: init_change_maps(),
            create: HashMap::new(),
            visual: HashMap::new(),
            vline: HashMap::new(),
            insert: init_imaps(),
        }
    }

    /// Returns base Normal mode keymaps
    #[must_use]
    pub const fn normal_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.normal
    }

    /// Returns Yank keymaps (y_)
    #[must_use]
    pub const fn yank_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.yank
    }

    /// Returns Change keymaps (c_)
    #[must_use]
    pub const fn change_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.change
    }

    /// Returns Create keymaps (o_)
    #[must_use]
    pub const fn create_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.create
    }

    /// Returns Delete keymaps (d_)
    #[must_use]
    pub const fn delete_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.delete
    }

    /// Returns base Visual mode keymaps
    #[must_use]
    pub const fn visual_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.visual
    }

    /// Returns base Visual Line mode keymaps
    #[must_use]
    pub const fn vline_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.vline
    }

    /// Returns base Insert mode keymaps
    #[must_use]
    pub const fn insert_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.insert
    }
}
