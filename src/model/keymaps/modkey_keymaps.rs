use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;

use crate::model::keymaps::{CueCmd, Message, ModKeyCmd};

/// Inserts general maps for each modkey as most of them have the same mappings
/// e.g. dn -> delete name, yn -> yank name, cn -> change name
fn gen_mod_maps(mod_key: fn(CueCmd) -> ModKeyCmd) -> HashMap<KeyCode, Message> {
    // Inserts a mapping with the provided modkey
    //
    // e.g. `ModKeyCmd::Yank` + `CueCmd::CueNum`->
    // `Message::ModKey(ModKeyCmd::Yank(CueCmd::CueNum))`
    let wrapper = | cmd: CueCmd | {
        Message::ModKey(mod_key(cmd))
    };

    HashMap::from([
        (KeyCode::Char('q'), wrapper(CueCmd::CueNum)),
        (KeyCode::Char('n'), wrapper(CueCmd::Name)),
        (KeyCode::Char('p'), wrapper(CueCmd::PreWait)),
        (KeyCode::Char('P'), wrapper(CueCmd::PostWait)),
        (KeyCode::Char('w'), wrapper(CueCmd::Follow))
    ])
}

pub fn init_yank_maps() -> HashMap<KeyCode, Message> {
    let mut yank_maps: HashMap<KeyCode, Message> = gen_mod_maps(ModKeyCmd::Yank);

    yank_maps.insert( // insert yy -> yank line
        KeyCode::Char('y'),
        Message::ModKey(ModKeyCmd::Yank(CueCmd::Cue))
    );

    yank_maps
}

pub fn init_delete_maps() -> HashMap<KeyCode, Message> {
    let mut delete_maps: HashMap<KeyCode, Message> = gen_mod_maps(ModKeyCmd::Delete);

    delete_maps.insert( // insert dd -> delete line
        KeyCode::Char('d'),
        Message::ModKey(ModKeyCmd::Delete(CueCmd::Cue))
    );

    delete_maps
}

pub fn init_change_maps() -> HashMap<KeyCode, Message> {
    let mut change_maps: HashMap<KeyCode, Message> = gen_mod_maps(ModKeyCmd::Change);

    change_maps.insert( // insert cc -> clear line
        KeyCode::Char('c'),
        Message::ModKey(ModKeyCmd::Change(CueCmd::Cue))
    );

    change_maps
}
