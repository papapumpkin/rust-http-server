pub struct ParsedRequest {
    pub path: String,
}

fn get_request_path(request: &str) -> Option<String> {
    request.split_whitespace().nth(1).map(|s| s.to_string())
}

pub fn parse_request(buffer: &[u8]) -> Option<ParsedRequest> {
    let request_str = String::from_utf8_lossy(buffer);
    let request_lines: Vec<&str> = request_str.split_terminator("\r\n").collect();
    Some(ParsedRequest {
        path: get_request_path(&request_lines[0])?,
    })
}
