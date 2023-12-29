pub struct ParsedRequest {
    pub path: String,
    pub user_agent: String,
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

pub fn parse_request(buffer: &[u8]) -> Option<ParsedRequest> {
    let request_str = String::from_utf8_lossy(buffer);
    println!("{}", request_str);
    let request_lines: Vec<&str> = request_str.split_terminator("\r\n").collect();
    Some(ParsedRequest {
        path: get_request_path(&request_lines[0])?,
        user_agent: get_user_agent(&request_lines[2])?,
    })
}
