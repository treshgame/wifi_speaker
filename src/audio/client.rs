use rtrb::Consumer;

use crate::{audio::AudioStream, core::AUDIO_BUFFER_LENGTH};

pub fn audio_client(mut queue: Consumer<[u8; AUDIO_BUFFER_LENGTH]>, stream: Box<dyn AudioStream>) {
    loop {
        if let Ok(bytes) = queue.pop() {
            // TODO: normal error handling after testing
            stream.write(&bytes);
        }
    }
}
