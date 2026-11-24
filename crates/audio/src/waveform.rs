//! # Waveform
//!
//! Waveform object for an audio file. Relies heavily on the `audio_waveform` package.

use audio_waveform::{WaveformOptions, generate_from_u8_samples, generate_u8};
use num_traits::ToPrimitive;
use std::{path::Path, time::Duration};

/// Waveform Object
///
/// Contains a waveform object in samples, as well as a renderable set of text within the terminal.
#[derive(Clone)]
pub struct Waveform {
    points_upper: Vec<u8>,
    points_current: Vec<u8>,
    current_sample_size: usize,
    wave_string: Vec<Vec<char>>,
    total_duration: Duration,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            points_upper: Vec::default(),
            points_current: Vec::default(),
            current_sample_size: 1000,
            wave_string: Vec::default(),
            total_duration: Duration::default(),
        }
    }
}

impl Waveform {
    /// Creates a new `Waveform`
    ///
    /// Takes an audio path (probably can be relative), as well as a duration (`Duration`).
    #[must_use]
    pub fn new(audio_path: &str, duration: Duration) -> Self {
        let max_samples_per_second = 100;
        // max samples * duration of track... or 1000 if it errors
        let samples_to_create = duration
            .as_secs()
            .saturating_mul(max_samples_per_second)
            .to_usize()
            .unwrap_or(1000);

        let options = WaveformOptions::new(samples_to_create);

        let path: &Path = Path::new(audio_path);

        // Generate in u8 (don't need more precision as we are converting seconds into 1/8 block cells anyways).
        let generated = generate_u8(path, &options).ok();

        // points upper as well as the duration which we don't need as we already pass it.
        let (points_upper, _) = generated.unwrap_or_else(|| (Vec::default(), 0.0));

        Self {
            points_current: points_upper.clone(),
            points_upper,
            total_duration: duration,
            current_sample_size: 500,
            wave_string: Vec::default(),
        }
    }

    /// Accessor for samples (Immutable)
    ///
    /// This returns an immutable reference, but mutable references to the samples should
    /// never be needed.
    #[must_use]
    pub const fn samples(&self) -> usize {
        self.current_sample_size
    }

    /// Sets the current sample size.
    ///
    /// Note that most times, this should not be called directly, and instead implicitly
    /// done via `regenerate_with_samples()`.
    pub const fn set_sample_size(&mut self, size: usize) {
        self.current_sample_size = size;
    }

    /// Accessor for the maximum sample size (Immutable)
    ///
    /// Useful for setting the maximum zoom
    #[must_use]
    pub const fn max_samples(&self) -> usize {
        self.points_upper.len()
    }

    /// Creates a new waveform image based on the existing waveform and a sample size.
    ///
    /// The new sample size should be less than the maximum, otherwise it starts to distort
    /// the image.
    pub fn regenerate_with_samples(&mut self, samples: usize) {
        // New options
        let options = WaveformOptions::new(samples);

        // Regenerate if it's a smaller sample size. Currently this function cannot regenerate with
        // more samples as that involves a completely new file generation. Possibly will just limit
        // the maximum file increment to 0.1s or something.
        if samples < self.points_upper.len() {
            self.points_current = generate_from_u8_samples(&self.points_upper, &options);
            self.current_sample_size = samples;
        }
    }

    /// Generates a waveform string based on a height
    ///
    /// TODO: This height should be an even number as the block size will not render properly
    /// for small db columns (it would need unicode blocks) between blocks. This could
    /// possibly be cheated by inverting top/bottom blocks?
    pub fn generate_waveform_string(&mut self, height: usize) -> &Vec<Vec<char>> {
        self.wave_string = self
            .points_current
            .iter()
            .map(|point| block_percentage(*point, height).chars().rev().collect())
            .collect();

        &self.wave_string
    }

    /// Accessor for the waveform string (Immutable)
    ///
    /// This is an immutable reference, and should be read-only.
    #[must_use]
    pub const fn waveform(&self) -> &Vec<Vec<char>> {
        &self.wave_string
    }

    /// Given a Duration, find the position in the track based on sample size
    #[must_use]
    pub fn relative_position(&self, timestamp: &Duration) -> f64 {
        // like it should always unwrap
        let sample_size_f64: f64 = self.current_sample_size.to_f64().unwrap_or(f64::MAX);

        // Percentage of the way through
        let progress_f64: f64 = timestamp.div_duration_f64(self.total_duration);

        // That ratio in total number of sample ticks
        progress_f64 * sample_size_f64
    }

    /// Accessor for the total duration (Immutable)
    ///
    /// This is an immutable reference, and should be read-only.
    #[must_use]
    pub const fn total_duration(&self) -> Duration {
        self.total_duration
    }
}

// TODO: odd heights not having block elements for the middle section
fn block_percentage(vol: u8, max_height: usize) -> String {
    #[allow(
        clippy::as_conversions,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "clamped to 0..=255 first; NaN casts to 0"
    )]
    fn safe_floor(v: f32) -> u8 {
        v.floor().clamp(0.0, f32::from(u8::MAX)) as u8
    }

    let max_height_u: u8 = u8::try_from(max_height).unwrap_or(10);

    // Height relative to the set height
    //
    // f32 Between 0 and max_height
    let float_height: f32 = f32::from(vol) / 255.0 * f32::from(max_height_u);

    let wholes: u8 = safe_floor(float_height);

    // Closest Whole Number matching even/odd sign of max height
    let wholes_even: u8 = if (wholes.saturating_add(max_height_u)).is_multiple_of(2) {
        wholes
    } else {
        wholes.saturating_sub(1)
    };

    let remainder: f32 = float_height - f32::from(wholes_even); // [0 <= x < 2]

    let upscale: f32 = 0.5 * remainder * 8.0; // [0 <= x < 8]
    //
    let final_perc: u8 = safe_floor(upscale);

    let full_block: &str = &"█".repeat(usize::from(wholes_even));

    // "█▇▆▅▄▃▂▁"
    // "█🮆🮅🮄▀🮃🮂▔"
    let (start, end): (&str, &str) = match final_perc {
        1 => ("▁", "▔"), // 1/8
        2 => ("▂", "🮂"), // 1/4
        3 => ("▃", "🮃"), // 3/8
        4 => ("▄", "▀"), // 1/2
        5 => ("▅", "🮄"), // 5/8
        6 => ("▆", "🮅"), // 3/4
        7 => ("▇", "🮆"), // 7/8
        _ => ("", ""),   // 0 or full. not a space so that volumes of max height don't
                          // generate a second block
    };

    // max height - wholes - blank (if exists)
    let thing = max_height_u
        .saturating_sub(wholes_even)
        .saturating_sub(if start.is_empty() { 0 } else { 2 })
        .saturating_div(2);

    let filler: &str = &" ".repeat(usize::from(thing));

    let mut build: String = filler.to_string();

    build.push_str(end);
    build.push_str(full_block);
    build.push_str(start);
    build.push_str(filler);

    build
}
