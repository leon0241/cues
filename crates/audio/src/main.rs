use audio_waveform::{WaveformOptions, generate_from_u8_samples, generate_u8};
// use audio_waveform_render::{RenderOptions, render_svg};
use std::path::Path;

fn main() {
    let audio_path = Path::new("/mnt/data/Documents/git/cues/project/audio/boom.wav");

    let mut wav = Waveform::new(audio_path);
    wav.regenerate_with_samples(50);

    let string = wav.generate_waveform_string(10);

    for f in string {
        println!("{f}");
    }

    println!("test");
}

struct Waveform {
    points: Vec<u8>,
    current_sample_size: usize,
    wave_string: Vec<String>,
}

impl Waveform {
    pub fn new(audio_path: &Path) -> Self {
        let options = WaveformOptions::new(250);

        let generated = generate_u8(audio_path, &options);
        let points = match generated {
            Ok(v) => v.0,
            // TODO: actually learn how errors work
            Err(v) => Vec::default(),
        };

        Self {
            points,
            current_sample_size: 250,
            wave_string: Vec::default(),
        }
    }

    pub fn regenerate_with_samples(&mut self, samples: usize) {
        let options = WaveformOptions::new(samples);

        let generated = generate_from_u8_samples(&self.points, &options);

        self.current_sample_size = samples;

        self.points = generated;
    }

    pub fn generate_waveform_string(&mut self, height: usize) -> &Vec<String> {
        let verti: Vec<Vec<char>> = self
            .points
            .iter()
            .map(|point| block_percentage(*point, height).chars().collect())
            .collect();

        self.wave_string = (0..height)
            .map(|col| {
                verti
                    .iter()
                    .map(|row| row.get(col).copied().unwrap_or(' '))
                    .collect::<String>()
            })
            .collect();

        self.wave_string.reverse();

        &self.wave_string
    }

    pub const fn waveform(&self) -> &Vec<String> {
        &self.wave_string
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
