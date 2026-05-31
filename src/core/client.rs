use std::thread;

use rtrb::RingBuffer;

use crate::{audio::{SpeakerDirection, client::audio_client}, core::{create_audio_stream, server::DEFAULT_SERVER_PORT}, network::client::{connect_to_server, network_client, receive_audio_params}};

// Main clinet-mode application loop
pub fn client_loop() {
    let sock = connect_to_server("192.168.1.111", DEFAULT_SERVER_PORT+1, DEFAULT_SERVER_PORT);
    if let Err(err) = sock {
        println!("Error while acquiring sockect: {:?}", err);
        return;
    }
    let sock = sock.unwrap();
    
    let params = receive_audio_params(&sock);
    if let None = params {
        println!("No audio params were received");
        return;
    }
    let params = params.unwrap();

    let stream = create_audio_stream(SpeakerDirection::Playback, params).expect("No stream");

    let (producer, consumer) = RingBuffer::new(8);
    let bytes_producer_thread = thread::spawn(move || {
        if let Err(err) = network_client(sock, producer) {
            println!("Error starting a client socket: {:?}", err);
        }
    });
    println!("Client is started");

    audio_client(consumer, stream);

    bytes_producer_thread.join().expect("Error while join in producer thread");
}
