use rtrb::Consumer;

use crate::{audio::{SpeakerDirection, create_audio_stream}, core::AUDIO_BUFFER_LENGTH};

pub fn audio_client(mut queue: Consumer<[u8; AUDIO_BUFFER_LENGTH]>) {
    let stream = create_audio_stream(SpeakerDirection::Playback).expect("No stream");
    loop {
        if let Ok(bytes) = queue.pop() {
            // TODO: normal error handling after testing
            stream.write(&bytes);
        }
    }
}
