// use audio_waveform_render::{RenderOptions, render_svg};
use std::path::Path;

use cue_audio::waveform::Waveform;

use cue_audio::waveform_widget::{WaveformState, WaveformWidget};
use ratatui::Frame;
use ratatui::crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListState};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut samples: usize = 100;

    let mut waveform_state = WaveformState::default().with_selected(Some(0));
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| render(frame, &mut waveform_state, samples))?;
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
fn render(frame: &mut Frame, waveform_state: &mut WaveformState, samples: usize) {
    let constraints = [
        Constraint::Length(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ];
    let layout = Layout::vertical(constraints).spacing(1);
    let [top, first, second] = frame.area().layout(&layout);

    let title = Line::from_iter([
        Span::from("List Widget").bold(),
        Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_wave(frame, first, waveform_state, samples);
}

/// Render a list.
pub fn render_wave(
    frame: &mut Frame,
    area: Rect,
    waveform_state: &mut WaveformState,
    samples: usize,
) {
    let audio_path = Path::new("/mnt/data/Documents/git/cues/project/audio/boom.wav");

    let mut wav = Waveform::new(audio_path);
    wav.regenerate_with_samples(samples);

    let string = wav.generate_waveform_string(10);

    // let string_strs: Vec<String> = string
    //     .iter()
    //     .map(|vecchar| vecchar.iter().collect::<String>())
    //     .collect();
    //
    // for f in string_strs {
    //     println!("{f}");
    // }

    let waveform = WaveformWidget::new(string.clone());

    frame.render_stateful_widget(waveform, area, waveform_state);
}

// fn main() {
//     let audio_path = Path::new("/mnt/data/Documents/git/cues/project/audio/boom.wav");
//
//     let mut wav = Waveform::new(audio_path);
//     wav.regenerate_with_samples(50);
//
//     let string = wav.generate_waveform_string(10);
//
//     for f in string {
//         println!("{f}");
//     }
// }
