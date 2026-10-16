use crate::model::{
    cuestack::CueStack,
    editor::{Editor, Mode},
    full_model::Model,
};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Position},
    style::{Color, Style},
    widgets::{Block, Paragraph, Row, Table},
};

pub fn view(model: &mut Model, frame: &mut Frame) {
    let (editor, cuestack): (&mut Editor, &mut CueStack) = model.full_mut();
    // Vec of ListItems (which comes from a decompiled list of Cues)
    let rows: Vec<Row> = cuestack
        .cues // Cues of the model
        .iter() // Iterator across cues
        .map(|i| i.get_row())
        .collect(); //maps to string name and transforms an iterator back to collection

    let table: Table = init_table(rows);

    let paragraph = Paragraph::new(editor.mode_str())
        .style(Color::White)
        .alignment(Alignment::Center);
    let combo = Paragraph::new(editor.combo_str())
        .style(Color::White)
        .alignment(Alignment::Center);

    let layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);
    let [help_area, input_area, messages_area] = frame.area().layout(&layout);

    // frame.render_widget(paragraph, frame.area());
    frame.render_widget(combo, frame.area());

    frame.render_stateful_widget(table, frame.area(), &mut cuestack.current_cue);

    match editor.mode() {
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        Mode::Normal | Mode::Visual => {}
        // Make the cursor visible and ask ratatui to put it at the specified coordinates after
        // rendering
        #[expect(clippy::cast_possible_truncation)]
        Mode::Insert => frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position can be controlled via the left and right arrow key
            input_area.x + editor.character_index() as u16 + 1,
            // Move one line down, from the border to the input line
            // TODO: this probably isn't right
            input_area.y.wrapping_add(1),
        )),
    }
}

#[must_use]
pub fn init_table(rows: Vec<Row>) -> Table {
    // Setting Header
    let header: Row = Row::new(["P", "Number", "Name", "Duration"])
        .style(Style::new().bold())
        .bottom_margin(1);

    // Setting Column Widths
    let widths: [Constraint; 5] = [
        Constraint::Percentage(5),
        Constraint::Percentage(5),
        Constraint::Percentage(10),
        Constraint::Percentage(40),
        Constraint::Percentage(40),
    ];

    // Creating New Table
    Table::new(rows, widths)
        .header(header)
        .block(Block::new().title("Cue Stack"))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol(">>")
        .column_highlight_style(Color::Blue)
}
