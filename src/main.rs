use std::fmt;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 1024;

enum HTTPResponse {
    Ok,
    NotFound,
    InternalServerError,
}

impl fmt::Display for HTTPResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            HTTPResponse::Ok => "HTTP/1.1 200 OK\r\n\r\n",
            HTTPResponse::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n",
            HTTPResponse::InternalServerError => "HTTP/1.1 500 Internal Server Error\r\n\r\n",
        };
        write!(f, "{}", message)
    }
}

struct ParsedRequest {
    path: String,
}

fn get_request_path(request: &str) -> Option<String> {
    println!("{}", request);
    request.split_whitespace().nth(1).map(|s| s.to_string())
}

fn parse_request(buffer: &[u8]) -> Option<ParsedRequest> {
    let request_str = String::from_utf8_lossy(buffer);
    let request_lines: Vec<&str> = request_str.split_terminator("\r\n").collect();
    Some(ParsedRequest {
        path: get_request_path(&request_lines[1])?,
    })
}

fn is_valid_path(path: &str) -> bool {
    let valid_paths = ["/"];
    valid_paths.contains(&path)
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    println!("Accepted new connection!");
    let mut buffer = [0; BUFFER_SIZE];

    match stream.read(&mut buffer) {
        Ok(_) => {
            let response = if let Some(request) = parse_request(&buffer) {
                if is_valid_path(&request.path) {
                    HTTPResponse::Ok
                } else {
                    HTTPResponse::NotFound
                }
            } else {
                HTTPResponse::InternalServerError
            };
            stream.write_all(response.to_string().as_bytes())?;
        }
        Err(_) => {
            let response = HTTPResponse::InternalServerError;
            stream.write_all(response.to_string().as_bytes())?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Starting server...");

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    println!("Server listening on port 4221...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream)?,
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
    Ok(())
}
