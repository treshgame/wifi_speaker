use rtrb::Producer;

use crate::{audio::AudioStream, core::AUDIO_BUFFER_LENGTH};

pub fn audio_server(bytes_to_send: Producer<[u8; AUDIO_BUFFER_LENGTH]>, stream: Box<dyn AudioStream>) {
    stream.read_loop(bytes_to_send);
}
