use std::{io, net::{SocketAddr, UdpSocket}, sync::{Arc, RwLock}};

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

    pub fn send_to_all(&self, bytes: &[i16; 4096]) -> io::Result<()> {
        let mut packet = [0u8; 8192];
        for (i, sample) in bytes.iter().enumerate() {
            let b = sample.to_le_bytes(); // Convert to Little Endian bytes
            packet[i * 2] = b[0];
            packet[i * 2 + 1] = b[1];
        }
        let read_lock = &self.clients.read();
        match read_lock {
            Ok(rwlock) => {
                for client in rwlock.iter() {
                    self.sock.send_to(&packet, client)?;
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
    let addr = format!("127.0.0.1:{}", port);
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

