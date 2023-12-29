use std::fmt;

use crate::HTTPStatus;
use crate::HTTPBody;

const LINE_FEED: &'static str = "\r\n";

pub struct HTTPResponse {
    pub status: HTTPStatus,
    pub body: Option<HTTPBody>,
}

impl fmt::Display for HTTPResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.status)?;

        if let Some(ref body) = self.body {
            write!(
                f,
                "Content-Type: text/plain{}\r\nContent-Length: {}{}\r\n\r\n{}",
                LINE_FEED,
                body.body.len(),
                LINE_FEED,
                body.body
            )
        } else {
            write!(f, "\r\n")
        }
    }
}
