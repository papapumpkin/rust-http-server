use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;

mod config;
mod http;
mod request;
mod response;

use config::Config;
use http::{HTTPBody, HTTPStatus};
use response::HTTPResponse;

fn handle_connection(mut stream: TcpStream, config: Arc<Config>) -> io::Result<()> {
    println!("Accepted new connection!");
    let mut buffer = vec![0u8; config.buffer_size];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let response = if let Some(request) = request::parse_request(&buffer) {
                match request.path.as_str() {
                    "/" => HTTPResponse {
                        status: HTTPStatus::Ok,
                        body: None,
                    },
                    "/user-agent" => {
                        HTTPResponse {
                            status: HTTPStatus::Ok,
                            body: Some(HTTPBody { body: request.user_agent }),
                        }
                    },
                    path if path.starts_with("/echo/") => {
                        let to_echo = &path[6..];
                        let to_echo = to_echo.to_string();
                        HTTPResponse {
                            status: HTTPStatus::Ok,
                            body: Some(HTTPBody { body: to_echo }),
                        }
                    }
                    _ => HTTPResponse {
                        status: HTTPStatus::NotFound,
                        body: None,
                    },
                }
            } else {
                HTTPResponse {
                    status: HTTPStatus::InternalServerError,
                    body: None,
                }
            };
            println!("{}", format!("{}", response));
            stream.write_all(response.to_string().as_bytes())?;
        }
        Err(_) => {
            let response = HTTPResponse {
                status: HTTPStatus::InternalServerError,
                body: None,
            };
            stream.write_all(response.to_string().as_bytes())?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let config = Arc::new(Config::load().expect("Failed to load configuration"));
    println!("Starting server...");
    let address = format!("{}:{}", config.hostname, config.port);
    let listener = TcpListener::bind(&address)?;

    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        let config_clone = config.clone();
        match stream {
            Ok(stream) => {
                let _ = thread::spawn(move || {
                    let _ = handle_connection(stream, config_clone);
                });
            },
            Err(e) => {
                let _ = thread::spawn(move || {
                    eprintln!("Connection failed: {}", e);
                });
            },
        }
    }
    Ok(())
}
