use crate::model::{
    cues::cue::CueColumn,
    cuestack::CueStack,
    keymaps::{CueCmd, Message, ModKeyCmd}
};


/// # Errors
///
/// TODO
pub fn modkey_handler(model: &mut CueStack, cmd: &ModKeyCmd) -> color_eyre::Result<Option<Message>> {

    const fn match_inner(cmd: CueCmd) -> Option<CueColumn> {
        match cmd {
            CueCmd::Cue => None,
            CueCmd::CueNum => Some(CueColumn::Number),
            CueCmd::Name => Some(CueColumn::Name),
            CueCmd::PreWait => Some(CueColumn::PreWait),
            CueCmd::PostWait => Some(CueColumn::PostWait),
            CueCmd::Follow => Some(CueColumn::Follow)
        }
    }

    match cmd {
        ModKeyCmd::Yank(cue_cmd) => {
            if *cue_cmd == CueCmd::Cue {
                    model.copy_cue();
            } else if let Some(i) = match_inner(*cue_cmd) {
                model.copy_cell(&i);
            }
        },
        ModKeyCmd::Delete(cue_cmd) => {
            if *cue_cmd == CueCmd::Cue {
                    model.delete_cue()?;
            } else if let Some(i) = match_inner(*cue_cmd) {
                model.delete_cell(&i)?;
            }
        },
        ModKeyCmd::Change(cue_cmd) => {
            if *cue_cmd == CueCmd::Cue {
                model.clear_cue()?;
            } else if let Some(i) = match_inner(*cue_cmd) {
                model.change_cell(&i)?;
            }
        },

        ModKeyCmd::Create(_cue_cmd) => {
            { }
        }
    }

    Ok(None)
}
