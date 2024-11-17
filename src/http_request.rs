use crate::headers::Header;
use anyhow::{Context, Result};
use std::collections::HashMap;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct HttpRequest {
    method: Option<HttpMethod>,
    url: Option<String>,
    protocol: Option<String>,
    pub headers: HashMap<Header, String>,
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
        let (req_and_headers, body) = request_string
            .split_once("\r\n")
            .and_then(|(r, b)| Some((r.split("\r\n").collect::<Vec<_>>(), b)))
            .unwrap();

        Ok(Self {
            method: {
                if req_and_headers.len() > 0 {
                    match req_and_headers[0]
                        .split_ascii_whitespace()
                        .collect::<Vec<_>>()[0]
                    {
                        "GET" => Some(HttpMethod::GET),
                        _ => None,
                    }
                } else {
                    None
                }
            },
            url: {
                if req_and_headers.len() > 0 {
                    Some(
                        req_and_headers[0]
                            .split_ascii_whitespace()
                            .collect::<Vec<_>>()[1]
                            .into(),
                    )
                } else {
                    None
                }
            },
            protocol: {
                if req_and_headers.len() > 0 {
                    Some(
                        req_and_headers[0]
                            .split_ascii_whitespace()
                            .collect::<Vec<_>>()[2]
                            .into(),
                    )
                } else {
                    None
                }
            },
            headers: {
                if req_and_headers.len() > 1 {
                    let mut header_map = HashMap::new();

                    req_and_headers.iter().skip(1).for_each(|&s| {
                        match s.split_once(" ") {
                            Some(("Host:", v)) => header_map.insert(Header::Host, v.to_string()),
                            Some(("User-Agent:", v)) => {
                                header_map.insert(Header::UserAgent, v.to_string())
                            }

                            Some(("Accept:", v)) => {
                                header_map.insert(Header::Accept, v.to_string())
                            }
                            Some(_) => unimplemented!(),
                            None => None,
                        };
                    });

                    header_map
                } else {
                    HashMap::new()
                }
            },
            body: Some(body.to_string()),
        })
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
