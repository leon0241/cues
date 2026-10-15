use crate::model::{cuestack::CueStack, keymaps::{Message, NavigationCmd}};

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn navigation_handler(model: &mut CueStack, cmd: &NavigationCmd) -> color_eyre::Result<Option<Message>> {
    match cmd {
        NavigationCmd::LeftCol => {
            model.go_left()?;
        }
        NavigationCmd::PrevCue => {
            model.go_previous()?;
        }
        NavigationCmd::NextCue => {
            model.go_next()?;
        }
        NavigationCmd::RightCol => {
            model.go_right()?;
        }
    }
    Ok(None)
}
