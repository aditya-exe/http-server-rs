use std::{collections::HashMap, fmt::Display};

pub type Headers = HashMap<Header, String>;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Header {
    ContentType,
    ContentLength,
    Host,
    UserAgent,
    Accept,
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Header::ContentType => "Content-Type",
                Header::ContentLength => "Content-Length",
                Header::Host => "Host",
                Header::UserAgent => "User-Agent",
                Header::Accept => "Accept",
            }
        )
    }
}
