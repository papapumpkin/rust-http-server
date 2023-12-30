pub struct ParsedRequest {
    pub method: String,
    pub path: String,
    pub user_agent: String,
    pub content_length: usize,
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

pub fn parse_request(buffer: &[u8]) -> Option<ParsedRequest> {
    let request_str = String::from_utf8_lossy(buffer);
    let request_lines: Vec<&str> = request_str.split_terminator("\r\n").collect();

    let method = get_request_method(&request_lines[0])?;
    let path = get_request_path(&request_lines[0])?;
    let user_agent = get_user_agent(&request_lines[2])?;
    let content_length = get_content_length(&request_lines[4])?;

    let parsed_request = ParsedRequest {
        method,
        path,
        user_agent,
        content_length,
    };

    Some(parsed_request)
}
