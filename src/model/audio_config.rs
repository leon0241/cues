//! # Audio Config
//!
//! Handles the device patch using Rodio. Currently can only find the default sink but hopefully
//! later will let you choose matrix points and multiple output devices.

use rodio::MixerDeviceSink;

/// Handler for an Audio Device
///
/// Currently only has a Device Sink but it has big dreams.
#[derive(Default)]
pub struct AudioDevice {
    /// Device Sink. This is the output that any Rodio `Players` will attach to
    pub sink: Option<MixerDeviceSink>,
}

impl AudioDevice {
    /// Sets the default sink
    ///
    /// # Errors
    ///
    /// Will return `Err` if no default sink, and therefore not playable audio
    pub fn set_default_sink(&mut self) -> color_eyre::Result<()> {
        // MixerSinkError if there is any
        self.sink = Some(rodio::DeviceSinkBuilder::open_default_sink()?);

        Ok(())
    }
    //
    // pub fn change_device() {
    //     unimplemented!();
    // }
}
