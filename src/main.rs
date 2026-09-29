pub mod model;

pub mod update;

pub mod view;

// pub mod util;

use model::{
    model::{CueStack, RunningState, Cue, FollowState},
    editor::{Editor, Mode}
};

use update::update::{
    update,
    handle_event
};

use view::{
    view::view,
    terminal::Tui,
};

// use util::logging::initialize_logging;

use ratatui::{backend::CrosstermBackend, Terminal};

use color_eyre::Result;

use cli_log::init_cli_log;


fn main() -> Result<()>{

    init_cli_log!();

    let mut model = CueStack::new();
    let mut editor = Editor::new();

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;

    let mut tui = Tui::new(terminal).init_terminal()?;

    model.set_items(
        vec![
            Cue::new(1, String::from("test"), 1, FollowState::None),
            Cue::new(2, String::from("test2"), 1, FollowState::None),
        ]
    );

    while model.running_state != RunningState::Done {
        tui.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;
        let mut current_mode = editor.get_mode();

        // Inside Normal Mode
        if current_mode == Mode::Normal {
            // Process events
            while current_msg.is_some() {
                (current_mode, current_msg) = update(&mut model, current_msg.unwrap());
            }
        }
    }

    Ok(())
}
