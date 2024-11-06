mod http_request;
mod http_response;

use anyhow::{Context, Result};
use http_request::HttpRequest;
use http_response::{HttpResponse, HttpStatusCode};
use std::net::SocketAddr;
use tokio::{
    io::AsyncWriteExt,
    net::{TcpListener, TcpStream},
};

pub async fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")
        .await
        .context("TRY: Attempting to bind to port 4221")?;

    loop {
        let (stream, socket) = listener
            .accept()
            .await
            .context("TRY: Accepting new connections")?;

        tokio::spawn(async move { handle_connection(stream, socket).await });
    }
}

async fn handle_connection(mut stream: TcpStream, socket: SocketAddr) -> Result<()> {
    println!("OK: Accepted connection from {socket}");

    let http_req = HttpRequest::new(&mut stream).await?;

    match http_req.url.unwrap().as_str() {
        "/" => {
            let response =
                HttpResponse::create(http_req.protocol.unwrap().as_str(), HttpStatusCode::Ok);

            stream
                .write_all(response.to_string().as_bytes())
                .await
                .context("TRY: Returning response")?;
        }
        _ => {
            let response = HttpResponse::create(
                http_req.protocol.unwrap().as_str(),
                HttpStatusCode::NotFound,
            );

            stream
                .write_all(response.to_string().as_bytes())
                .await
                .context("TRY: Returning response")?;
        }
    }

    Ok(())
}
