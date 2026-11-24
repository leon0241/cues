// WHAT TODO:
// Make timestamp an arc (or waveform state?)
// Somehow increase timestamp as a track plays
// Arc clone reference the timestamp on each loop
// Time divided by the zoom of the state
// Indications to how zoomed a track is relative to time in secs
// Adding time indicators or ticks to the graph

use std::time::Duration;

use cue_audio::audio_config::AudioDevice;
use cue_audio::audio_file::AudioFile;
use cue_audio::waveform::Waveform;

use cue_audio::waveform_widget::{WaveformState, WaveformWidget};
use num_traits::ToPrimitive;
use ratatui::Frame;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut samples: usize = 100;

    let audio_path = "/mnt/data/Documents/git/cues/project/audio/test.wav";

    let mut audio_device: AudioDevice = AudioDevice::default();
    audio_device.set_default_sink()?;

    let mut audio_file = AudioFile::new("Boom", audio_path, &audio_device)?;

    // Waveform State selecting Some(0)
    let mut waveform_state = WaveformState::default()
        .with_selected(Some(0))
        .with_timestamp(0);

    let max_sample_size = audio_file.waveform_mut().max_samples();

    ratatui::run(|terminal| {
        loop {
            let duration: Duration = audio_file.timestamp();

            // Non blocking event read
            if event::poll(Duration::from_millis(500))?
                && let event::Event::Key(key) = event::read()?
                && key.kind == event::KeyEventKind::Press
            {
                match key.code {
                    KeyCode::Char(' ') => audio_file.play_file()?,

                    KeyCode::Char('h') | KeyCode::Left => waveform_state.scroll_previous(),
                    KeyCode::Char('l') | KeyCode::Right => waveform_state.scroll_next(),
                    KeyCode::Char('j') | KeyCode::Up => {
                        samples = samples
                            .saturating_add(10)
                            .clamp(10, max_sample_size.saturating_sub(1));
                    }
                    KeyCode::Char('k') | KeyCode::Down => {
                        samples = samples
                            .saturating_sub(10)
                            .clamp(10, max_sample_size.saturating_sub(1));
                    }

                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }

            let wav: &mut Waveform = audio_file.waveform_mut();
            // Mutable reference to AudioFile's Waveform

            // Timestamp relative position
            let relative_position: usize = wav.relative_position(&duration).to_usize().unwrap_or(0);

            // Regenerate waveform if has been zoomed
            if samples != wav.samples() {
                let old_selected = waveform_state.selected().unwrap_or(0);
                let ratio = waveform_state
                    .selected()
                    .unwrap_or(0)
                    .to_f64()
                    .unwrap_or(0.0)
                    / wav.samples().to_f64().unwrap_or(1.0);

                let new_position = ratio * samples.to_f64().unwrap_or(0.0);

                let new_selected = new_position.round().to_usize().unwrap_or(0);

                // total_sample_size.div(selected) * new_sample_size
                wav.regenerate_with_samples(samples);
                waveform_state.select(Some(new_selected));

                // new position is bigger (further right)
                if old_selected < new_selected {
                    waveform_state.add_offset(new_selected.saturating_sub(old_selected));
                }
                // new position is smaller (further left)
                else if old_selected > new_selected {
                    waveform_state.sub_offset(old_selected.saturating_sub(new_selected));
                }
            }

            // Set the timestamp based on the percentage
            waveform_state.set_timestamp(relative_position);

            // Finally Draw
            terminal.draw(|frame| render(frame, &mut waveform_state, wav))?;
        }
    })
}

/// Render the UI with various lists.
fn render(frame: &mut Frame, waveform_state: &mut WaveformState, waveform: &mut Waveform) {
    let constraints = [
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Length(2),
    ];
    let layout = Layout::vertical(constraints).spacing(1);
    let [top, first, _second] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("List Widget").bold(),
        Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_wave(frame, first, waveform_state, waveform);
}

/// Render a list.
pub fn render_wave(
    frame: &mut Frame,
    area: Rect,
    waveform_state: &mut WaveformState,
    waveform: &mut Waveform,
) {
    let string = waveform.generate_waveform_string(area.height.into());

    let waveform_widget = WaveformWidget::new(string.clone());

    frame.render_stateful_widget(waveform_widget, area, waveform_state);
}

// TODO: remove this
// 50 [20 - 30] 25
// 60 [25 - 35] 30
// 70 [30 - 40] 35

// 50 [20 - 30] 30
// 60 [25 - 35] 35
// 70 [30 - 40] 40

// 50 [0 - 10] 5
// 60 [5 - 15] 10
// 70 [10 - 20] 15

// x + 5

// 50 [0 - 10] 0
// 60 [0 - 10] 0
// 70 [0 - 10] 0

// 50 [0 - 10] 10
// 60 [10 - 20] 15
// 70 [15 - 25] 20

// 111222333444555666777888999000
// 11223344556677889900
// 1234567890

// 50 [0 - 10] 3
// 60 [3 - 13] 8
// 70 [8 - 18] 13
