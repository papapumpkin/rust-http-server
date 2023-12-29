use std::fmt;

pub enum HTTPStatus {
    Ok,
    NotFound,
    InternalServerError,
}

impl HTTPStatus {
    fn status_code(&self) -> u16 {
        match self {
            HTTPStatus::Ok => 200,
            HTTPStatus::NotFound => 404,
            HTTPStatus::InternalServerError => 500,
        }
    }

    fn reason_phrase(&self) -> &str {
        match self {
            HTTPStatus::Ok => "OK",
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

pub struct HTTPBody {
    pub body: String,
}

// impl HTTPBody {
//     pub fn content_type(&self) -> String {
//         "Content-Type: text/plain".to_string()
//     }
//     pub fn content_length(&self) -> String {
//         format!("Content-Length: {}", self.body.len())
//     }
// }
