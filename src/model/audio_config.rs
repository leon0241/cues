use rodio::{MixerDeviceSink};

// #[derive(Debug)]
pub struct AudioDevice {
    pub sink: MixerDeviceSink,
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

    pub fn change_device() {
        unimplemented!();
    }
}
