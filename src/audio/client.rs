use libpulse_binding::stream::Direction;
use rtrb::Consumer;

use crate::{audio::pulse_stream, core::AUDIO_BUFFER_LENGTH};

pub fn audio_client(mut queue: Consumer<[u8; AUDIO_BUFFER_LENGTH]>) {
    let stream = pulse_stream::create_pusle_stream(Direction::Playback).expect("No stream");
    loop {
        if let Ok(bytes) = queue.pop() {
            if let Err(err) = stream.write(&bytes) {
                println!("Error while writing to a stream: {:?}", err);
            }
        }
    }
}
