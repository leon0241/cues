pub mod model;

pub mod update;

pub mod view;

// pub mod util;

use model::{
    cuestack::CueStack,
    editor::{Editor, RunningState},
    full_model::Model,
};

use update::event_handler::init_cues;

use view::terminal::Tui;

use ratatui::{Terminal, backend::CrosstermBackend};

use color_eyre::Result;

use crate::update::event_handler::event_matcher;

fn main() -> Result<()> {
    // Color_eyre error handling
    color_eyre::install()?;

    let tui: Tui = create_terminal()?;

    run_loop(tui)?;

    Ok(())
}

fn create_terminal() -> color_eyre::Result<Tui> {
    let backend: CrosstermBackend<std::io::Stdout> = CrosstermBackend::new(std::io::stdout());
    let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    Tui::init_terminal()?;
    let mut tui = Tui::new(terminal);
    tui.set_panic_hook();
    tui.clear()?;

    Ok(tui)
}

fn run_loop(mut tui: Tui) -> color_eyre::Result<()> {
    let mut model: Model = Model::new(Editor::new(), CueStack::new());
    // let mut model: CueStack = CueStack::new();
    model.cuestack_mut().set_default_sink()?;
    // maybe put mut back with insert/normal

    init_cues(model.cuestack_mut())?;

    while *model.editor().running_state() != RunningState::Done {
        // Draw (all logic in the View files)
        tui.draw(&mut model)?; // Calls view
        //
        event_matcher(&mut model)?;
    }
    Ok(())
}
