use crate::model::{
    cues::CueColumn,
    cuestack::CueStack,
    editor::{ComboState, Editor, Mode},
    keymaps::{CueCmd, Message, ModKeyCmd},
};

/// # Errors
///
/// TODO
pub fn modkey_handler(
    editor: &mut Editor,
    cuestack: &mut CueStack,
    cmd: &ModKeyCmd,
) -> color_eyre::Result<Option<Message>> {
    const fn match_inner(cmd: CueCmd) -> Option<CueColumn> {
        match cmd {
            CueCmd::Cue => None,
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
                cuestack.copy_cue();
            } else if let Some(i) = match_inner(*cue_cmd) {
                cuestack.copy_cell(&i);
            }
        }
        ModKeyCmd::Delete(cue_cmd) => {
            if *cue_cmd == CueCmd::Cue {
                cuestack.delete_cue();
            } else if let Some(i) = match_inner(*cue_cmd) {
                cuestack.delete_cell(&i);
            }
        }
        ModKeyCmd::Change(cue_cmd) => {
            if *cue_cmd == CueCmd::Cue {
                cuestack.clear_cue();
            } else if let Some(i) = match_inner(*cue_cmd) {
                cuestack.delete_cell(&i);
                editor.reset_input_buffer();
                editor.reset_cursor();
            }
            editor.set_mode(Mode::Insert);
        }
    }

    editor.set_combo_state(ComboState::None);

    Ok(None)
}
