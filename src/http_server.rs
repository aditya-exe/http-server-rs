use anyhow::Result;
use std::net::SocketAddr;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[derive(Clone)]
pub struct HttpServer {
    address: SocketAddr,
}

impl HttpServer {
    pub async fn run(&self) -> Result<()> {
        let address = self.address;
        let listener = TcpListener::bind(address).await?;
        println!("OK: Listening on {}", address);

        loop {
            let (mut stream, _) = listener.accept().await?;

            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                let bytes_read = stream.read(&mut buffer).await.unwrap();
            });
        }

        todo!()
    }
}
