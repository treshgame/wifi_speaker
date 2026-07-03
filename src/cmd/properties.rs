use std::env;

use crate::cmd::DEFAULT_SERVER_PORT;

pub struct AppProperties {
    pub server_addr: String,
    pub server_port: u16,
    pub client_port: u16,
    pub mode: AppMode
}

impl AppProperties {
    pub fn new(server_addr: String, server_port: u16, client_port: u16, mode: AppMode) -> AppProperties {
        AppProperties {
            server_addr, server_port, client_port, mode
        }
    }
}

pub enum AppMode {
    Server,
    Client
}

impl AppMode {
    pub fn from(str: &str) -> Option<AppMode> {
        match str {
            "s" | "server" => Some(AppMode::Server),
            "c" | "client" => Some(AppMode::Client),
            _ => None
        }
    }
}

// Parse properties from console launch params
pub fn properties_input_parse() -> Option<AppProperties> {
    // first arg is a command name
    let mut args = env::args().skip(1);
    println!("Args: {:?}", args);

    let mut app_params = AppProperties::new(
            String::from("127.0.0.1"),
            DEFAULT_SERVER_PORT,
            DEFAULT_SERVER_PORT + 1,
            AppMode::Server
        );

    while let Some(arg_name) = args.next() {
        match arg_name.as_str() {
            "-h" | "--help" => {
                print_help();
                return None;
            }
            "-m" | "--mode" => {
                if let Some(arg_val) = args.next() {
                    if let Some(app_mode) = AppMode::from(&arg_val) {
                        app_params.mode = app_mode;
                    } else {
                        println!("App mode value is illigal: {}", arg_val);
                        return None;
                    }
                } else {
                    println!("There's no value for {}", arg_name);
                    return None;
                }
            },
            "-s" | "--server_port" => {
                if let Some(arg_val) = args.next() {
                    if let Ok(port) = arg_val.parse::<u16>() {
                        app_params.server_port = port;
                    } else {
                        println!("Non valid server port number: {}", arg_val);
                    }
                } else {
                    println!("There's no value for {}", arg_name);
                    return None;
                }
            },
            "-c" | "--client_port" => {
                if let Some(arg_val) = args.next() {
                    if let Ok(port) = arg_val.parse::<u16>() {
                        app_params.client_port = port;
                    } else {
                        println!("Non valid client port number: {}", arg_val);
                    }
                } else {
                    println!("There's no value for {}", arg_name);
                    return None;
                }
            },
            "-d" | "--server_addr" => {
                // no validation since connect function will do it anyway
                if let Some(arg_val) = args.next() {
                    app_params.server_addr = arg_val;
                } else {
                    println!("There's no value for {}", arg_name);
                    return None;
                }
            },
            _ => {
                println!("{arg_name} is illigal argument");
                return None
            }
        }
    }

    Some(app_params)
}

// Find app properties file and parse it
pub fn properties_file_parse() -> Option<AppProperties> {
    None
}

pub fn print_help() {
    // TODO
    println!("Print help function")
}

