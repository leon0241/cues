use std::fs::File;
use std::num::NonZero;

use rodio::mixer::{self, Mixer, MixerSource};
// use rodio::{Player, Decoder, MixerDeviceSink, source::Source};
use rodio::{Player, Decoder, MixerDeviceSink};

// #[derive(Debug)]
pub struct AudioDevice {
    pub sink: MixerDeviceSink,
}
pub struct AudioPlayer {
    player: Player,
}

impl Default for AudioDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioDevice {
    pub fn new() -> Self {
        let sink = rodio::DeviceSinkBuilder::
            open_default_sink()
            .expect("open default audio stream");

        Self { sink }
    }
}
