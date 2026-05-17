pub mod server;
pub mod client;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::create_audio_stream;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::create_audio_stream;

use crate::core::AUDIO_BUFFER_LENGTH;

pub enum SpeakerDirection {
    Capture,
    Playback
}

pub trait AudioStream {
    fn write(&self, bytes: &[u8]);
    fn read(&self, buf: &mut [u8; AUDIO_BUFFER_LENGTH]);
}
