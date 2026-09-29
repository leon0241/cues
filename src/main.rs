pub mod model;

pub mod update;

pub mod view;

use model::model::{CueStack, RunningState};

use update::update::{
    update,
    handle_event
};

use view::{
    view::view,
    terminal::Tui,
};

use ratatui::{backend::CrosstermBackend, Terminal};

use color_eyre::Result;

fn main() -> Result<()>{
    let mut model = CueStack::new();

    let backend = CrosstermBackend::new(std::io::stdout());
    let terminal = Terminal::new(backend)?;

    let mut tui = Tui::new(terminal).init_terminal()?;

    while model.running_state != RunningState::Done {
        tui.draw(|f| view(&mut model, f))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }
    Ok(())
}
