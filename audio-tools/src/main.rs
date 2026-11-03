use audio_waveform::{WaveformOptions, generate_from_u8_samples, generate_u8};
use audio_waveform_render::{RenderOptions, render_svg};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_path = Path::new("/mnt/data/Documents/git/cues/project/audio/test.wav");

    let mut wav = Waveform::new(audio_path);

    // let svg = render_svg(wav.points(), &RenderOptions::new(1200, 300))?;
    // println!("{:?}", wav.points());
    // wav.regenerate_with_samples(10);
    // println!("{:?}", wav.points());
    // wav.regenerate_with_samples(50);
    // println!("{:?}", wav.points());
    // std::fs::write("waveform.svg", svg)?;
    //
    wav.points
        .iter()
        .for_each(|point| println!("{}", block_percentage(*point)));
    Ok(())
}

struct Waveform {
    points: Vec<u8>,
    current_sample_size: usize,
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
        }
    }

    pub fn regenerate_with_samples(&mut self, samples: usize) {
        let options = WaveformOptions::new(samples);

        let generated = generate_from_u8_samples(&self.points, &options);

        self.current_sample_size = samples;

        self.points = generated;
    }

    pub const fn points(&self) -> &Vec<u8> {
        &self.points
    }
}

fn block_percentage(vol: u8) -> String {
    let modulo: u8 = vol % 8;
    let wholes: u8 = (vol.wrapping_sub(modulo)) / 8;

    let full_block: &str = &"█".repeat(usize::from(wholes));

    // "█▇▆▅▄▃▂▁"
    // "█🮆🮅🮄▀🮃🮂▔"
    let (start, end): (&str, &str) = match modulo {
        1 => ("▁", "▔"), // 1/8
        2 => ("▂", "🮂"), // 1/4
        3 => ("▃", "🮃"), // 3/8
        4 => ("▄", "▀"), // 1/2
        5 => ("▅", "🮄"), // 5/8
        6 => ("▆", "🮅"), // 3/4
        7 => ("▇", "🮆"), // 7/8
        _ => ("", ""),
    };

    format!("{start}, {full_block}, {end}")
}
