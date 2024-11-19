use crate::headers::{Header, Headers};
use anyhow::{Context, Result};
use std::collections::HashMap;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct HttpRequest {
    method: Option<HttpMethod>,
    url: Option<String>,
    protocol: Option<String>,
    pub headers: Headers,
    body: Option<String>,
}

impl HttpRequest {
    pub async fn new(stream: &mut TcpStream) -> Result<Self> {
        let mut request = [0u8; 1024];
        let bytes_read = stream
            .read(&mut request)
            .await
            .context("TRY: Reading incoming stream")?;
        let request_string = String::from_utf8_lossy(&request[..bytes_read]);

        let req = request_string.split_once("\r\n").and_then(|(r, b)| {
            Some((
                r.split(" ").collect::<Vec<_>>(),
                b.split_once("\r\n\r\n").and_then(|(h, b)| {
                    Some((
                        h.split("\r\n").collect::<Vec<_>>(),
                        if b.len() > 0 {
                            Some(b.to_owned())
                        } else {
                            None
                        },
                    ))
                }),
            ))
        });

        if let Some((req, headers_and_body)) = req {
            let (raw_headers, body) = headers_and_body.unwrap_or_default();

            Ok(Self {
                method: match req.get(0) {
                    Some(&"GET") => Some(HttpMethod::GET),
                    Some(_) => unimplemented!(),
                    None => None,
                },
                url: match req.get(1) {
                    Some(&url) => Some(url.to_owned()),
                    None => None,
                },
                protocol: match req.get(2) {
                    Some(&protocol) => Some(protocol.to_owned()),
                    None => None,
                },
                headers: raw_headers.into_iter().fold(
                    HashMap::<Header, String>::new(),
                    |mut headers, raw_header| {
                        let _ = match raw_header.split_once(" ") {
                            Some(("Host:", v)) => headers.insert(Header::Host, v.to_string()),
                            Some(("User-Agent:", v)) => {
                                headers.insert(Header::UserAgent, v.to_string())
                            }
                            Some(("Accept:", v)) => headers.insert(Header::Accept, v.to_string()),
                            Some(_) => unimplemented!(),
                            None => None,
                        };

                        headers
                    },
                ),
                body,
            })
        } else {
            panic!();
        }
    }

    pub fn get_url(&self) -> &String {
        self.url.as_ref().unwrap()
    }

    pub fn get_protocol(&self) -> &String {
        self.protocol.as_ref().unwrap()
    }
}

pub enum HttpMethod {
    GET,
}
