use std::thread;

use rtrb::RingBuffer;

use crate::{audio::{AudioParams, SpeakerDirection, server::audio_server}, core::{RING_BUFFER_SIZE, create_audio_stream}, network::server::{network_server, start_server}};

pub const DEFAULT_SERVER_PORT: u16 = 10101;

// Main server-mode application loop
pub fn server_loop() {
    let default_params = AudioParams {rate: 44100, channels: 2, format: 16};
    // getting our audio stream with parameters
    let audio_param = create_audio_stream(SpeakerDirection::Capture, default_params).expect("Error creating audio_stream");

    let net_server = start_server(DEFAULT_SERVER_PORT, audio_param.get_audio_params())
        .expect("Failed to start a server");

    let server_for_listener = net_server.clone();

    // listen for new clients
    let _ = thread::spawn(move || {
        network_server(server_for_listener);
    });
    
    let (producer, mut consumer) = RingBuffer::new(RING_BUFFER_SIZE);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(bytes) = consumer.pop() {
                net_server.send_to_all(&bytes).expect("Error while sending bytes");
            }
        }
    });
    audio_server(producer, audio_param);

    println!("Started a server");
}
