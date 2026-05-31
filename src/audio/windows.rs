use std::{collections::VecDeque, thread, time::Duration};

use rtrb::{Consumer, Producer};
use wasapi::{AudioClient, DeviceEnumerator, Direction, SampleType, StreamMode, WaveFormat, initialize_mta};

use crate::{audio::{AudioParams, AudioStream, SpeakerDirection}, core::AUDIO_BUFFER_LENGTH};

struct WindowsStream {
    audio_client: AudioClient,
    audio_params: AudioParams
}

// TODO
impl AudioStream for WindowsStream {
    fn write_loop(&self, consumer: Consumer<[u8; AUDIO_BUFFER_LENGTH]>) {
    }

    fn read_loop(&self, mut producer: Producer<[u8; AUDIO_BUFFER_LENGTH]>) {
        match self.audio_client.start_stream() {
            Ok(_) => {println!("Stream started successfuly")}
            Err(err) => {
                println!("Error while starting stream: {:?}", err);
                return;
            }
        }

        let sleep_duration = Duration::from_millis(1);

        let capture_client = self.audio_client.get_audiocaptureclient().expect("Error getting AudioCaptureClient");
        let mut buf = [0u8; AUDIO_BUFFER_LENGTH];
        let mut vec_buf: VecDeque<u8> = VecDeque::with_capacity(8192);
        let range = ..AUDIO_BUFFER_LENGTH;
        loop {
            if let Ok(Some(_packet_size)) = capture_client.get_next_packet_size() {
                capture_client.read_from_device_to_deque(&mut vec_buf).expect("Error while reading form device");
            }

            while vec_buf.len() >= AUDIO_BUFFER_LENGTH {
                let (front, _) = vec_buf.as_slices();
                if front.len() >= AUDIO_BUFFER_LENGTH {
                    buf.copy_from_slice(&front[range]);
                    if let Ok(()) = producer.push(buf) {
                    }

                    vec_buf.drain(range);
                }
            }
            // Sleep to not 100% cpu while waiting
            thread::sleep(sleep_duration);
            
        }
    }

    fn get_audio_params(&self) -> AudioParams {
        self.audio_params.clone()
    }
}

// TODO: Do a normal error handling
pub fn create_audio_stream(dir: SpeakerDirection, default_params: AudioParams) -> Option<Box<dyn AudioStream>> {
        
    println!("Create audio stream windows");

    let (audio_client, audio_params) = init_audio_client();
    let _ = audio_client.set_get_eventhandle().expect("error getting h_event");


    return Some(Box::new(WindowsStream{audio_client: audio_client, audio_params}))
}

fn init_audio_client() -> (AudioClient, AudioParams) {
    initialize_mta().ok().expect("Error while initialize_mta");

    let mut desired_format = WaveFormat::new(
        32, 32, &wasapi::SampleType::Int,
        44100, 2, None
    );
    let mode = StreamMode::EventsShared {
        autoconvert: true,
        buffer_duration_hns: 200_000
    };

    let enumeration = DeviceEnumerator::new().expect("DeviceEnumerator::new failed");
    let default_device = enumeration.get_default_device(&Direction::Render)
                    .expect("Filed to get default device");
    let mut audio_client = default_device.get_iaudioclient().expect("Erorr getting audio_client");

    if let Ok(format) = audio_client.is_supported(&desired_format, &wasapi::ShareMode::Shared) {
        if let Some(format) = format {
            println!("Format set: {:?}", format);
            desired_format = format
        }
    }
    
    let audio_params = make_audio_params(&desired_format);
    audio_client.initialize_client(&desired_format, &Direction::Capture, &mode).expect("Error init client");

    (audio_client, audio_params)
}

fn make_audio_params(wave_format: &WaveFormat) -> AudioParams {
    let sample_type = wave_format.get_subformat().expect("Error while getting subformat");
    let format = convert_from_format(sample_type);
    let rate = wave_format.get_samplespersec();
    let channels = wave_format.get_nchannels();
    // TODO: Checks if there's possibility of a bad convertion
    AudioParams {format, rate, channels: channels as u8}
}

fn convert_from_format(sample_type: SampleType) -> u8 {
    match sample_type {
        SampleType::Float => 32,
        SampleType::Int => 16,
    }
}
