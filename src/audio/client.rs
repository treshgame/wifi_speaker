use rtrb::Consumer;

use crate::{audio::AudioStream, core::AUDIO_BUFFER_LENGTH};

pub fn audio_client(queue: Consumer<[u8; AUDIO_BUFFER_LENGTH]>, stream: Box<dyn AudioStream>) {
    stream.write_loop(queue);
}
