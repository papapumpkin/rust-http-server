use std::io::{self};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

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

pub async fn parse_request_headers(headers: &str) -> RequestHeaders {
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

pub async fn parse_stream(stream: &mut TcpStream) -> io::Result<ParsedRequest> {
    let mut headers = String::new();

    // Read headers asynchronously
    let mut header_buffer = [0; 1024]; // Adjust buffer size as needed
    let mut end_of_headers_found = false;

    while !end_of_headers_found {
        let bytes_read = stream.read(&mut header_buffer).await?;
        if bytes_read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "Connection closed",
            ));
        }

        let chunk = String::from_utf8_lossy(&header_buffer[..bytes_read]);
        if let Some(pos) = chunk.find("\r\n\r\n") {
            let end = pos + 4; // Include the length of "\r\n\r\n"
            headers.push_str(&chunk[..end]);
            end_of_headers_found = true;
        } else {
            headers.push_str(&chunk);
        }
    }

    let parsed_headers = parse_request_headers(&headers).await;

    // Read the body
    let mut body_bytes = vec![0; parsed_headers.content_length.unwrap_or(0)];
    stream.read_exact(&mut body_bytes).await?;

    // Convert body to String
    let body_str =
        String::from_utf8(body_bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(ParsedRequest {
        headers: parsed_headers,
        body: Some(body_str),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_get_request() {
        let headers = "GET /home HTTP/1.1\r\nUser-Agent: TestAgent\r\n\r\n";
        let parsed = parse_request_headers(headers).await;
        assert_eq!(parsed.method, "GET");
        assert_eq!(parsed.path, "/home");
        assert_eq!(parsed.user_agent, "TestAgent");
        assert_eq!(parsed.content_length, None);
    }

    #[tokio::test]
    async fn test_parse_post_request_with_content_length() {
        let headers =
            "POST /submit HTTP/1.1\r\nUser-Agent: TestAgent\r\nContent-Length: 15\r\n\r\n";
        let parsed = parse_request_headers(headers).await;
        assert_eq!(parsed.method, "POST");
        assert_eq!(parsed.path, "/submit");
        assert_eq!(parsed.user_agent, "TestAgent");
        assert_eq!(parsed.content_length, Some(15));
    }

    #[tokio::test]
    async fn test_parse_malformed_request() {
        let headers = "INVALID REQUEST\r\n";
        let parsed = parse_request_headers(headers).await;
        assert_eq!(parsed.method, "");
        assert_eq!(parsed.path, "");
        assert_eq!(parsed.user_agent, "");
        assert_eq!(parsed.content_length, None);
    }
}
