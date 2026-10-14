use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{model::{cues::cue::CueColumn, cuestack::CueStack, editor::ComboState}, update::keymaps_file::Message};

#[derive(Debug)]

pub enum ModKeyCmd {
    Yank(CueCmd),
    Delete(CueCmd),
    Create(CueCmd),
    Change(CueCmd),
}

#[derive(Debug)]
/// Enum for all Cue column editing
pub enum CueCmd {
    Cue,
    CueNum,
    Name,
    PreWait,
    PostWait,
    Follow
}

/// Handles double key modifiers such as Yank, Delete
const fn modkey(key: KeyCode, _modifier: KeyModifiers, combo_state: ComboState) -> Option<Message> {
    // Match the key press
    let cue_type: Option<CueCmd> = match key {
        KeyCode::Char('q') => Some(CueCmd::CueNum),
        KeyCode::Char('n') => Some(CueCmd::Name),
        KeyCode::Char('f') => Some(CueCmd::PreWait),
        KeyCode::Char('F') => Some(CueCmd::PostWait),
        KeyCode::Char('w') => Some(CueCmd::Follow),
        // Double presses e.g. yy => yank line
        _ => {
            match (key, combo_state) {
                (KeyCode::Char('y'), ComboState::Yank) => Some(CueCmd::Cue),
                (KeyCode::Char('d'), ComboState::Delete) => Some(CueCmd::Cue),
                // (KeyCode::Char('o'), ComboState::Create) => Some(CueCmd::Cue),
                (KeyCode::Char('c'), ComboState::Change) => Some(CueCmd::Cue),
                _ => None
            }
        },
    };

    // If there was a compatible command, map to the associated command
    if let Some(i) = cue_type {
        let mod_key = match combo_state {
            ComboState::Yank => Some(ModKeyCmd::Yank(i)),
            ComboState::Delete => Some(ModKeyCmd::Delete(i)),
            ComboState::Change => Some(ModKeyCmd::Change(i)),
            // ComboState::Create => Some(ModKeyCmd::Create(i)),
            ComboState::None => None
        };
        if let Some(i) = mod_key {
            return Some(Message::ModKey(i))
        }
    }
    // Else return none and reset
    None
}

pub fn modkey_handler(model: &mut CueStack, cmd: &ModKeyCmd) -> color_eyre::Result<Option<Message>> {

    fn match_inner(cmd: &CueCmd) -> Option<CueColumn> {
        match cmd {
            CueCmd::Cue => None,
            CueCmd::CueNum => Some(CueColumn::Number),
            CueCmd::Name => Some(CueColumn::Name),
            CueCmd::PreWait => Some(CueColumn::PreWait),
            CueCmd::PostWait => Some(CueColumn::PostWait),
            CueCmd::Follow => Some(CueColumn::Follow)
        }
    }


    let cmd2 = match cmd {
        ModKeyCmd::Yank(cue_cmd) => {
            match cue_cmd {
                CueCmd::Cue => {
                    model.copy_cue()
                }
                cell => {
                    if let Some(i) = match_inner(cell) {
                        model.copy_cell(i);
                    }
                }
            }
        },
        ModKeyCmd::Delete(cue_cmd) => {
            match cue_cmd {
                CueCmd::Cue => {
                    model.delete_cue()
                }
                cell => {
                    if let Some(i) = match_inner(cell) {
                        model.delete_cell(i);
                    }
                }
            }
        },
        ModKeyCmd::Create(cue_cmd) => {
            match cue_cmd {
                CueCmd::Cue => {
                    
                }
                cell => {
                    if let Some(i) = match_inner(cell) {
                        model.delete_cell(i);
                    }
                }
            }
        },
        ModKeyCmd::Change(cue_cmd) => {

        },
    };

}
