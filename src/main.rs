//! TUI Based Audio Cue playback tool
//! Very much WIP!

use cues::view::terminal::{Tui, create_terminal, run_loop};

use color_eyre::Result;

fn main() -> Result<()> {
    // Color_eyre error handling
    color_eyre::install()?;

    let tui: Tui = create_terminal()?;

    run_loop(tui)?;

    Ok(())
}
