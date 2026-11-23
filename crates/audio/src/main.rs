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
use ratatui::Frame;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut samples: usize = 100;

    let audio_path = "/mnt/data/Documents/git/cues/project/audio/boom.wav";

    let mut audio_device: AudioDevice = AudioDevice::default();
    audio_device.set_default_sink()?;

    let mut audio_file = AudioFile::new("Boom", audio_path, &audio_device)?;

    let duration = audio_file.timestamp();

    // Mutable reference to AudioFile's Waveform
    let wav: &mut Waveform = audio_file.waveform_mut();
    wav.regenerate_with_samples(samples);

    // Waveform State selecting Some(0)
    let mut waveform_state = WaveformState::default()
        .with_selected(Some(0))
        .with_timestamp(0);

    ratatui::run(|terminal| {
        loop {
            waveform_state.set_timestamp(wav.pos_from_time(duration));

            terminal.draw(|frame| render(frame, &mut waveform_state, wav))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('h') | KeyCode::Left => waveform_state.select_previous(),
                    KeyCode::Char('l') | KeyCode::Right => waveform_state.select_next(),
                    KeyCode::Char('j') | KeyCode::Up => samples = samples.saturating_add(10),
                    KeyCode::Char('k') | KeyCode::Down => samples = samples.saturating_sub(10),
                    KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }
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
