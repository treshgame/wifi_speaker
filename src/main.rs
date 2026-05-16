use std::env;

use crate::core::{client, server};

mod audio;
mod network;
mod core;

const MIN_ENV_ARGS_COUNT: usize = 2;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < MIN_ENV_ARGS_COUNT {
        println!("No arguments were supplied");
        return;
    }

    match args[1].to_lowercase().as_str() {
        "s" | "server" => {
            server::server_loop();
        },
        "c" | "client" => {
            client::client_loop();
        }
        _ => {
            println!("No valid arguments were supplied");
        }
    }
}
