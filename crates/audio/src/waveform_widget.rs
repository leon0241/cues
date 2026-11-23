//! Waveform Widget
//!
//! A horizontally scrolling implementation of a Ratatui List

use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

// Allow unused import of `ratatui::List` to -steal- reference the source code
#[allow(unused_imports)]
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Text, ToText},
    widgets::{Block, BlockExt, Cell, List, StatefulWidget, Widget},
};

pub struct WaveformWidget<'a> {
    samples: Vec<Vec<char>>,
    bounds: (usize, usize),
    style: Style,
    block: Option<Block<'a>>,
    scroll_padding: usize,
}

#[derive(Default)]
pub struct WaveformState {
    pub offset: usize,
    pub selected: Option<usize>,
    pub timestamp: usize,
}

impl WaveformState {
    #[must_use]
    pub const fn with_offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    #[must_use]
    pub const fn with_selected(mut self, selected: Option<usize>) -> Self {
        self.selected = selected;
        self
    }

    #[must_use]
    pub const fn with_timestamp(mut self, timestamp: usize) -> Self {
        self.timestamp = timestamp;
        self
    }

    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    #[must_use]
    pub const fn offset_mut(&mut self) -> &mut usize {
        &mut self.offset
    }

    #[must_use]
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub const fn selected_mut(&mut self) -> &mut Option<usize> {
        &mut self.selected
    }

    pub const fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    pub fn scroll_next(&mut self) {
        let next = self.selected.map_or(0, |i| i.saturating_add(1));
        self.select(Some(next));
    }

    pub fn scroll_previous(&mut self) {
        let previous = self.selected.map_or(usize::MAX, |i| i.saturating_sub(1));
        self.select(Some(previous));
    }

    pub const fn scroll_first(&mut self) {
        self.select(Some(0));
    }

    pub const fn scroll_last(&mut self) {
        self.select(Some(usize::MAX));
    }

    pub fn scroll_left_by(&mut self, amount: u16) {
        let selected = self.selected.unwrap_or_default();
        self.select(Some(selected.saturating_add(amount.into())));
    }

    pub fn scroll_right_by(&mut self, amount: u16) {
        let selected = self.selected.unwrap_or_default();
        self.select(Some(selected.saturating_sub(amount.into())));
    }

    #[must_use]
    pub const fn timestamp(&self) -> &usize {
        &self.timestamp
    }

    pub const fn timestamp_mut(&mut self) -> &mut usize {
        &mut self.timestamp
    }

    pub const fn set_timestamp(&mut self, timestamp: usize) {
        self.timestamp = timestamp;
    }

    pub const fn timestamp_on_selected(&mut self) {
        if let Some(i) = self.selected {
            self.timestamp = i;
        }
    }

    pub const fn recalibrate_add(&mut self, to_add: usize) {
        let half = to_add.saturating_div(2);
        self.offset = self.offset.saturating_add(half);
        if let Some(i) = self.selected {
            self.selected = Some(i.saturating_add(half));
        }
    }

    pub const fn recalibrate_sub(&mut self, to_sub: usize) {
        let half = to_sub.saturating_div(2);
        self.offset = self.offset.saturating_sub(half);
        if let Some(i) = self.selected {
            self.selected = Some(i.saturating_sub(half));
        }
    }
}

impl StatefulWidget for WaveformWidget<'_> {
    type State = WaveformState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(&self, area, buf, state);
    }
}

impl StatefulWidget for &WaveformWidget<'_> {
    type State = WaveformState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, Style::new().bg(Color::Blue));

        // Set the block
        self.block.as_ref().render(area, buf);
        let waveform_area = self.block.inner_if_some(area);
        // Nothing if it's empty
        if waveform_area.is_empty() {
            return;
        }

        // Width of the area
        let width: usize = waveform_area.width.into();

        // Selected cue, or first cue if nothing selected
        let selected = state.selected().unwrap_or(0);

        // Indexes that are currently visible on the screen
        let (first_visible_index, last_visible_index) =
            self.calculate_offset(selected, state.offset, width);

        state.offset = first_visible_index;
        state.selected = Some(selected.min(last_visible_index.saturating_sub(1)));

        // Slice from offset plus the width
        let visible_box = self
            .samples
            .get(first_visible_index..last_visible_index)
            .map_or_default(|f| f);

        // let transposed = transpose_samples(visible_box);
        self.render_columns(waveform_area, buf, state, visible_box);
    }
}

