use std::io::{self, Stdout, stdout};

use crate::{model::full_model::Model, view::view_handler::view};

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
