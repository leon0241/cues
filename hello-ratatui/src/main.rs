pub mod app;

pub mod event;

pub mod ui;

pub mod tui;

pub mod update;

use app::App;
use color_eyre::Result;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;


fn main() -> Result<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;
    //
    // #[test]
    // fn render() {
    //     let app = App::default();
    //     let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
    //
    //     app.render(buf.area, &mut buf);
    //
    //     let mut expected = Buffer::with_lines(vec![
    //         "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
    //         "┃                    Value: 0                    ┃",
    //         "┃                                                ┃",
    //         "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
    //     ]);
    //     let title_style = Style::new().bold();
    //     let counter_style = Style::new().yellow();
    //     let key_style = Style::new().blue().bold();
    //     expected.set_style(Rect::new(14, 0, 22, 1), title_style);
    //     expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
    //     expected.set_style(Rect::new(13, 3, 6, 1), key_style);
    //     expected.set_style(Rect::new(30, 3, 7, 1), key_style);
    //     expected.set_style(Rect::new(43, 3, 4, 1), key_style);
    //
    //     assert_eq!(buf, expected);
    // }
    //
    // #[test]
    // fn handle_key_event() {
    //     let mut app = App::default();
    //     app.handle_key_event(KeyCode::Right.into()).unwrap();
    //     assert_eq!(app.counter, 1);
    //
    //     app.handle_key_event(KeyCode::Left.into()).unwrap();
    //     assert_eq!(app.counter, 0);
    //
    //     let mut app = App::default();
    //     app.handle_key_event(KeyCode::Char('q').into()).unwrap();
    //     assert!(app.exit);
    // }
    //
    // #[test]
    // #[should_panic(expected = "attempt to subtract with overflow")]
    // fn handle_key_event_panic() {
    //     let mut app = App::default();
    //     let _ = app.handle_key_event(KeyCode::Left.into());
    // }
    //
    // #[test]
    // fn handle_key_event_overflow() {
    //     let mut app = App::default();
    //     assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
    //     assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
    //     assert_eq!(
    //         app.handle_key_event(KeyCode::Right.into())
    //             .unwrap_err()
    //             .to_string(),
    //         "counter overflow"
    //     );
    // }
}
