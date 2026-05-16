use std::thread;

use rtrb::RingBuffer;

use crate::{audio::client::audio_client, core::server::DEFAULT_SERVER_PORT, network::client::network_client};

// Main clinet-mode application loop
pub fn client_loop() {
    let (producer, consumer) = RingBuffer::new(16);
    let bytes_consumer_thread = thread::spawn(move || {
        audio_client(consumer);
    });

    let bytes_producer_thread = thread::spawn(move || {
        if let Err(err) = network_client(DEFAULT_SERVER_PORT+1, DEFAULT_SERVER_PORT, producer) {
            println!("Error starting a client socket: {:?}", err);
        }
    });
    println!("Client is started");

    bytes_producer_thread.join().expect("Error while join in producer thread");
    bytes_consumer_thread.join().expect("Error while join in consumer thread");
}
