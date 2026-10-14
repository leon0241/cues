use ratatui::crossterm::event::{KeyCode, KeyModifiers};

use crate::{model::cuestack::CueStack, update::keymaps_file::Message};

#[derive(Debug)]
pub enum EditorCmd {
    Quit,
    EnterInsertStart,
    EnterInsertEnd,
    EnterNormal,
    EnterVisual,
}

const fn editor_key(key: KeyCode, _modifier: KeyModifiers) -> Option<Message> {
    match key {
        KeyCode::Char('q') => Some(
            Message::Editor(EditorCmd::Quit)
        ),
        KeyCode::Char('i') => Some(
            Message::Editor(EditorCmd::EnterInsertStart)
        ),
        KeyCode::Char('a') => Some(
            Message::Editor(EditorCmd::EnterInsertEnd)
        ),
        KeyCode::Char('v') => Some(
            Message::Editor(EditorCmd::EnterVisual)
        ),
        _ => None,
    }
}

/// # Errors
///
/// Will return `Err` if cannot handle a key
pub fn editor_handler(model: &mut CueStack, cmd: &EditorCmd) -> color_eyre::Result<Option<Message>> {
    match cmd {
        EditorCmd::Quit => {
            Ok(Some(Message::Editor(EditorCmd::Quit)))
        }
        EditorCmd::EnterInsertStart => {
            model.edit_current_cell_start();

            Ok(Some(Message::Editor(EditorCmd::EnterInsertEnd)))
        }
        EditorCmd::EnterInsertEnd => {
            model.edit_current_cell_end();

            Ok(Some(Message::Editor(EditorCmd::EnterInsertEnd)))
        }
        EditorCmd::EnterNormal => {
            model.save_current_cell();

            Ok(Some(Message::Editor(EditorCmd::EnterNormal)))
        }
        EditorCmd::EnterVisual => {
            // model.set_mode(Mode::Visual);
            Ok(Some(Message::Editor(EditorCmd::EnterVisual)))
        }
    }
}


/// # Errors
///
/// Will return `Err` if cannot handle a key
pub const fn update_editor(editor: &mut Editor, msg: &EditorCmd) -> color_eyre::Result<Option<Message>> {
    match msg {
        EditorCmd::Quit => {
            editor.set_running_state(crate::model::editor::RunningState::Done);
        }
        EditorCmd::EnterInsertStart | EditorCmd::EnterInsertEnd => {
            editor.set_mode(Mode::Insert);
        }
        EditorCmd::EnterNormal => {
            editor.set_mode(Mode::Normal);
        }
        EditorCmd::EnterVisual => {
            editor.set_mode(Mode::Visual);
        }
    }
    Ok(None)
}
