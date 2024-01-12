use std::fmt;

pub enum HTTPStatus {
    Ok,
    Created,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl HTTPStatus {
    fn status_code(&self) -> u16 {
        match self {
            HTTPStatus::Ok => 200,
            HTTPStatus::Created => 201,
            HTTPStatus::BadRequest => 400,
            HTTPStatus::NotFound => 404,
            HTTPStatus::InternalServerError => 500,
        }
    }

    fn reason_phrase(&self) -> &str {
        match self {
            HTTPStatus::Ok => "OK",
            HTTPStatus::Created => "Created",
            HTTPStatus::BadRequest => "Bad Request",
            HTTPStatus::NotFound => "Not Found",
            HTTPStatus::InternalServerError => "Internal Server Error",
        }
    }
}

impl fmt::Display for HTTPStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}",
            self.status_code(),
            self.reason_phrase()
        )
    }
}

pub enum HTTPContentType {
    PlainText,
    File,
}

impl HTTPContentType {
    fn content_type(&self) -> &str {
        match self {
            HTTPContentType::PlainText => "text/plain",
            HTTPContentType::File => "application/octet-stream",
        }
    }
}

impl fmt::Display for HTTPContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Content-Type: {}", self.content_type())
    }
}

pub struct HTTPBody {
    pub body: String,
    pub content_type: HTTPContentType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_status_code_and_phrase() {
        assert_eq!(HTTPStatus::Ok.status_code(), 200);
        assert_eq!(HTTPStatus::Ok.reason_phrase(), "OK");

        assert_eq!(HTTPStatus::Created.status_code(), 201);
        assert_eq!(HTTPStatus::Created.reason_phrase(), "Created");

        assert_eq!(HTTPStatus::BadRequest.status_code(), 400);
        assert_eq!(HTTPStatus::BadRequest.reason_phrase(), "Bad Request");

        assert_eq!(HTTPStatus::NotFound.status_code(), 404);
        assert_eq!(HTTPStatus::NotFound.reason_phrase(), "Not Found");

        assert_eq!(HTTPStatus::InternalServerError.status_code(), 500);
        assert_eq!(
            HTTPStatus::InternalServerError.reason_phrase(),
            "Internal Server Error"
        );
    }

    #[test]
    fn http_status_display_format() {
        assert_eq!(format!("{}", HTTPStatus::Ok), "HTTP/1.1 200 OK");
        assert_eq!(format!("{}", HTTPStatus::Created), "HTTP/1.1 201 Created");
        assert_eq!(
            format!("{}", HTTPStatus::BadRequest),
            "HTTP/1.1 400 Bad Request"
        );
        assert_eq!(
            format!("{}", HTTPStatus::NotFound),
            "HTTP/1.1 404 Not Found"
        );
        assert_eq!(
            format!("{}", HTTPStatus::InternalServerError),
            "HTTP/1.1 500 Internal Server Error"
        );
    }
    #[test]
    fn test_http_content_type() {
        assert_eq!(
            HTTPContentType::File.content_type(),
            "application/octet-stream"
        );
        assert_eq!(HTTPContentType::PlainText.content_type(), "text/plain");
    }

    #[test]
    fn test_http_content_display_format() {
        assert_eq!(
            format!("{}", HTTPContentType::File),
            "Content-Type: application/octet-stream"
        );
        assert_eq!(
            format!("{}", HTTPContentType::PlainText),
            "Content-Type: text/plain"
        );
    }
}
