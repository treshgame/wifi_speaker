use std::{io, net::{SocketAddr, UdpSocket}, sync::{Arc, RwLock}};

use crate::core::AUDIO_BUFFER_LENGTH;

#[derive(Clone)]
pub struct Server {
    sock: Arc<UdpSocket>,
    clients: Arc<RwLock<Vec<SocketAddr>>>
}

impl Server {
    fn new(sock: UdpSocket) -> Server {
        let clients = Vec::new();
        let rw_lock = RwLock::new(clients);
        let arc = Arc::new(rw_lock);
        Server {
            sock: Arc::new(sock),
            clients: arc
        }
    }

    pub fn send_to_all(&self, bytes: &[u8; AUDIO_BUFFER_LENGTH]) -> io::Result<()> {
        let read_lock = &self.clients.read();
        match read_lock {
            Ok(rwlock) => {
                for client in rwlock.iter() {
                    self.sock.send_to(bytes, client)?;
                }
            },
            Err(err) => {
                println!("Error while acquire lock for read: {:?}", err);
            }
        };

        Ok(())
    }

    pub fn add_client(&self, client: SocketAddr) {
        let write_lock = self.clients.write();
        match write_lock {
            Ok(mut rwlock) => {
                rwlock.push(client);           
            },
            Err(err) => {
                println!("Error while acquire lock for read: {:?}", err);
            }
        }
    }

    pub fn get_sock_arc(&self) -> Arc<UdpSocket> {
        self.sock.clone()
    }
}

// Return struct with opened socket and clients lists
pub fn start_server(port: u16) -> io::Result<Server> {
    let addr = format!("0.0.0.0:{}", port);
    let socket = UdpSocket::bind(&addr)?;
    Ok(Server::new(socket))
}

pub fn network_server(server: Server) {
    let mut buf: [u8; 4] = [0; 4];
    let socket = server.get_sock_arc();
    while let Ok((_, client_addr)) = socket.recv_from(&mut buf) {
        println!("Added to the clients: {:?}", client_addr);
        server.add_client(client_addr);
    }
}

