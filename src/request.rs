pub struct ParsedRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub user_agent: String,
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

fn get_request_body(request_line: &str) -> Option<String> {
    request_line
        .split_whitespace()
        .nth(0)
        .map(|s| s.to_string())
}

pub fn parse_request(buffer: &[u8]) -> Option<ParsedRequest> {
    let request_str = String::from_utf8_lossy(buffer);
    println!("{}", request_str);
    let request_lines: Vec<&str> = request_str.split_terminator("\r\n").collect();
    parsed_request = ParsedRequest {
        method: get_request_method(&request_lines[0])?,
        path: get_request_path(&request_lines[0])?,
        user_agent: get_user_agent(&request_lines[2])?,
    };
    if parsed_request.method == "GET" {
        parsed_request.body = get_request_body(&request_lines[-1])?
    };
    Some(parsed_request)
}
