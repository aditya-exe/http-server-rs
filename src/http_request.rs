use anyhow::{Context, Result};
use tokio::{io::AsyncReadExt, net::TcpStream};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Option<HttpMethod>,
    url: Option<String>,
    protocol: Option<String>,
}

impl HttpRequest {
    pub async fn new(stream: &mut TcpStream) -> Result<Self> {
        let mut request = [0u8; 1024];
        let bytes_read = stream
            .read(&mut request)
            .await
            .context("TRY: Reading incoming stream")?;
        let request_string = String::from_utf8_lossy(&request[..bytes_read]);
        let request_vec = request_string.split("\r\n").collect::<Vec<_>>();

        Ok(Self {
            method: {
                if request_vec.len() > 0 {
                    match request_vec[0].split_ascii_whitespace().collect::<Vec<_>>()[0] {
                        "GET" => Some(HttpMethod::GET),
                        _ => None,
                    }
                } else {
                    None
                }
            },
            url: {
                if request_vec.len() > 0 {
                    Some(request_vec[0].split_ascii_whitespace().collect::<Vec<_>>()[1].into())
                } else {
                    None
                }
            },
            protocol: {
                if request_vec.len() > 0 {
                    Some(request_vec[0].split_ascii_whitespace().collect::<Vec<_>>()[2].into())
                } else {
                    None
                }
            },
        })
    }

    pub fn get_url(&self) -> &String {
        self.url.as_ref().unwrap()
    }

    pub fn get_protocol(&self) -> &String {
        self.protocol.as_ref().unwrap()
    }
}

#[derive(Debug)]
pub enum HttpMethod {
    GET,
}
