pub struct ParsedRequest {
    pub method: String,
    pub path: String,
    pub user_agent: String,
    pub content_length: Option<usize>,
    pub body: Option<String>,
}

fn get_request_path(request_line: &str) -> Option<String> {
    request_line
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string())
}

fn get_user_agent(request_line: &str) -> Option<String> {
    request_line
        .split_whitespace()
        .nth(1)
        .map(|s| s.to_string())
}

fn get_request_method(request_line: &str) -> Option<String> {
    request_line
        .split_whitespace()
        .nth(0)
        .map(|s| s.to_string())
}

fn get_content_length(request_line: &str) -> Option<usize> {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    Some(parts[1].parse::<usize>().unwrap_or(0))
}

// fn get_request_body(request_line: &str) -> Option<usize> {

// }

pub fn parse_request(buffer: &[u8]) -> Option<ParsedRequest> {
    let request_str = String::from_utf8_lossy(buffer);
    let request_lines: Vec<&str> = request_str.split_terminator("\r\n").collect();

    let method = get_request_method(&request_lines[0])?;
    let path = get_request_path(&request_lines[0])?;
    let user_agent = get_user_agent(&request_lines[2])?;

    let content_length = if method == "POST" {
        Some(get_content_length(&request_lines[4])?)
    } else {
        None
    };

    let body = if method == "POST" {
        Some(request_lines[7].to_string())
    } else {
        None
    };

    let parsed_request = ParsedRequest {
        method,
        path,
        user_agent,
        content_length,
        body,
    };

    Some(parsed_request)
}
