use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::{Backend, CrosstermBackend},
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
    pub fn new(terminal: CrosstermTerminal) -> Self {
        Self {terminal}
    }

    pub fn init_terminal(self) -> color_eyre::Result<CrosstermTerminal> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Self::set_panic_hook();
        Ok(self.terminal)
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn set_panic_hook() {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            hook(panic_info);
        }));
    }
}
