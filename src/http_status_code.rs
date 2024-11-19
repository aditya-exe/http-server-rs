use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
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
