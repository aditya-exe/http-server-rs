use crate::{
    headers::{Header, Headers},
    http_status_code::HttpStatusCode,
};
use anyhow::{Context, Result};
use std::{collections::HashMap, fmt::Display};
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[derive(Debug, Clone, Copy)]
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
            headers: HashMap::new(),
            body: None,
        }
    }

    pub async fn send(&self, stream: &mut TcpStream) -> Result<()> {
        let body = self.get_body();
        Ok(stream
            .write_all(
                format!(
                    "{} {}\r\n{}\r\n\r\n{}",
                    self.protocol,
                    self.status_code,
                    self.headers
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join("\r\n"),
                    body
                )
                .as_bytes(),
            )
            .await
            .context("TRY: Returning response")?)
    }

    pub fn add_header(mut self, header: Header, val: String) -> Self {
        self.headers.insert(header, val);
        self
    }

    pub fn add_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn get_body(self) -> String {
        self.body.as_ref().unwrap().to_owned()
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let response_string = format!(
            "{} {}\r\n{}\r\n\r\n{}",
            self.protocol,
            self.status_code,
            self.headers
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<_>>()
                .join("\r\n"),
            self.body.as_ref().unwrap()
        );

        write!(f, "{response_string}")
    }
}
