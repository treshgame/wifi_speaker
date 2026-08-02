use std::thread::{self, sleep};

use rtrb::RingBuffer;

use crate::{
    audio::{AudioParams, SpeakerDirection, server::audio_server},
    cmd::properties::AppProperties, core::{RING_BUFFER_SIZE, create_audio_stream},
    network::{PING_INTERVAL, server::{network_server, start_server}}
};


// Main server-mode application loop
pub fn server_loop(app_properties: AppProperties) {
    let default_params = AudioParams {rate: 44100, channels: 2, format: 16};
    // getting our audio stream with parameters
    let audio_param = create_audio_stream(SpeakerDirection::Capture, default_params).expect("Error creating audio_stream");

    let net_server = start_server(app_properties.server_port, audio_param.get_audio_params())
        .expect("Failed to start a server");

    let connection_listener_instance = net_server.clone();
    let health_check_instance = net_server.clone();

    // listen for new clients
    let _ = thread::spawn(move || {
        network_server(connection_listener_instance);
    });
    
    let (producer, mut consumer) = RingBuffer::new(RING_BUFFER_SIZE);
    let _ = thread::spawn(move || {
        loop {
            if let Ok(bytes) = consumer.pop() {
                net_server.send_to_all(&bytes).expect("Error while sending bytes");
            }
        }
    });

    let _ = thread::spawn(move || {
        loop {
            sleep(PING_INTERVAL);
            health_check_instance.check_clients();
        }
    });
    audio_server(producer, audio_param);

    println!("Started a server");
}
