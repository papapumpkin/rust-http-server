use std::io::{self, BufReader, BufRead, Read};
use std::net::TcpStream;

pub struct RequestHeaders {
    pub method: String,
    pub path: String,
    pub user_agent: String,
    pub content_length: Option<usize>,
}

pub struct ParsedRequest {
    pub headers: RequestHeaders,
    pub body: Option<String>,
}


pub fn parse_request_headers(headers: &str) -> RequestHeaders {
    println!("Headers: {}", headers);
    let header_lines: Vec<&str> = headers.split("\r\n").collect();

    let mut method = String::new();
    let mut path = String::new();
    let mut user_agent = String::new();
    let mut content_length = None;

    for line in header_lines.iter() {
        if line.starts_with("GET") || line.starts_with("POST") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            method = parts.get(0).unwrap_or(&"").to_string();
            path = parts.get(1).unwrap_or(&"").to_string();
        } else if line.starts_with("User-Agent:") {
            let parts: Vec<&str> = line.split(": ").collect();
            user_agent = parts.get(1).unwrap_or(&"").to_string();
        } else if line.starts_with("Content-Length:") {
            let parts: Vec<&str> = line.split(": ").collect();
            content_length = parts.get(1).and_then(|s| s.parse::<usize>().ok());
        }
    }

    RequestHeaders {
        method,
        path,
        user_agent,
        content_length,
    }
}


pub fn parse_stream(stream: &TcpStream) -> io::Result<ParsedRequest> {
    let mut reader = BufReader::new(stream);
    let mut headers = String::new();

    // Read headers
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line == "\r\n" {
            break; // End of headers
        }
        headers.push_str(&line);
    }

    let parsed_headers = parse_request_headers(&headers);

    // Read the body
    let mut body_bytes = vec![0; parsed_headers.content_length.unwrap_or(0)];
    reader.read_exact(&mut body_bytes)?;

    // Convert body to String
    let body_str = String::from_utf8(body_bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(ParsedRequest {
        headers: parsed_headers,
        body: Some(body_str),
    })
}
