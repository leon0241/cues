use cpal::{
    {Data, Sample, SampleFormat, FromSample},
    traits::{DeviceTrait, HostTrait},
    platform::{ Device, Host }
};


pub fn audio_device_init() {
    let host: Host = cpal::default_host();

    let device: Device = host.default_output_device().expect("no output device available");

    let mut supported_config_range = device.supported_output_configs()
        .expect("error while querying configs");

    let supported_config = supported_config_range.next()
        .expect("no support config?!")
        .with_max_sample_rate();

    let config = supported_config.into();
    let sample_format = supported_config.sample_format();

    let err_fn = |err| eprintln!("an error occurred: {err}");

    let stream = match sample_format {
        SampleFormat::F32 => device.build_output_stream(config, write_silence::<f32>, err_fn, None),
        SampleFormat::I16 => device.build_output_stream(config, write_silence::<i16>, err_fn, None),
        SampleFormat::U16 => device.build_output_stream(config, write_silence::<u16>, err_fn, None),
        sample_format => panic!("Unsupported sample format '{sample_format}'")
    }.unwrap();

    fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
        for sample in data.iter_mut() {
            *sample = Sample::EQUILIBRIUM;
        }
    }
}
