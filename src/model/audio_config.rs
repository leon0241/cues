use std::fs::File;

// use rodio::mixer::{self, Mixer, MixerSource};
// use rodio::{Player, Decoder, MixerDeviceSink, source::Source};
use rodio::{Decoder, MixerDeviceSink};

#[derive(Debug)]
pub struct AudioDevice {
    handle: MixerDeviceSink,
}

impl Default for AudioDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioDevice {
    pub fn new() -> Self {
        let handle = rodio::DeviceSinkBuilder::
            open_default_sink()
            .expect("open default audio stream");

        Self { handle }
    }

    pub fn play_file(&self, file: File) {
        // Decode that sound file into a source
        let source = Decoder::try_from(file).unwrap();

        self.handle.mixer().add(source);
    }
}
