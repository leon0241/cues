//! # View Handler
//!
//! The home of Ratatui Rendering

use crate::{
    cues::cuestack::CueStack,
    editor::{Editor, Mode},
    model::full_model::Model,
    tui_core::layout::Sections,
};

use cue_tui_core::widgets::Widgets;
use ratatui::{
    Frame,
    layout::Position,
    widgets::{Row, Table},
};

/// Crude view function
pub fn view(model: &mut Model, frame: &mut Frame) {
    let (editor, cuestack): (&mut Editor, &mut CueStack) = model.full_mut();
    // Vec of ListItems (which comes from a decompiled list of Cues)
    let rows: Vec<Row> = cuestack
        .cues // Cues of the model
        .iter() // Iterator across cues
        .map(|i| i.get_row())
        .collect(); //maps to string name and transforms an iterator back to collection

    let table: Table = Widgets::cuestack_table(rows);

    let layout: Sections = Sections::new(frame);

    let title = Widgets::title();

    let current_cue = cuestack.get_current_cue();

    let cue_name = Widgets::cue_name(current_cue.map_or("", |f| f.0.name()));

    let command_line = Widgets::command_line();

    let status_line = Widgets::status_line(editor.mode().as_str(), editor.mode());

    let input_buffer_text: &str = editor.input_buffer();

    let input_buffer = Widgets::input_buffer(input_buffer_text);

    frame.render_widget(cue_name, layout.main.info.cue_name);

    // frame.render_widget(paragraph, frame.area());
    // frame.render_widget(mode, layout.status_line);
    frame.render_widget(title, layout.tab_bar);
    frame.render_widget(command_line, layout.status_bar.command_line);
    frame.render_widget(status_line, layout.status_bar.status_line);

    match editor.mode() {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        Mode::Normal | Mode::Visual => {}

        // Make the cursor visible and ask ratatui to put it at the specified coordinates after
        // rendering
        Mode::Insert => {
            let idx = u16::try_from(editor.cursor_idx()).unwrap_or_default();

            frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position can be controlled via the left and right arrow key
                layout.status_bar.input_buffer.x.wrapping_add(idx),
                // Move one line down, from the border to the input line
                layout.status_bar.input_buffer.y,
            ));
        }
    }

    frame.render_widget(input_buffer, layout.status_bar.input_buffer);

    frame.render_stateful_widget(table, layout.main.cues, &mut cuestack.current_state);
}
