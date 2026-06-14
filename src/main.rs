use std::env;

use crate::{cmd::properties::{AppMode, properties_file_parse, properties_input_parse}, core::{client, server}};

mod audio;
mod network;
mod core;
mod cmd;

const MIN_ENV_ARGS_COUNT: usize = 2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let app_properties = if args.len() < MIN_ENV_ARGS_COUNT {
        properties_file_parse()
    } else {
        properties_input_parse()
    };

    if let Some(props) = app_properties {
        match props.mode {
            AppMode::Server => {
                server::server_loop(props);
            },
            AppMode::Client => {
                client::client_loop(props);
            }
        }
    } else {
        println!("Application is not started because app properties weren't set");
    }

}
