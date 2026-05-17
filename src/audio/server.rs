use libpulse_binding::stream::Direction;
use rtrb::Producer;

use crate::{audio::pulse_stream, core::AUDIO_BUFFER_LENGTH};

pub fn audio_server(mut bytes_to_send: Producer<[u8; AUDIO_BUFFER_LENGTH]>) {
    
    let stream = pulse_stream::create_pusle_stream(Direction::Record).expect("No stream");
    // Wait for stream to be ready
    let mut buffer = [0u8; AUDIO_BUFFER_LENGTH];
    loop {
        match stream.read(&mut buffer) {
            Ok(()) => {
                println!("Read: {:?}", buffer);
                if !bytes_to_send.is_full() {
                    let _ = bytes_to_send.push(buffer);
                }
            }
            Err(err) => {
                eprintln!("Error while reading: {:?}", err);
                break;
            }
        }
    }
}
