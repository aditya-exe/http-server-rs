use anyhow::{Context, Result};
use tokio::{io::AsyncWriteExt, net::TcpListener};

pub async fn run() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")
        .await
        .context("TRY: Attempting to bind to port 4221")?;

    loop {
        let (mut stream, _socket) = listener
            .accept()
            .await
            .context("TRY: Accepting new connections")?;

        let response = "HTTP/1.1 200 OK\r\n\r\n";
        
        stream
            .write_all(response.as_bytes())
            .await
            .context("TRY: Writing all response data")?;
        stream.flush().await.context("TRY: Flushing write")?;
    }
}
