use std::time::Duration;

pub mod server;
pub mod client;

pub const PING_INTERVAL: Duration = Duration::from_secs(5);
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(15);

pub const REGISTER_MESSAGE: u8 = 0;
pub const KEEPALIVE_MESSAGE: u8 = 1;
