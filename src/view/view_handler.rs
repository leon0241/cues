use crate::model::{cuestack::CueStack, editor::Editor, full_model::Model};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Row, Table},
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

    let title = Line::from_iter([Span::from("Cuitui")])
        .bg(Color::DarkGray)
        .fg(Color::Black)
        .centered();

    let status_line = Line::from_iter([
        Span::from(editor.mode_str()).bold().bg(Color::LightBlue),
        Span::from(editor.combo_str()).fg(Color::White),
    ])
    .bg(Color::Black);

    let command_line = Line::from_iter([Span::from("CMD: ")])
        .bg(Color::DarkGray)
        .fg(Color::Black);

    // let mode = Paragraph::new(editor.mode_str())
    //     .style(Color::White)
    //     .block(Block::new().borders(Borders::LEFT).borders(Borders::RIGHT))
    //     .alignment(Alignment::Left);
    // let combo = Paragraph::new(editor.combo_str())
    //     .style(Color::White)
    //     .block(Block::new().borders(Borders::LEFT).borders(Borders::RIGHT))
    //     .alignment(Alignment::Center);
    //
    let layout: Sections = Sections::new(frame);

    // frame.render_widget(paragraph, frame.area());
    // frame.render_widget(mode, layout.status_line);
    frame.render_widget(title, layout.tab_line);
    frame.render_widget(command_line, layout.command_line);
    frame.render_widget(status_line, layout.status_line);

    frame.render_stateful_widget(table, layout.middle, &mut cuestack.current_cue);
}

#[allow(dead_code)]
struct Sections {
    pub tab_line: Rect,
    pub main: Rect,
    pub command_line: Rect,
    pub status_line: Rect,
    pub left: Rect,
    pub middle: Rect,
    pub right: Rect,
}

impl Sections {
    pub fn new(frame: &Frame) -> Self {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .spacing(0);

        let bottom_lines =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).spacing(0);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(5),
            Constraint::Fill(1),
            Constraint::Percentage(5),
        ])
        .spacing(1);

        let [tab_line, main, bottom] = frame.area().layout(&vertical);
        let [command_line, status_line] = bottom.layout(&bottom_lines);
        let [left, middle, right] = main.layout(&horizontal);

        Self {
            tab_line,
            main,
            command_line,
            status_line,
            left,
            middle,
            right,
        }
    }
}

#[must_use]
pub fn init_table(rows: Vec<Row>) -> Table {
    // Setting Header
    let header: Row = Row::new(["", "", "Num", "Name", "Pre", "Duration", "Post", "F"])
        .style(Style::new().bold())
        .bottom_margin(1);

    // Setting Column Widths
    let widths: [Constraint; 8] = [
        Constraint::Length(2),
        Constraint::Length(2),
        Constraint::Length(5),
        Constraint::Fill(1),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(10),
        Constraint::Length(2),
    ];

    // Creating New Table
    Table::new(rows, widths)
        .header(header)
        .block(Block::new().title("Cuestack"))
        .row_highlight_style(Style::new().reversed())
        .highlight_symbol(">>")
        .column_highlight_style(Color::Blue)
}
