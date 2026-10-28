//! # Mode
//!
//! Handler for the editor mode

/// Current Editor Mode
///
/// These follow the same modes as Vim
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    /// Normal - Movements and such
    #[default]
    Normal,
    /// Insert - text input. For this application Insert mode
    /// locks input within the command line
    Insert,
    /// Visual - Selection of cells / cues
    Visual,
}

impl Mode {
    /// Returns a string variant of the mode
    ///
    /// Used for rendering to Ratatui
    ///
    /// # Examples
    /// ```
    /// use cues::model::editor::Mode;
    ///
    /// assert_eq!(Mode::Normal.as_str(), "Normal");
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => " NORMAL ",
            Self::Insert => " INSERT ",
            Self::Visual => " VISUAL ",
        }
    }
}
