use std::thread;

use rtrb::RingBuffer;

use crate::{audio::server::audio_server, network::server::{network_server, start_server}};

pub const DEFAULT_SERVER_PORT: u16 = 10101;

// Main server-mode application loop
pub fn server_loop() {
    let net_server = start_server(DEFAULT_SERVER_PORT)
        .expect("Failed to start a server");

    let server_for_listener = net_server.clone();

    let _ = thread::spawn(move || {
        network_server(server_for_listener);
    });
    
    let (producer, mut consumer) = RingBuffer::new(8);

    let _ = thread::spawn(move || {
        loop {
            if let Ok(bytes) = consumer.pop() {
                net_server.send_to_all(&bytes).expect("Error while sending bytes");
            }
        }
    });
    audio_server(producer);

    println!("Started a server");
}
