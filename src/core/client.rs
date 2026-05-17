use std::thread;

use rtrb::RingBuffer;

use crate::{audio::client::audio_client, core::server::DEFAULT_SERVER_PORT, network::client::network_client};

// Main clinet-mode application loop
pub fn client_loop() {
    let (producer, consumer) = RingBuffer::new(8);

    let bytes_producer_thread = thread::spawn(move || {
        if let Err(err) = network_client(DEFAULT_SERVER_PORT+1, DEFAULT_SERVER_PORT, producer) {
            println!("Error starting a client socket: {:?}", err);
        }
    });
    println!("Client is started");

    audio_client(consumer);

    bytes_producer_thread.join().expect("Error while join in producer thread");
}
