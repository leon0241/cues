//! # Terminal
//!
//! Wrapper for Crossterm and its associated functions

use std::io::{Stdout, stdout};

use crate::{
    model::{editor::RunningState, full_model::Model},
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

/// Shorthand Type for a terminal backend
pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;

/// Main Tui struct that terminal based functions are called from
pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    /// Creates a new Tui
    ///
    /// # Examples
    /// ```
    /// use cues::view::terminal::Tui;
    /// use ratatui::{backend::CrosstermBackend, Terminal};
    ///
    /// let backend = CrosstermBackend::new(std::io::stdout());
    /// // don't unwrap in prod!
    /// let terminal = Terminal::new(backend).unwrap();
    ///
    /// let new_tui = Tui::new(terminal);
    /// ```
    #[must_use]
    pub const fn new(terminal: CrosstermTerminal) -> Self {
        Self { terminal }
    }

    /// Preps terminal for rendering Ratatui by clearing the screen and enabling raw mode.
    ///
    /// # Errors
    ///
    /// Will return `Err` if either `EnterAlternateScreen` or `enable_raw_mode()` fails
    pub fn init_terminal() -> color_eyre::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(())
    }

    /// Restores terminal to normal
    ///
    /// # Errors
    ///
    /// Will return `Err` if either `LeaveAlternateScreen` or `disable_raw_mode()` fails
    ///
    /// # Examples
    /// ```
    /// use cues::view::terminal::Tui;
    ///
    /// // never unwrap in prod!
    /// Tui::restore_terminal().unwrap();
    /// ```
    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    /// Sets a panic hook so weird stuff doesn't happen if the software panics such as getting
    /// trapped in raw mode.
    ///
    /// ..probably
    ///
    /// # Panics
    ///
    /// Takes over hook to leave alternate screen and disable raw mode
    ///
    /// # Examples
    /// ```
    /// use cues::view::terminal::Tui;
    ///
    /// // never unwrap in prod!
    /// Tui::set_panic_hook();
    /// ```
    pub fn set_panic_hook() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = stdout().execute(LeaveAlternateScreen);
            let _ = disable_raw_mode();
            hook(panic_info);
        }));
    }

    /// Main draw loop for Ratatui rendering.
    ///
    /// Takes a `Model` and calls `cues::view::view()` every frame.
    ///
    /// # Errors
    ///
    /// Will error if view propagates back
    ///
    /// # Examples
    /// ```
    /// use cues::view::terminal::Tui;
    /// use cues::model::full_model::Model;
    ///
    /// use ratatui::{backend::CrosstermBackend, Terminal};
    ///
    /// let mut model = Model::new();
    ///
    /// let backend = CrosstermBackend::new(std::io::stdout());
    /// // don't unwrap in prod!
    /// let terminal = Terminal::new(backend).unwrap();
    ///
    /// let mut tui = Tui::new(terminal);
    ///
    /// tui.draw(&mut model);
    /// ```
    pub fn draw(&mut self, model: &mut Model) -> color_eyre::Result<()> {
        self.terminal.draw(|frame| view(model, frame))?;
        Ok(())
    }

    /// Clears the terminal
    ///
    /// # Errors
    ///
    /// Will error if it can't clear i guess
    ///
    /// # Examples
    /// ```
    /// use cues::view::terminal::Tui;
    ///
    /// use ratatui::{backend::CrosstermBackend, Terminal};
    ///
    /// let backend = CrosstermBackend::new(std::io::stdout());
    /// // don't unwrap in prod!
    /// let terminal = Terminal::new(backend).unwrap();
    ///
    /// let mut tui = Tui::new(terminal);
    ///
    /// tui.clear();
    /// ```
    pub fn clear(&mut self) -> color_eyre::Result<()> {
        Ok(self.terminal.clear()?)
    }
}

/// Creates a new tui using stdout
///
/// # Errors
///
/// Errors if shit goes wrong (endless propagations)
pub fn create_terminal() -> color_eyre::Result<Tui> {
    let backend: CrosstermBackend<std::io::Stdout> = CrosstermBackend::new(std::io::stdout());
    let terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    Tui::init_terminal()?;
    let mut tui = Tui::new(terminal);
    Tui::set_panic_hook();
    tui.clear()?;

    Ok(tui)
}

/// Main run loop of the program
///
/// # Errors
///
/// endless propagation
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
