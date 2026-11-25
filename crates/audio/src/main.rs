// WHAT TODO:
// Make timestamp an arc (or waveform state?)
// Somehow increase timestamp as a track plays
// Arc clone reference the timestamp on each loop
// Time divided by the zoom of the state
// Indications to how zoomed a track is relative to time in secs
// Adding time indicators or ticks to the graph

use std::sync::{Arc, RwLock};
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

    // not idx at 0 so sub one
    let max_sample_size = audio_file
        .waveform()
        .read()
        .map_or(0, |f| f.max_samples().saturating_sub(1));

    ratatui::run(|terminal| {
        loop {
            // Current timestamp
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
                        samples = samples.saturating_add(10).clamp(10, max_sample_size);
                    }
                    KeyCode::Char('k') | KeyCode::Down => {
                        samples = samples.saturating_sub(10).clamp(10, max_sample_size);
                    }

                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }

            // read da guy
            let wav: Arc<RwLock<Waveform>> = audio_file.waveform();

            // Timestamp relative position
            let relative_position: usize = wav.read().map_or(0, |f| {
                f.relative_position(&duration).to_usize().unwrap_or(0)
            });

            // Set the timestamp based on the percentage
            waveform_state.set_timestamp(relative_position);

            // rejig the waveform state based on zoom state and other factors
            recalibrate(samples, &wav, &mut waveform_state);

            // Finally Draw
            terminal.draw(|frame| render(frame, &mut waveform_state, &wav))?;
        }
    })
}

/// Render the UI with various lists.
fn render(frame: &mut Frame, waveform_state: &mut WaveformState, wav: &Arc<RwLock<Waveform>>) {
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

    let waveform_string: Vec<Vec<char>> = wav.write().map_or_else(
        |_e| Vec::default(),
        |mut f| f.generate_waveform_string(first.height.into()).clone(),
    );

    render_wave(frame, first, waveform_state, &waveform_string);
}

/// Render a list.
pub fn render_wave(
    frame: &mut Frame,
    area: Rect,
    waveform_state: &mut WaveformState,
    waveform: &[Vec<char>],
) {
    let waveform_widget = WaveformWidget::new(waveform);

    frame.render_stateful_widget(waveform_widget, area, waveform_state);
}

// change the selected according to zoom
fn recalibrate(samples: usize, wav: &Arc<RwLock<Waveform>>, waveform_state: &mut WaveformState) {
    let old_samples = wav.read().map_or(0, |f| f.samples());
    // Regenerate waveform if has been zoomed
    if samples != old_samples {
        let old_selected = waveform_state.selected().unwrap_or_default();

        let ratio = waveform_state
            .selected()
            .unwrap_or_default()
            .to_f64()
            .unwrap_or_default()
            / old_samples.to_f64().unwrap_or(1.0);

        let new_position = ratio * samples.to_f64().unwrap_or_default();

        let new_selected = new_position.round().to_usize().unwrap_or_default();

        if let Ok(mut w) = wav.try_write() {
            w.regenerate_with_samples(samples);
        }
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
}
