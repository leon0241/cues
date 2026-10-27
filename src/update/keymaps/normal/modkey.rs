//! # Normal Mode Mod Key Handler
//!
//! Handles combo'd `ModKeys` in Normal Mode.

use crate::model::{
    cue::{CopiedCell, CueColumn},
    cuestack::CueStack,
    editor::{Clipboard, ComboState, Editor, Mode},
    keymaps::{CueCmd, Message, ModKeyCmd},
};

/// Handler for Modkey commands
///
/// # Errors
///
/// Will return `Err` if a file cell is set and the file path cannot be read.
pub fn modkey_handler(
    editor: &mut Editor,
    cuestack: &mut CueStack,
    cmd: &ModKeyCmd,
) -> color_eyre::Result<Option<Message>> {
    const fn match_inner(cmd: CueCmd) -> Option<CueColumn> {
        match cmd {
            CueCmd::Cue | CueCmd::Current => None,
            CueCmd::Number => Some(CueColumn::Number),
            CueCmd::Name => Some(CueColumn::Name),
            CueCmd::Target => Some(CueColumn::Target),
            CueCmd::PreWait => Some(CueColumn::PreWait),
            CueCmd::PostWait => Some(CueColumn::PostWait),
            CueCmd::Follow => Some(CueColumn::Follow),
        }
    }

    match cmd {
        ModKeyCmd::Yank(cue_cmd) => {
            if *cue_cmd == CueCmd::Cue {
                let cue_to_copy = cuestack.copy_cue();
                editor.clipboard_copy_cue(cue_to_copy);
            } else if *cue_cmd == CueCmd::Current {
                let cell_to_copy: CopiedCell = cuestack.copy_current_cell();
                editor.clipboard_copy_cell(cell_to_copy);
            } else if let Some(i) = match_inner(*cue_cmd) {
                let cell_to_copy: CopiedCell = cuestack.copy_cell(&i);
                editor.clipboard_copy_cell(cell_to_copy);
            }
        }
        ModKeyCmd::Delete(cue_cmd) => {
            if *cue_cmd == CueCmd::Cue {
                cuestack.delete_cue();
            } else if *cue_cmd == CueCmd::Current {
                cuestack.delete_current_cell();
            } else if let Some(i) = match_inner(*cue_cmd) {
                cuestack.delete_cell(&i);
            }
        }
        ModKeyCmd::Change(cue_cmd) => {
            if *cue_cmd == CueCmd::Cue {
                cuestack.clear_cue();
            } else if *cue_cmd == CueCmd::Current {
                cuestack.delete_current_cell();
                editor.reset_input_buffer();
                editor.reset_cursor();
            } else if let Some(i) = match_inner(*cue_cmd) {
                cuestack.delete_cell(&i);
                editor.reset_input_buffer();
                editor.reset_cursor();
            }
            editor.set_mode(Mode::Insert);
        }
        ModKeyCmd::Paste => {
            let clipboard: Clipboard = editor.clipboard_paste();

            match clipboard {
                Clipboard::Cue(cue) => cuestack.new_cue(cue),
                Clipboard::Cues(_cues) => {}
                Clipboard::Cell(cell) => cuestack.set_current_cell_str(cell.as_str())?,
                Clipboard::Text(text) => cuestack.set_current_cell_str(text)?,
                Clipboard::Cells(_cells) => {}
                Clipboard::None => {}
            }
        }
    }

    editor.set_combo_state(ComboState::None);

    Ok(None)
}
