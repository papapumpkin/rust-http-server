use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

mod config;
mod http;
mod request;
mod response;

use config::Config;
use http::{HTTPBody, HTTPStatus};
use response::HTTPResponse;

fn handle_connection(mut stream: TcpStream, config: &Config) -> io::Result<()> {
    println!("Accepted new connection!");
    let mut buffer = vec![0u8; config.buffer_size];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let response = if let Some(request) = request::parse_request(&buffer) {
                match request.path.as_str() {
                    "/" => HTTPResponse{
                        status: HTTPStatus::Ok,
                        body: None,
                    },
                    "/echo" => {
                        let to_echo: String = "test".to_string();
                        HTTPResponse {
                            status: HTTPStatus::Ok,
                            body: Some(HTTPBody { body: to_echo }),
                        }
                    }
                    _ => HTTPResponse{
                        status: HTTPStatus::NotFound,
                        body: None,
                    },
                }
            } else {
                HTTPResponse{
                    status: HTTPStatus::InternalServerError,
                    body: None,
                }
            };
            stream.write_all(response.to_string().as_bytes())?;
        }
        Err(_) => {
            let response = HTTPStatus::InternalServerError;
            stream.write_all(response.to_string().as_bytes())?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let config = Config::load().expect("Failed to load configuration");

    println!("Starting server...");
    let address = format!("{}:{}", config.hostname, config.port);
    let listener = TcpListener::bind(&address)?;

    println!("Server listening on {}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream, &config)?,
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}
