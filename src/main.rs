pub mod model;

pub mod update;

pub mod view;

// pub mod util;

use model::{
    cuestack::{CueStack},
    editor::{Editor, Mode, RunningState}
};

use update::{
    update::{ init_cues, update, },
    keymaps::{ handle_event }
};

use view::{
    terminal::Tui,
};

use ratatui::{backend::CrosstermBackend, Terminal};

use color_eyre::Result;

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
    let mut model: CueStack = CueStack::new();
    model.set_default_sink()?;
    // maybe put mut back with insert/normal

    let editor: Editor = Editor::new();

    init_cues(&mut model)?;

    while model.running_state != RunningState::Done {
        // Draw (all logic in the View files)
        tui.draw(&mut model)?; // Calls view

        // Start on Normal mode (default)
        let mut current_mode = editor.get_mode();
        // Handle events and map to a Message
        let mut current_msg = handle_event(&model, current_mode)?;

        // Inside Normal Mode
        if current_mode == Mode::Normal {
            // Process events
            while let Some(message) = current_msg {
                (current_mode, current_msg) = update(&mut model, &message)?;
            }
        }
    }
    Ok(())
}
