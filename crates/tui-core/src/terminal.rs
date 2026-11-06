//! # Terminal
//!
//! Wrapper for Crossterm and its associated functions

use std::io::{Stdout, stdout};

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
    #[must_use]
    pub const fn new(terminal: CrosstermTerminal) -> Self {
        Self { terminal }
    }

    #[must_use]
    pub const fn terminal(&mut self) -> &mut CrosstermTerminal {
        &mut self.terminal
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
    pub fn set_panic_hook() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = stdout().execute(LeaveAlternateScreen);
            let _ = disable_raw_mode();
            hook(panic_info);
        }));
    }

    /// Clears the terminal
    ///
    /// # Errors
    ///
    /// Will error if it can't clear i guess
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
