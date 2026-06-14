use std::thread;

use rtrb::RingBuffer;

use crate::{
    audio::{SpeakerDirection, client::audio_client},
    cmd::properties::AppProperties,
    core::{RING_BUFFER_SIZE, create_audio_stream},
    network::client::{connect_to_server, network_client, receive_audio_params}
};

// Main clinet-mode application loop
pub fn client_loop(app_properties: AppProperties) {
    let sock = connect_to_server(
        app_properties.server_addr.as_str(),
        app_properties.client_port,
        app_properties.server_port
    );

    if let Err(err) = sock {
        println!("Error while acquiring socket: {:?}", err);
        return;
    }
    let sock = sock.unwrap();
    
    let params = receive_audio_params(&sock);
    if let None = params {
        println!("No audio params were received");
        return;
    }
    let params = params.unwrap();
    println!("Received params: {:?}", params);

    let stream = create_audio_stream(SpeakerDirection::Playback, params).expect("No stream");

    let (producer, consumer) = RingBuffer::new(RING_BUFFER_SIZE * 2);
    let bytes_producer_thread = thread::spawn(move || {
        if let Err(err) = network_client(sock, producer) {
            println!("Error starting a client socket: {:?}", err);
        }
    });
    println!("Client is started");

    audio_client(consumer, stream);

    bytes_producer_thread.join().expect("Error while join in producer thread");
}
