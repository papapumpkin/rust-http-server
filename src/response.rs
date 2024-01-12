use std::fmt;

use crate::HTTPBody;
use crate::HTTPStatus;

const LINE_FEED: &'static str = "\r\n";

pub struct HTTPResponse {
    pub status: HTTPStatus,
    pub body: Option<HTTPBody>,
}

impl fmt::Display for HTTPResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut response = format!("{}{}", self.status, LINE_FEED);

        if let Some(ref body) = self.body {
            response.push_str(&format!(
                "{}{}Content-Length: {}{}{}{}",
                body.content_type,
                LINE_FEED,
                body.body.len(),
                LINE_FEED,
                LINE_FEED,
                &body.body,
            ));
        }
        response.push_str(LINE_FEED);
        write!(f, "{}", response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HTTPContentType;

    #[test]
    fn test_http_response_without_body() {
        let response = HTTPResponse {
            status: HTTPStatus::Ok,
            body: None,
        };

        let expected_output = format!("HTTP/1.1 200 OK\r\n\r\n");
        assert_eq!(format!("{}", response), expected_output);
    }

    #[test]
    fn test_http_response_with_body() {
        let response = HTTPResponse {
            status: HTTPStatus::NotFound,
            body: Some(HTTPBody {
                body: "Page not found".to_string(),
                content_type: HTTPContentType::PlainText,
            }),
        };

        let expected_output = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 14\r\n\r\nPage not found\r\n"
        );
        assert_eq!(format!("{}", response), expected_output);
    }
}
