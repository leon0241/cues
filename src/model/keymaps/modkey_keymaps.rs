//! # Mod Key Keymaps
//!
//! Handles the mapping of mod keys that have similar behaviour such as yanking, changing, and
//! deleting.
//!
//! For example yn -> yank name, cn -> change name, dn -> delete name

use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{CreateCmd, CueCmd, Message, ModKeyCmd};

/// Helper function to initialise general maps for a specified modkey
///
/// This is helpful as as most of them have the same mappings
///
/// e.g. dn -> delete name, yn -> yank name, cn -> change name
///
/// Note that this does not include the double press to modify a cue such as cc, yy, or dd.
fn gen_mod_maps(mod_key: fn(CueCmd) -> ModKeyCmd) -> HashMap<KeyCode, Message> {
    // Inserts a mapping with the provided modkey
    //
    // e.g. `ModKeyCmd::Yank` + `CueCmd::CueNum`->
    // `Message::ModKey(ModKeyCmd::Yank(CueCmd::CueNum))`
    let wrapper = |cmd: CueCmd| Message::ModKey(mod_key(cmd));

    HashMap::from([
        (KeyCode::Char('i'), wrapper(CueCmd::Current)),
        (KeyCode::Char('a'), wrapper(CueCmd::Current)),
        (KeyCode::Char('q'), wrapper(CueCmd::Number)),
        (KeyCode::Char('t'), wrapper(CueCmd::Target)),
        (KeyCode::Char('n'), wrapper(CueCmd::Name)),
        (KeyCode::Char('p'), wrapper(CueCmd::PreWait)),
        (KeyCode::Char('P'), wrapper(CueCmd::PostWait)),
        (KeyCode::Char('w'), wrapper(CueCmd::Follow)),
    ])
}

/// Initialises Yank Keymaps.
///
/// This creates standard yank maps for each column with the `gen_mod_maps()` helper function, as
/// well as a map yy for yanking a line
///
/// # Examples
/// ```
/// use cues::model::keymaps::modkey_keymaps::init_yank_maps;
///
/// let ymaps = init_yank_maps();
/// ```
pub fn init_yank_maps() -> HashMap<KeyCode, Message> {
    let mut yank_maps: HashMap<KeyCode, Message> = gen_mod_maps(ModKeyCmd::Yank);

    yank_maps.insert(
        // insert yy -> yank line
        KeyCode::Char('y'),
        Message::ModKey(ModKeyCmd::Yank(CueCmd::Cue)),
    );

    yank_maps
}

/// Initialises Delete Keymaps.
///
/// This creates standard delete maps for each column with the `gen_mod_maps()` helper function, as
/// well as a map dd for deleting a line
///
/// # Examples
/// ```
/// use cues::model::keymaps::modkey_keymaps::init_delete_maps;
///
/// let dmaps = init_delete_maps();
/// ```
pub fn init_delete_maps() -> HashMap<KeyCode, Message> {
    let mut delete_maps: HashMap<KeyCode, Message> = gen_mod_maps(ModKeyCmd::Delete);

    delete_maps.insert(
        // insert dd -> delete line
        KeyCode::Char('d'),
        Message::ModKey(ModKeyCmd::Delete(CueCmd::Cue)),
    );

    delete_maps
}

/// Initialises Change Keymaps.
///
/// This creates standard change maps for each column with the `gen_mod_maps()` helper function, as
/// well as a map dd for deleting a line
///
/// # Examples
/// ```
/// use cues::model::keymaps::modkey_keymaps::init_change_maps;
///
/// let cmaps = init_change_maps();
/// ```
pub fn init_change_maps() -> HashMap<KeyCode, Message> {
    let mut change_maps: HashMap<KeyCode, Message> = gen_mod_maps(ModKeyCmd::Change);

    change_maps.insert(
        // insert cc -> clear line
        KeyCode::Char('c'),
        Message::ModKey(ModKeyCmd::Change(CueCmd::Cue)),
    );

    change_maps
}

/// Initialises Create Keymaps.
///
/// This creates Create maps for each type of cue.
///
/// # Examples
/// ```
/// use cues::model::keymaps::modkey_keymaps::init_create_maps;
///
/// let omaps = init_create_maps();
/// ```
#[must_use]
pub fn init_create_maps() -> HashMap<KeyCode, Message> {
    let wrapper = |cmd: CreateCmd| Message::Create(cmd);

    HashMap::from([
        (KeyCode::Char('o'), wrapper(CreateCmd::CreateAudio)),
        (KeyCode::Char('f'), wrapper(CreateCmd::CreateFade)),
        (KeyCode::Char('s'), wrapper(CreateCmd::CreateStart)),
        (KeyCode::Char('x'), wrapper(CreateCmd::CreateStop)),
        (KeyCode::Char('p'), wrapper(CreateCmd::CreatePause)),
    ])
}
