use audio_waveform::{WaveformOptions, generate_from_u8_samples, generate_u8};
use num_traits::ToPrimitive;
use std::{path::Path, time::Duration};

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
            current_sample_size: 250,
            wave_string: Vec::default(),
            total_duration: Duration::default(),
        }
    }
}

impl Waveform {
    #[must_use]
    pub fn new(audio_path: &str) -> Self {
        let options = WaveformOptions::new(500);

        let path: &Path = Path::new(audio_path);

        let generated = generate_u8(path, &options).ok();

        let (points_upper, duration_f64) = generated.unwrap_or_else(|| (Vec::default(), 0.0));

        let total_duration = Duration::from_secs_f64(duration_f64);

        Self {
            points_current: points_upper.clone(),
            points_upper,
            total_duration,
            current_sample_size: 500,
            wave_string: Vec::default(),
        }
    }

    #[must_use]
    pub const fn samples(&self) -> usize {
        self.current_sample_size
    }

    pub const fn set_sample_size(&mut self, size: usize) {
        self.current_sample_size = size;
    }

    #[must_use]
    pub const fn max_samples(&self) -> usize {
        self.points_upper.len()
    }

    pub fn regenerate_with_samples(&mut self, samples: usize) {
        let options = WaveformOptions::new(samples);

        if samples < self.points_upper.len() {
            self.points_current = generate_from_u8_samples(&self.points_upper, &options);
        }

        self.current_sample_size = samples;
    }

    pub fn generate_waveform_string(&mut self, height: usize) -> &Vec<Vec<char>> {
        self.wave_string = self
            .points_current
            .iter()
            .map(|point| block_percentage(*point, height).chars().rev().collect())
            .collect();

        &self.wave_string
    }

    #[must_use]
    pub const fn waveform(&self) -> &Vec<Vec<char>> {
        &self.wave_string
    }

    /// Given a Duration, find the Position in the track based on sample size
    #[must_use]
    pub fn relative_position(&self, timestamp: &Duration) -> f64 {
        // like it should always unwrap
        let sample_size_f64: f64 = self.current_sample_size.to_f64().unwrap_or(f64::MAX);

        // Percentage of the way through
        let progress_f64: f64 = timestamp.div_duration_f64(self.total_duration);

        // That ratio in total number of sample ticks
        progress_f64 * sample_size_f64
    }

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
