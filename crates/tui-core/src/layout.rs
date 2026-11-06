use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

#[allow(dead_code)]
pub struct Sections {
    pub tab_bar: Rect,
    pub main: Main,
    pub status_bar: StatusBar,
}

impl Sections {
    #[must_use]
    pub fn new(frame: &Frame) -> Self {
        let vertical = Layout::vertical([
            // Title
            Constraint::Length(1),
            // Cuestack
            Constraint::Fill(1),
            // CMD/Status line
            Constraint::Length(2),
        ])
        .spacing(0);

        let [tab_bar, main_rect, status_bar_rect] = frame.area().layout(&vertical);

        let status_bar: StatusBar = StatusBar::init(status_bar_rect);

        let main: Main = Main::init(main_rect);

        Self {
            tab_bar,
            main,
            status_bar,
        }
    }
}

pub struct StatusBar {
    pub command_line: Rect,
    pub input_buffer: Rect,
    pub status_line: Rect,
}

impl StatusBar {
    #[must_use]
    pub fn init(space: Rect) -> Self {
        // Vertical bars of length 1 to separate command line and status line
        let [command_line, status_line] = space.layout(
            &Layout::vertical([
                // Command Line of height 1
                Constraint::Length(1),
                // Status Line of height 1
                Constraint::Length(1),
            ])
            .spacing(0),
        );

        // Horizontal Spacing to separate text box from text saying "CMD: "
        let [_, input_buffer] = command_line.layout(
            &Layout::horizontal([
                // Text saying: "CMD: "
                Constraint::Length(5),
                // Input buffer
                Constraint::Fill(1),
            ])
            .spacing(0),
        );

        Self {
            command_line,
            input_buffer,
            status_line,
        }
    }
}

pub struct Main {
    pub cues: Rect,
    pub info: CueInfo,
}

impl Main {
    #[must_use]
    pub fn init(space: Rect) -> Self {
        let [_left, middle, _right] = space.layout(
            &Layout::horizontal([
                Constraint::Percentage(5),
                Constraint::Fill(1),
                Constraint::Percentage(5),
            ])
            .spacing(1),
        );

        let [cues, info_area] =
            middle.layout(&Layout::vertical([Constraint::Fill(1), Constraint::Max(10)]).spacing(1));

        let info: CueInfo = CueInfo::init(info_area);

        Self { cues, info }
    }
}

pub struct CueInfo {
    pub cue_name: Rect,
    pub duration: Rect,
}

impl CueInfo {
    #[must_use]
    pub fn init(space: Rect) -> Self {
        let [cue_name, duration] = space.layout(
            &Layout::vertical([
                // Name
                Constraint::Length(1),
                // Rest of it
                Constraint::Fill(1),
            ])
            .spacing(0),
        );

        Self { cue_name, duration }
    }
}
