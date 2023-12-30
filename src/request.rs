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

fn get_request_path(request_line: &str) -> String {
    request_line
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string()).unwrap()
}

fn get_user_agent(request_line: &str) -> String {
    request_line
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string()).unwrap()
}

fn get_request_method(request_line: &str) -> String {
    request_line
        .split_whitespace()
        .nth(0)
        .map(|s| s.to_string()).unwrap()
}

fn get_content_length(request_line: &str) -> usize {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    parts[1].parse::<usize>().unwrap_or(0)
}

pub fn parse_request_headers(headers: &str) -> RequestHeaders {
    let header_lines: Vec<&str> = headers.split("\r\n").collect();
    let method = get_request_method(header_lines[0]);
    let path = get_request_path(header_lines[0]);
    let user_agent = get_user_agent(header_lines[2]);

    let content_length = if method == "POST" {
        Some(get_content_length(header_lines[4]))
    } else {
        None
    };

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
