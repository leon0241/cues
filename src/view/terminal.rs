use std::io::{self, stdout, Stdout};

use crate::view::view::view;
use crate::model::cuestack::CueStack;

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

    pub fn init_terminal(&self) -> color_eyre::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.set_panic_hook();
        Ok(())
    }

    pub fn restore_terminal() -> color_eyre::Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn set_panic_hook(&self) {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            stdout().execute(LeaveAlternateScreen).unwrap();
            disable_raw_mode().unwrap();
            hook(panic_info);
        }));
    }

    pub fn draw(&mut self, model: &mut CueStack) -> io::Result<()> {
        self.terminal.draw(|frame| view(model, frame))?;
        Ok(())
    }
}
