//! # View Handler
//!
//! The home of Ratatui Rendering

use crate::model::{
    cuestack::CueStack,
    editor::{Editor, Mode},
    full_model::Model,
};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Row, Table},
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

    let table: Table = init_table(rows);

    let layout: Sections = Sections::new(frame);

    let title = Line::from_iter([Span::from("Cuitui")])
        .bg(Color::DarkGray)
        .fg(Color::Black)
        .centered();

    let cue_name = Line::from_iter([Span::from(
        cuestack
            .get_current_cue()
            .map_or(String::new(), |f| f.0.name()),
    )])
    .bg(Color::DarkGray)
    .fg(Color::White);

    let command_line = StatusLine::style_command_line();

    let status_line = StatusLine::style_status_line(editor.mode().as_str(), editor.mode());

    let input_buffer_text: String = editor.input_buffer();

    let input_buffer = StatusLine::style_input_buffer(&input_buffer_text);

    frame.render_widget(cue_name, layout.cue_info.cue_name);

    // frame.render_widget(paragraph, frame.area());
    // frame.render_widget(mode, layout.status_line);
    frame.render_widget(title, layout.tab_line);
    frame.render_widget(command_line, layout.status_lines.command_line);
    frame.render_widget(status_line, layout.status_lines.status_line);

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
                layout.status_lines.input_buffer.x.wrapping_add(idx),
                // Move one line down, from the border to the input line
                layout.status_lines.input_buffer.y,
            ));
        }
    }

    frame.render_widget(input_buffer, layout.status_lines.input_buffer);

    frame.render_stateful_widget(table, layout.middle, &mut cuestack.current_state);
}

struct StatusLine {
    pub command_line: Rect,
    pub input_buffer: Rect,
    pub status_line: Rect,
}

impl StatusLine {
    pub fn init(space: Rect) -> Self {
        // Set up cmd and status line
        let verticals: Layout = Layout::vertical([
            // Command Line of height 1
            Constraint::Length(1),
            // Status Line of height 1
            Constraint::Length(1),
        ])
        .spacing(0);

        let command_line: Layout = Layout::horizontal([
            // Text saying: "CMD: "
            Constraint::Length(5),
            // Input buffer
            Constraint::Fill(1),
        ])
        .spacing(0);

        let [command_line_bar, status_line_bar] = space.layout(&verticals);

        let [cmd_text, input_buffer] = command_line_bar.layout(&command_line);

        Self {
            command_line: command_line_bar,
            input_buffer,
            status_line: status_line_bar,
        }
    }

    pub fn style_command_line() -> Line<'static> {
        Line::from_iter([Span::from("CMD: ")])
            .bg(Color::DarkGray)
            .fg(Color::White)
    }

    pub fn style_status_line(text: &str, mode: Mode) -> Line<'_> {
        let mode_span = Span::from(text).bold().patch_style(mode.style());

        Line::from_iter([mode_span]).bg(Color::Black)
    }

    pub fn style_input_buffer(text: &str) -> Line<'_> {
        Line::from_iter([Span::from(text)])
            .bg(Color::DarkGray)
            .fg(Color::White)
    }
}

struct CueInfo {
    pub cue_name: Rect,
    pub duration: Rect,
}

impl CueInfo {
    pub fn init(space: Rect) -> Self {
        // Set up cmd and status line
        let verticals: Layout = Layout::vertical([
            // Name
            Constraint::Length(1),
            // Rest of it
            Constraint::Fill(1),
        ])
        .spacing(0);

        let [cue_name, duration] = space.layout(&verticals);

        Self { cue_name, duration }
    }
}

#[allow(dead_code)]
struct Sections {
    pub tab_line: Rect,
    pub main: Rect,
    pub cue_info: CueInfo,
    pub status_lines: StatusLine,
    pub left: Rect,
    pub middle: Rect,
    pub right: Rect,
}

impl Sections {
    pub fn new(frame: &Frame) -> Self {
        let vertical = Layout::vertical([
            // Title
            Constraint::Length(1),
            // Cuestack
            Constraint::Fill(1),
            // CMD/Status line
            Constraint::Length(7),
            // CMD/Status line
            Constraint::Length(2),
        ])
        .spacing(0);

        let horizontal = Layout::horizontal([
            Constraint::Percentage(5),
            Constraint::Fill(1),
            Constraint::Percentage(5),
        ])
        .spacing(1);

        let [tab_line, main, cue_info_box, bottom] = frame.area().layout(&vertical);
        let [left, middle, right] = main.layout(&horizontal);

        let cue_info = CueInfo::init(cue_info_box);

        let status_lines = StatusLine::init(bottom);

        Self {
            tab_line,
            main,
            cue_info,
            status_lines,
            left,
            middle,
            right,
        }
    }
}

/// Creates a table for rendering a `CueStack`
///
/// Takes a `Vec` of `Rows` which are taken by iterating the `CueStack`
#[must_use]
pub fn init_table(rows: Vec<Row>) -> Table {
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
