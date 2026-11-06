use cue_editor::Mode;
use ratatui::{
    layout::Constraint,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Row, Table},
};

pub struct Widgets {}

impl Widgets {
    /// Creates a table for rendering a `CueStack`
    ///
    /// Takes a `Vec` of `Rows` which are taken by iterating the `CueStack`
    #[must_use]
    pub fn cuestack_table(rows: Vec<Row<'_>>) -> Table<'_> {
        // Setting Header
        let header: Row = Row::new([
            "", "", "Num", "Name", "Target", "Pre", "Duration", "Post", "F",
        ])
        .style(Style::new().bold())
        .bottom_margin(1);

        // Setting Column Widths
        let widths: [Constraint; 9] = [
            Constraint::Length(2),  // Now Playing
            Constraint::Length(2),  // Icon
            Constraint::Length(5),  // Number
            Constraint::Fill(1),    // Name
            Constraint::Length(5),  // Follow
            Constraint::Length(10), // Pre
            Constraint::Length(10), // Duration
            Constraint::Length(10), // Post
            Constraint::Length(2),  // Follow
        ];

        // Creating New Table
        Table::new(rows, widths)
            .header(header)
            .block(Block::new().title("Cuestack"))
            .row_highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .column_highlight_style(Color::Blue)
    }

    #[must_use]
    pub fn title() -> Line<'static> {
        Line::from_iter([Span::from("Cuitui")])
            .bg(Color::DarkGray)
            .fg(Color::Black)
            .centered()
    }

    #[must_use]
    pub fn cue_name(name: &str) -> Line<'_> {
        Line::from_iter([Span::from(name)])
            .bg(Color::DarkGray)
            .fg(Color::White)
    }

    #[must_use]
    pub fn command_line() -> Line<'static> {
        Line::from_iter([Span::from("CMD: ")])
            .bg(Color::DarkGray)
            .fg(Color::White)
    }

    #[must_use]
    pub fn status_line(text: &str, mode: Mode) -> Line<'_> {
        let mode_span = Span::from(text).bold();
        // let mode_span = Span::from(text).bold().patch_style(mode.style());

        Line::from_iter([mode_span]).bg(Color::Black)
    }

    #[must_use]
    pub fn input_buffer(text: &str) -> Line<'_> {
        Line::from_iter([Span::from(text)])
            .bg(Color::DarkGray)
            .fg(Color::White)
    }
}
