use std::{io, net::UdpSocket,  time::Duration};

use rtrb::Producer;

use crate::core::AUDIO_BUFFER_LENGTH;

pub fn network_client(port: u16, server_port: u16, mut queue: Producer<[i16; AUDIO_BUFFER_LENGTH]>) -> io::Result<()> {
    let client_addr = format!("127.0.0.1:{}", port);
    let sock = UdpSocket::bind(client_addr)?;
    
    let connect_to_addr = format!("127.0.0.1:{}", server_port);
    sock.connect(connect_to_addr)?;
    println!("Connected to the server");
    sock.send(&[0])?;

    sock.set_read_timeout(Some(Duration::from_secs(3)))?;

    loop {
        let mut buffer = [0u8; AUDIO_BUFFER_LENGTH * 2];
        let mut samples = [0i16; AUDIO_BUFFER_LENGTH];
        match sock.recv(&mut buffer) {
            Ok(_) => {
                for i in 0..AUDIO_BUFFER_LENGTH {
                    samples[i] = i16::from_le_bytes([buffer[i*2], buffer[i*2+1]]);
                }
                if let Err(_) = queue.push(samples) {
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
