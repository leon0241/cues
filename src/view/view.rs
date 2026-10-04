use crate::model::cuestack::CueStack;

use ratatui::{
    layout::{Constraint, Rect, Alignment},
    style::{Color, Style, Stylize},
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

pub fn view(model: &mut CueStack, frame: &mut Frame) {
    // Vec of ListItems (which comes from a decompiled list of Cues)
    let rows: Vec<Row> = model
        .cues // Cues of the model
        .iter() // Iterator across cues
        .map(|i| i.get_row()) 
        .collect(); //maps to string name and transforms an iterator back to collection

    let table: Table = init_table(rows);

    frame.render_stateful_widget(table, frame.area(), &mut model.current_cue);
}

pub fn init_table(rows: Vec<Row>) -> Table {
    // Setting Header
    let header: Row = Row::new(["P", "Number", "Name", "Duration"])
        .style(Style::new().bold())
        .bottom_margin(1);

    // Setting Column Widths
    let widths: [Constraint; 4] = [
        Constraint::Percentage(5),
        Constraint::Percentage(10),
        Constraint::Percentage(45),
        Constraint::Percentage(40),
    ];

    // Creating New Table
    Table::new(rows, widths)
        .header(header)
        .block(Block::new().title("Cue Stack"))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol(">>")
}
