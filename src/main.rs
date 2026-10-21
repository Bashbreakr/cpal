use cpal::{Device, SampleFormat::F32, traits::{DeviceTrait, HostTrait}};

enum InOut {
    Input,
    Output,
}

fn find_x32(in_out: InOut) -> Device {
    let host = cpal::default_host();

    match in_out {
        InOut::Input => {
        host.input_devices()
            .expect("failed to get input devices")
            .find(|d| {
                d.description()
                    .map(|desc| desc.name().contains("X-USB"))
                    .unwrap_or(false)
            })
            .expect("failed to find x32")
        },
        InOut::Output => {
        host.output_devices()
            .expect("failed to get output devices")
            .find(|d| {
                d.description()
                    .map(|desc| desc.name().contains("X-USB"))
                    .unwrap_or(false)
            })
            .expect("failed to find x32")
        }
    }
}

fn main() {
    let output_device = find_x32(InOut::Output);
    let _input_device = find_x32(InOut::Input);

    let mut supported_configs_range = output_device.supported_output_configs()
        .expect("error while querying output configs");
    
    let config = supported_configs_range
        .find(|config| 
            config.channels() == 32 && config.sample_format() == F32
            )
        .map(|config| config.with_sample_rate(48000));

    println!("{:#?}", config);
}
