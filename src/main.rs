use std::io::{self, Write};
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

fn handle_connection(mut stream: TcpStream, _config: Arc<Config>) -> io::Result<()> {
    println!("Accepted new connection!");

    match request::parse_stream(&stream) {
        Ok(request) => {
            let response = match request.headers.path.as_str() {
                "/" => HTTPResponse {
                    status: HTTPStatus::Ok,
                    body: None,
                },
                "/user-agent" => HTTPResponse {
                    status: HTTPStatus::Ok,
                    body: Some(HTTPBody {
                        body: request.headers.user_agent,
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
                    if request.headers.method == "GET" {
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
                    } else if request.headers.method == "POST" {
                        let body = request.body.unwrap();
                        file::write_string_to_file(&full_path, &body)?;

                        HTTPResponse {
                            status: HTTPStatus::Created,
                            body: Some(HTTPBody {
                                body: body,
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
