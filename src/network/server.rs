use std::{collections::HashMap, io, net::{SocketAddr, UdpSocket}, sync::{Arc, RwLock}, time::Instant};

use crate::{audio::AudioParams, core::AUDIO_BUFFER_LENGTH, network::{CLIENT_TIMEOUT, KEEPALIVE_MESSAGE, REGISTER_MESSAGE}};

#[derive(Clone)]
pub struct Server {
    sock: Arc<UdpSocket>,
    clients: Arc<RwLock<HashMap<SocketAddr, Instant>>>,
    audio_params: [u8; 6]
}

impl Server {
    fn new(sock: UdpSocket, audio_params: AudioParams) -> Server {
        let clients = HashMap::new();
        let rw_lock = RwLock::new(clients);
        let arc = Arc::new(rw_lock);
        Server {
            sock: Arc::new(sock),
            clients: arc,
            audio_params: audio_params.to_bytes()
        }
    }

    pub fn send_to_all(&self, bytes: &[u8; AUDIO_BUFFER_LENGTH]) -> io::Result<()> {
        let read_lock = &self.clients.read();
        match read_lock {
            Ok(rwlock) => {
                for client in rwlock.iter() {
                    self.sock.send_to(bytes, client.0)?;
                }
            },
            Err(err) => {
                println!("Error while acquire lock for read: {:?}", err);
            }
        };

        Ok(())
    }

    pub fn check_clients(&self) {
        let now = Instant::now();
        let mut clients_to_remove: Vec<SocketAddr> = Vec::new();
        if let Ok(lock) = self.clients.read() {
            for (client, last_time) in lock.iter() {
                let time_between = now.duration_since(*last_time);
                if time_between >= CLIENT_TIMEOUT {
                    clients_to_remove.push(*client);
                }
            }
        }
        
        if !clients_to_remove.is_empty() {
            self.remove_client(clients_to_remove);
        }
    }

    fn remove_client(&self, clients_to_remove: Vec<SocketAddr>) {
        if let Ok(mut lock) = self.clients.write() {
            for client in clients_to_remove {
                lock.remove(&client);
            }
        }
    }

    pub fn add_client(&self, client: SocketAddr) {
        // Send audio_params to a new client
        let result = self.sock.send_to(&self.audio_params, client);
        if let Err(res) = result {
            println!("Problem with sending audio_params to a client: {:?}", res);
            return;
        }

        let write_lock = self.clients.write();
        match write_lock {
            Ok(mut rwlock) => {
                rwlock.insert(client, Instant::now());           
            },
            Err(err) => {
                println!("Error while acquire lock for read: {:?}", err);
            }
        }
    }

    pub fn update_client(&self, client: SocketAddr) {
        if let Ok(mut write_lock) = self.clients.write() {
            write_lock.entry(client).and_modify(|t| *t = Instant::now());
        }
    }

    pub fn get_sock_arc(&self) -> Arc<UdpSocket> {
        self.sock.clone()
    }
}

// Return struct with opened socket and clients lists
pub fn start_server(port: u16, audio_params: AudioParams) -> io::Result<Server> {
    let addr = format!("0.0.0.0:{}", port);
    let socket = UdpSocket::bind(&addr)?;
    Ok(Server::new(socket, audio_params))
}

pub fn network_server(server: Server) {
    let mut buf: [u8; 1] = [0; 1];
    let socket = server.get_sock_arc();
    while let Ok((_, client_addr)) = socket.recv_from(&mut buf) {
        match buf[0] {
            REGISTER_MESSAGE => {
                server.add_client(client_addr);
                println!("Added to the clients: {:?}", client_addr);
            }
            KEEPALIVE_MESSAGE => {
                server.update_client(client_addr);
            }
            _ => {
                println!("Unknown message type: {:?}", buf[0]);
            }
        }
    }
}

