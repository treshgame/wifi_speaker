pub mod server;
pub mod client;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::create_audio_stream;

#[cfg(target_os = "windows")]
pub mod windows;
use rtrb::{Consumer, Producer};
#[cfg(target_os = "windows")]
pub use windows::create_audio_stream;

use crate::core::AUDIO_BUFFER_LENGTH;

pub enum SpeakerDirection {
    Capture,
    Playback
}

pub trait AudioStream {
    fn write_loop(&self, consumer: Consumer<[u8; AUDIO_BUFFER_LENGTH]>);
    fn read_loop(&self, producer: Producer<[u8; AUDIO_BUFFER_LENGTH]>);
    fn get_audio_params(&self) -> AudioParams;
}

#[derive(Clone)]
pub struct AudioParams {
    pub rate: u32,
    pub channels: u8,
    pub format: u8
}

impl AudioParams {
    pub fn from_bytes(bytes: &[u8]) -> AudioParams {
        let rate = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let channels = bytes[4];
        let format = bytes[5];
        AudioParams {
            rate, channels, format
        }
    }

    pub fn to_bytes(&self) -> [u8; 6]{
        let rate_bytes = self.rate.to_le_bytes();

        return [
            rate_bytes[0],
            rate_bytes[1],
            rate_bytes[2],
            rate_bytes[3],
            self.channels,
            self.format
        ];
    }
}
