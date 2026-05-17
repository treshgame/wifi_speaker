use std::{io, net::UdpSocket,  time::Duration};

use rtrb::Producer;

use crate::core::AUDIO_BUFFER_LENGTH;

pub fn network_client(port: u16, server_port: u16, mut queue: Producer<[u8; AUDIO_BUFFER_LENGTH]>) -> io::Result<()> {
    let client_addr = format!("0.0.0.0:{}", port);
    let sock = UdpSocket::bind(client_addr)?;
    println!("Client socket is created");
    
    let connect_to_addr = format!("192.168.1.111:{}", server_port);
    println!("DEBUG: Attempting connection to -> '{}'", connect_to_addr);
    sock.connect(connect_to_addr)?;
    println!("Connected to the server");
    sock.send(&[0])?;

    sock.set_read_timeout(Some(Duration::from_secs(3)))?;

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
