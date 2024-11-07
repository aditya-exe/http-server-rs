use anyhow::{Context, Result};
use std::fmt::Display;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[derive(Debug)]
pub struct HttpResponse {
    pub protocol: String,
    pub status_code: HttpStatusCode,
    pub headers: Headers,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn create(protocol: &str, status_code: HttpStatusCode) -> Self {
        Self {
            protocol: protocol.to_owned(),
            status_code,
            headers: Headers(vec![]),
            body: None,
        }
    }

    pub async fn send(&self, stream: &mut TcpStream) -> Result<()> {
        Ok(stream
            .write_all(self.to_string().as_bytes())
            .await
            .context("TRY: Returning response")?)
    }

    pub fn add_header(mut self, header: Header) -> Self {
        self.headers.0.push(header);
        self
    }

    pub fn add_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = if let Some(body) = &self.body {
            body
        } else {
            ""
        };

        write!(
            f,
            "{} {}\r\n{}\r\n{}",
            self.protocol, self.status_code, self.headers, body
        )
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

#[derive(Debug)]
pub struct Headers(Vec<Header>);

impl Display for Headers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.0.iter().for_each(|header| {
            s.push_str(format!("{}\r\n", header.to_string()).as_str());
        });

        write!(f, "{s}\r\n")
    }
}

#[derive(Debug)]
pub enum Header {
    ContentType(String),
    ContentLength(usize),
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = match self {
            Header::ContentType(s) => format!("Content-Type: {}", s),
            Header::ContentLength(length) => format!("Content-Length: {}", length),
        };

        write!(f, "{w}")
    }
}
