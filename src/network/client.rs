use std::{io, net::UdpSocket,  time::Duration};

use rtrb::Producer;

use crate::{audio::AudioParams, core::AUDIO_BUFFER_LENGTH};

pub fn receive_audio_params(sock: &UdpSocket) -> Option<AudioParams> {
    let mut buf = [0u8; 6];
    match sock.recv(&mut buf) {
        Ok(size) => {
            if size == 6 {
                return Some(AudioParams::from_bytes(&buf));
            }
            println!("Wrong amount of bytes were recevied: {:?}", size);
            return None
        },
        Err(err) => {
            println!("Error while receiving audio params: {:?}", err);
            return None
        }
    }
}

pub fn connect_to_server(server_addr: &str, port: u16, server_port: u16) -> io::Result<UdpSocket> {
    let client_addr = format!("0.0.0.0:{}", port);
    let sock = UdpSocket::bind(client_addr)?;

    let server_addr = format!("{server_addr}:{}", server_port);
    sock.connect(server_addr)?;
    println!("Connected to the server");
    sock.set_read_timeout(Some(Duration::from_secs(300)))?;
    sock.send(&[0])?;
    
    Ok(sock)
}

pub fn network_client(sock: UdpSocket, mut queue: Producer<[u8; AUDIO_BUFFER_LENGTH]>) -> io::Result<()> {
    loop {
        let mut buffer = [0u8; AUDIO_BUFFER_LENGTH];
        match sock.recv(&mut buffer) {
            Ok(_) => {
                if let Err(_) = queue.push(buffer) {
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut => {
                println!("Server is not available");
                break;
            },
            Err(err) => {
                return Err(err);
            }
        }
    }

    Ok(())
}
