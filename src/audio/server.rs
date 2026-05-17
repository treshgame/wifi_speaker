use rtrb::Producer;

use crate::{audio::{SpeakerDirection, create_audio_stream}, core::AUDIO_BUFFER_LENGTH};

pub fn audio_server(mut bytes_to_send: Producer<[u8; AUDIO_BUFFER_LENGTH]>) {
    
    let stream = create_audio_stream(SpeakerDirection::Capture).expect("No stream");
    // Wait for stream to be ready
    let mut buffer = [0u8; AUDIO_BUFFER_LENGTH];
    loop {
        // TODO: normal error handling after testing
        stream.read(&mut buffer);
        let _ = bytes_to_send.push(buffer);
    }
}
