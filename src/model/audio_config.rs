use rodio::{MixerDeviceSink};

#[derive(Default)]
pub struct AudioDevice {
    // Default: None
    pub sink: Option<MixerDeviceSink>,
}

impl AudioDevice {
    /// # Errors
    ///
    /// Will return `Err` if no default sink
    pub fn set_default_sink(&mut self) -> color_eyre::Result<()> {
        // MixerSinkError if there is any
        self.sink = Some(rodio::DeviceSinkBuilder::
            open_default_sink()?);

        Ok(())
    }
    //
    // pub fn change_device() {
    //     unimplemented!();
    // }
}
