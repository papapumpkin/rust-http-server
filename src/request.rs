use std::io::{self, Error, ErrorKind};
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

pub async fn parse_request_headers(headers: &str) -> Result<RequestHeaders, Error> {
    headers.split("\r\n").try_fold(
        RequestHeaders {
            method: String::new(),
            path: String::new(),
            user_agent: String::new(),
            content_length: None,
        },
        |mut acc, line| {
            match line.split_once(' ') {
                Some(("GET", path)) | Some(("POST", path)) => {
                    acc.method = line.split_whitespace().next().unwrap_or("").to_string();
                    acc.path = path.split_whitespace().next().unwrap_or("").to_string();
                }
                Some((key, value)) if key == "User-Agent:" => {
                    acc.user_agent = value.to_string();
                }
                Some((key, value)) if key == "Content-Length:" => {
                    acc.content_length = value.parse().ok();
                }
                _ => {}
            }
            Ok(acc)
        },
    )
}

pub async fn parse_stream(stream: &mut TcpStream) -> io::Result<ParsedRequest> {
    let mut headers = String::new();
    let mut header_buffer = [0; 1024];

    while !headers.ends_with("\r\n\r\n") {
        let bytes_read = stream.read(&mut header_buffer).await?;
        if bytes_read == 0 {
            return Err(Error::new(ErrorKind::BrokenPipe, "Connection closed"));
        }
        headers.push_str(&String::from_utf8_lossy(&header_buffer[..bytes_read]));
    }

    let parsed_headers = parse_request_headers(&headers).await?;
    let body_length = parsed_headers.content_length.unwrap_or(0);
    let mut body_bytes = vec![0; body_length];
    stream.read_exact(&mut body_bytes).await?;

    let body_str =
        String::from_utf8(body_bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(ParsedRequest {
        headers: parsed_headers,
        body: if body_str.is_empty() {
            None
        } else {
            Some(body_str)
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_get_request() {
        let headers = "GET /home HTTP/1.1\r\nUser-Agent: TestAgent\r\n\r\n";
        let parsed = parse_request_headers(headers).await.unwrap();
        assert_eq!(parsed.method, "GET");
        assert_eq!(parsed.path, "/home");
        assert_eq!(parsed.user_agent, "TestAgent");
        assert_eq!(parsed.content_length, None);
    }

    #[tokio::test]
    async fn test_parse_post_request_with_content_length() {
        let headers =
            "POST /submit HTTP/1.1\r\nUser-Agent: TestAgent\r\nContent-Length: 15\r\n\r\n";
        let parsed = parse_request_headers(headers).await.unwrap();
        assert_eq!(parsed.method, "POST");
        assert_eq!(parsed.path, "/submit");
        assert_eq!(parsed.user_agent, "TestAgent");
        assert_eq!(parsed.content_length, Some(15));
    }

    #[tokio::test]
    async fn test_parse_malformed_request() {
        let headers = "INVALID REQUEST\r\n";
        let parsed = parse_request_headers(headers).await.unwrap();
        assert_eq!(parsed.method, "");
        assert_eq!(parsed.path, "");
        assert_eq!(parsed.user_agent, "");
        assert_eq!(parsed.content_length, None);
    }
}
