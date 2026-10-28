use ratatui::{
    style::{Color, Style},
    widgets::Table,
};

use crate::model::editor::Mode;

impl Mode {
    /// Creates styles for each mode
    ///
    /// # Examples
    /// ```
    /// use ratatui::style::{Color, Style};
    /// use cues::model::editor::Mode;
    ///
    /// let style = Mode::Insert.style();
    /// ```
    #[must_use]
    pub const fn style(self) -> Style {
        match self {
            Self::Insert => Style::new().bg(Color::Blue).fg(Color::Black),
            Self::Normal => Style::new().bg(Color::Green).fg(Color::Black),
            Self::Visual => Style::new().bg(Color::Magenta).fg(Color::Black),
        }
    }
}
