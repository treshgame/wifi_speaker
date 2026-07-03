pub mod server;
pub mod client;

pub const AUDIO_BUFFER_LENGTH: usize = 512;
pub const RING_BUFFER_SIZE: usize = 8;

#[cfg(target_os="linux")]
pub use crate::audio::linux::create_audio_stream;

#[cfg(target_os="windows")]
pub use crate::audio::windows::create_audio_stream;
