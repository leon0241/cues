pub mod model;

pub mod update;

pub mod view;

// pub mod util;

use model::{
    cuestack::{CueStack},
    editor::{Editor, Mode, RunningState},
    full_model::{Model}
};

use update::{
    update::{ init_cues, update_normal, update_insert, update_visual},
    keymaps::{ handle_event }
};

use view::{
    terminal::Tui,
};

use ratatui::{backend::CrosstermBackend, Terminal};

use color_eyre::Result;

use crate::update::{keymaps::Message, update::update_editor};

fn main() -> Result<()>{
    // Color_eyre error handling
    color_eyre::install()?;

    let tui: Tui = create_terminal()?;

    _ = run_loop(tui);

    Ok(())
}

fn create_terminal() -> color_eyre::Result<Tui> {
    let backend: CrosstermBackend<std::io::Stdout> = CrosstermBackend::new(std::io::stdout());
    let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    let tui = Tui::new(terminal);
    _ = tui.init_terminal();

    Ok(tui)
}

fn run_loop(mut tui: Tui) -> color_eyre::Result<()> {
    let mut model: Model = Model::new(
        Editor::new(),
        CueStack::new()
        );
    // let mut model: CueStack = CueStack::new();
    model.cuestack_mut().set_default_sink()?;
    // maybe put mut back with insert/normal

    init_cues(model.cuestack_mut())?;

    while *model.editor().running_state() != RunningState::Done {

        // Draw (all logic in the View files)
        tui.draw(&mut model)?; // Calls view

        // Start on Normal mode (default)
        let mut current_mode = model.editor().get_mode();
        // Handle events and map to a Message
        let mut current_msg = handle_event(model.editor(), current_mode)?;

        // Inside Normal Mode
        match current_mode {
            Mode::Normal => {
                while let Some(ref message) = current_msg {
                    current_msg = update_normal(model.cuestack_mut(), message)?;

                    match current_msg {
                        None => {}
                        Some(Message::Editor(cmd)) => {
                            update_editor(model.editor_mut(), &cmd)?;
                            current_msg = None;
                        }
                        _ => {current_msg = None}
                    }
                }
            }
            Mode::Insert => {
                while let Some(ref message) = current_msg {
                    current_msg = update_insert(model.cuestack_mut(), message)?;

                    match current_msg {
                        None => {}
                        Some(Message::Editor(cmd)) => {
                            update_editor(model.editor_mut(), &cmd)?;
                            current_msg = None;
                        }
                        _ => {current_msg = None}
                    }
                }
            }
            Mode::Visual => {
                while let Some(message) = current_msg {
                    (current_mode, current_msg) = update_visual(model.cuestack_mut(), &message)?;
                }
            }
        }
    }
    Ok(())
}
