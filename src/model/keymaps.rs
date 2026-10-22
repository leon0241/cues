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

#[derive(Debug, Default, Clone, Copy)]
pub enum Message {
    #[default]
    None,
    Navigation(NavigationCmd),
    Playback(PlaybackCmd),
    Editor(EditorCmd),
    Input(InputCmd),
    ModKey(ModKeyCmd),
    Create(CreateCmd),
    Combo(ComboCmd),
}

#[derive(Debug, Clone, Copy)]
pub enum InputCmd {
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorStart,
    MoveCursorEnd,
    InsertChar(char),
    DeleteChar,
    FinishInput,
}

#[derive(Debug, Clone, Copy)]
pub enum NavigationCmd {
    LeftCol,
    PrevCue,
    NextCue,
    RightCol,
}

#[derive(Debug, Clone, Copy)]
pub enum PlaybackCmd {
    PlayCue,
    PauseCue,
    StopCue,
    FadeStopCue,
}

#[derive(Debug, Clone, Copy)]
pub enum EditorCmd {
    Quit,
    EnterInsertStart,
    EnterInsertEnd,
    CancelInsertEnterNormal,
    SaveInsertEnterNormal,
    EnterVisual,
}

#[derive(Debug, Clone, Copy)]
pub enum ComboCmd {
    Return,
    EnterDelete, // d ->
    EnterCreate, // o ->
    EnterChange, // c ->
    EnterYank,   // y ->
}

#[derive(Debug, Clone, Copy)]
pub enum CreateCmd {
    CreateAudio,
    CreateFade,
    CreateStart,
    CreateStop,
    CreatePause,
}

#[derive(Debug, Clone, Copy)]
pub enum ModifierCmd {
    ModifyCue,
    NewCue,
    DeleteCue,
}

#[derive(Debug, Clone, Copy)]

pub enum ModKeyCmd {
    Yank(CueCmd),
    Delete(CueCmd),
    Change(CueCmd),
    Paste,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enum for all Cue column editing
pub enum CueCmd {
    Cue,
    Name,
    Number,
    Target,
    PreWait,
    PostWait,
    Follow,
}

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

    #[must_use]
    pub const fn normal_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.normal
    }

    #[must_use]
    pub const fn yank_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.yank
    }

    #[must_use]
    pub const fn change_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.change
    }

    #[must_use]
    pub const fn create_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.create
    }

    #[must_use]
    pub const fn visual_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.visual
    }

    #[must_use]
    pub const fn vline_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.vline
    }

    #[must_use]
    pub const fn delete_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.delete
    }

    #[must_use]
    pub const fn insert_maps(&self) -> &HashMap<KeyCode, Message> {
        &self.insert
    }
}
