use std::fmt::Display;

#[derive(Debug)]
pub struct HttpResponse {
    pub protocol: String,
    pub status_code: HttpStatusCode,
}

impl HttpResponse {
    pub fn create(protocol: &str, status_code: HttpStatusCode) -> Self {
        Self {
            protocol: protocol.to_owned(),
            status_code,
        }
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\r\n", self.protocol, self.status_code)
    }
}

#[derive(Debug)]
pub enum HttpStatusCode {
    Ok,
    NotFound,
}

impl Display for HttpStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (code, message) = match self {
            Self::Ok => ("200", "OK"),
            Self::NotFound => ("404", "Not Found"),
        };

        write!(f, "{code} {message}")
    }
}
