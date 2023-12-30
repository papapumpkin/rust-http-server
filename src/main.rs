use std::io::{self, Read, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::Arc;
use std::thread;

mod cli;
mod config;
mod file;
mod http;
mod request;
mod response;

use config::Config;
use http::{HTTPBody, HTTPContentType, HTTPStatus};
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
                    "/user-agent" => HTTPResponse {
                        status: HTTPStatus::Ok,
                        body: Some(HTTPBody {
                            body: request.user_agent,
                            content_type: HTTPContentType::PlainText,
                        }),
                    },
                    path if path.starts_with("/echo/") => {
                        let to_echo = &path[6..];
                        let to_echo = to_echo.to_string();
                        HTTPResponse {
                            status: HTTPStatus::Ok,
                            body: Some(HTTPBody {
                                body: to_echo,
                                content_type: HTTPContentType::PlainText,
                            }),
                        }
                    }
                    path if path.starts_with("/files/") => {
                        let directory =
                            cli::get_cli_arg_by_name("--directory").expect("Argument not found");

                        let safe_filename = file::parse_filename_from_request_path(&path)
                            .expect("Invalid filename in request");

                        let full_path = Path::new(&directory).join(safe_filename);
                        println!("Full path to file: {}", full_path.display());
                        if request.method == "GET" {
                            let result = match file::read_file_to_string(&full_path) {
                                Some(file_content) => HTTPResponse {
                                    status: HTTPStatus::Ok,
                                    body: Some(HTTPBody {
                                        body: file_content,
                                        content_type: HTTPContentType::File,
                                    }),
                                },
                                None => HTTPResponse {
                                    status: HTTPStatus::NotFound,
                                    body: None,
                                },
                            };
                            result
                        } else if request.method == "POST" {
                            let mut reader = BufReader::new(&mut stream);
                            let mut body = String::new();

                            // Use read_to_string to read until EOF
                            reader.read_to_string(&mut body).unwrap_or_else(|e| {
                                eprintln!("Error reading request body: {}", e);
                                0 // Handling the error by returning 0 bytes read
                            });
                            file::write_string_to_file(&full_path, &body)?;

                            HTTPResponse {
                                status: HTTPStatus::Created,
                                body: Some(HTTPBody {
                                    body: body.to_string(),
                                    content_type: HTTPContentType::File,
                                }),
                            }
                        } else {
                            HTTPResponse {
                                status: HTTPStatus::BadRequest,
                                body: None,
                            }
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
            }
            Err(e) => {
                let _ = thread::spawn(move || {
                    eprintln!("Connection failed: {}", e);
                });
            }
        }
    }
    Ok(())
}
