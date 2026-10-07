use std::io::{self, stdout, Stdout};

use crate::view::view::view;
use crate::model::cuestack::CueStack;

use ratatui::{
    backend::{CrosstermBackend},
    crossterm::{
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
        ExecutableCommand,
    },
    Terminal,
};


pub type CrosstermTerminal = Terminal<CrosstermBackend<Stdout>>;
pub struct Tui {
    terminal: CrosstermTerminal,
}

impl Tui {
    #[must_use]
    pub const fn new(terminal: CrosstermTerminal) -> Self {
        Self {terminal}
    }

    /// # Errors
    ///
    /// Will return `Err` terminal init doesn't work
    pub fn init_terminal(&self) -> color_eyre::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.set_panic_hook();
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
            let _ = stdout(). execute(LeaveAlternateScreen);
            let _ = disable_raw_mode();
            hook(panic_info);
        }));
    }

    /// # Errors
    ///
    /// Will error if something doesn't draw right
    pub fn draw(&mut self, model: &mut CueStack) -> io::Result<()> {
        self.terminal.draw(|frame| view(model, frame))?;
        Ok(())
    }
}
