use std::io::{self, Stdout, stdout};

use crate::{
    model::{
        cuestack::CueStack,
        editor::{Editor, RunningState},
        full_model::Model,
    },
    update::event_handler::{event_matcher, init_cues},
    view::view_handler::view,
};

use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    crossterm::{
        ExecutableCommand, execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};

pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;
pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    #[must_use]
    pub const fn new(terminal: CrosstermTerminal) -> Self {
        Self { terminal }
    }

    /// # Errors
    ///
    /// Will return `Err` terminal init doesn't work
    pub fn init_terminal() -> color_eyre::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` terminal restore doesn't work
    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    /// # Panics
    ///
    /// Emergency call if it does panic
    pub fn set_panic_hook(&self) {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = stdout().execute(LeaveAlternateScreen);
            let _ = disable_raw_mode();
            hook(panic_info);
        }));
    }

    /// # Errors
    ///
    /// Will error if something doesn't draw right
    pub fn draw(&mut self, model: &mut Model) -> io::Result<()> {
        self.terminal.draw(|frame| view(model, frame))?;
        Ok(())
    }

    /// # Errors
    ///
    /// Will error if it can't clear i guess
    pub fn clear(&mut self) -> color_eyre::Result<()> {
        Ok(self.terminal.clear()?)
    }
}

/// # Errors
///
/// TODO
pub fn create_terminal() -> color_eyre::Result<Tui> {
    let backend: CrosstermBackend<std::io::Stdout> = CrosstermBackend::new(std::io::stdout());
    let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    Tui::init_terminal()?;
    let mut tui = Tui::new(terminal);
    tui.set_panic_hook();
    tui.clear()?;

    Ok(tui)
}

/// # Errors
///
/// TODO
pub fn run_loop(mut tui: Tui) -> color_eyre::Result<()> {
    let mut model: Model = Model::new();
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