impl WaveformWidget<'_> {
    #[must_use]
    pub fn new(samples: Vec<Vec<char>>) -> Self {
        Self {
            samples,
            block: None,
            style: Style::default(),
            bounds: (0, 0),
            scroll_padding: 10,
        }
    }

    fn render_columns(
        &self,
        area: Rect,
        buf: &mut Buffer,
        state: &WaveformState,
        values: &[Vec<char>],
    ) {
        for (col_idx, strip) in values.iter().enumerate() {
            let x = area.x.saturating_add(col_idx.try_into().unwrap_or(50));

            let height = area.top();
            let row_area = Rect {
                x,
                y: height,
                width: 1,
                height: area.height,
            };

            self.render_strip_cells(buf, strip, row_area);

            if *state.timestamp() == state.offset().saturating_add(col_idx) {
                buf.set_style(row_area, Style::new().bg(Color::Magenta).fg(Color::Black));
            }

            if let Some(s) = state.selected()
                && s == state.offset().saturating_add(col_idx)
            {
                buf.set_style(row_area, Style::new().bg(Color::Green).fg(Color::Black));
            }
        }
    }

    fn render_strip_cells(&self, buf: &mut Buffer, cells: &[char], row_area: Rect) {
        for (column, current_char) in cells.iter().enumerate() {
            let new_y = row_area.y.saturating_add(column.try_into().unwrap_or(0));
            let area = Rect::new(row_area.x, new_y, 1, 1);

            buf.set_style(area, self.style);

            // Convert character to a Text to impl .render()
            let style: Style = Style::new().bg(Color::Red);
            let text: Text = current_char.to_text().style(style);

            // Render text
            text.render(area, buf);
        }
    }

    #[must_use]
    pub const fn bounds(&self) -> &(usize, usize) {
        &self.bounds
    }

    pub const fn bounds_mut(&mut self) -> &(usize, usize) {
        &self.bounds
    }

    pub const fn increase_start_bound(&mut self, time: usize) {
        self.bounds.0 = self.bounds.0.saturating_add(time);
    }

    pub const fn decrease_start_bound(&mut self, time: usize) {
        self.bounds.0 = self.bounds.0.saturating_sub(time);
    }

    pub const fn increase_end_bound(&mut self, time: usize) {
        self.bounds.1 = self.bounds.1.saturating_sub(time);
    }

    pub const fn decrease_end_bound(&mut self, time: usize) {
        self.bounds.1 = self.bounds.1.saturating_add(time);
    }
}

/// Rendering Methods
impl WaveformWidget<'_> {
    fn calculate_offset(&self, selected: usize, offset: usize, width: usize) -> (usize, usize) {
        // Find the workable space
        let max_length = self.samples.len();

        // Smallest out of offset vs or sample length - width
        let mut first_visible_index = offset.min(max_length.saturating_sub(width));
        // Smallest out of offset + width vs the sample length
        let mut last_visible_index = offset.saturating_add(width).min(max_length);

        // Add the padding (shouldn't ever saturate)
        let scrollable_right = selected.saturating_add(self.scroll_padding);
        // Take away the padding (will saturate at 0)
        let scrollable_left = selected.saturating_sub(self.scroll_padding);

        // If there is space to scroll to the right
        if scrollable_right > last_visible_index && scrollable_right < max_length {
            // Add one to both the start and the end
            first_visible_index = first_visible_index.saturating_add(1);
            last_visible_index = last_visible_index.saturating_add(1);
        // If there is space to scroll to the left
        } else if scrollable_left < first_visible_index && scrollable_right > 0 {
            // Take one to both the start and the end
            first_visible_index = first_visible_index.saturating_sub(1);
            last_visible_index = last_visible_index.saturating_sub(1);
        }

        // Return new (or unchanged) values
        (first_visible_index, last_visible_index)
    }
}

// /// Turns strips of vertical samples to horizontal samples
// /// to render as a list to the ratatui widget
// fn transpose_samples(vertical_samples: &[Vec<char>]) -> Vec<String> {
//     let height: usize = vertical_samples.first().iter().len();
//
//     let mut transposed: Vec<String> = (0..height)
//         .map(|col| {
//             vertical_samples
//                 .iter()
//                 .map(|row| row.get(col).copied().unwrap_or(' '))
//                 .collect::<String>()
//         })
//         .collect();
//
//     transposed.reverse();
//     transposed
// }
