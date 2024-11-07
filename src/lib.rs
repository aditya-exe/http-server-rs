mod http_request;
mod http_response;
mod router;

use anyhow::{Context, Result};
use http_request::HttpRequest;
use http_response::{Header, HttpResponse, HttpStatusCode};
use std::net::SocketAddr;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

pub struct HttpServer {
    port: usize,
}

impl HttpServer {
    pub fn new(port: usize) -> Self {
        Self { port }
    }

    pub async fn serve(self) -> Result<()> {
        let listener = TcpListener::bind("127.0.0.1:4221")
            .await
            .context("TRY: Attempting to bind to port 4221")?;

        loop {
            let (stream, socket) = listener
                .accept()
                .await
                .context("TRY: Accepting new connections")?;

            tokio::spawn(async move { Self::handle_connection(stream, socket).await });
        }
    }

    async fn handle_connection(mut stream: TcpStream, socket: SocketAddr) -> Result<()> {
        println!("OK: Accepted connection from {socket}");

        let http_req = HttpRequest::new(&mut stream).await?;
        let url = http_req.get_url().as_str();

        match url {
            "/" => {
                let response =
                    HttpResponse::create(http_req.get_protocol().as_str(), HttpStatusCode::Ok);

                response.send(&mut stream).await?;
            }
            path if url.starts_with("/echo/") => {
                let slug = path.split_once("/echo/").unwrap().1;
                let response =
                    HttpResponse::create(http_req.get_protocol().as_str(), HttpStatusCode::Ok)
                        .add_header(Header::ContentType("text/plain".into()))
                        .add_header(Header::ContentLength(slug.len()))
                        .add_body(format!("{}", slug.len()));

                response.send(&mut stream).await?;
            }
            _ => {
                let response = HttpResponse::create(
                    http_req.get_protocol().as_str(),
                    HttpStatusCode::NotFound,
                );

                response.send(&mut stream).await?;
            }
        }

        stream
            .flush()
            .await
            .context("TRY: Flushing out the stream")?;

        Ok(())
    }
}
