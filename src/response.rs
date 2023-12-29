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
                "Content-Type: text/plain{}Content-Length: {}{}{}{}",
                LINE_FEED,
                body.body.len(),
                LINE_FEED,
                LINE_FEED,
                &body.body,
            ));
        }
        // response.push_str(LINE_FEED);
        write!(f, "{}", response)
    }
}

