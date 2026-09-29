use crate::model::model::CueStack;

use ratatui::{
    layout::Alignment,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn view(model: &CueStack, frame: &mut Frame) {
    frame.render_widget(
        Paragraph::new("Test"),
        frame.area()
    );
}
