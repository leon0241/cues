use crate::model::{
    cuestack::CueStack,
    keymaps::{Message, NavigationCmd},
};

/// # Handler for movement within the cuestack
pub fn navigation_handler(model: &mut CueStack, cmd: &NavigationCmd) -> Option<Message> {
    match cmd {
        NavigationCmd::LeftCol => {
            model.go_left();
        }
        NavigationCmd::PrevCue => {
            model.go_previous();
        }
        NavigationCmd::NextCue => {
            model.go_next();
        }
        NavigationCmd::RightCol => {
            model.go_right();
        }
    }
    None
}
