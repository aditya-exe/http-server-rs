use codecrafters_http_server::HttpServer;
use std::process::exit;

#[tokio::main]
async fn main() {
    let server = HttpServer::new(4221);

    match server.serve().await {
        Ok(()) => println!("OK: Server exit"),
        Err(err) => {
            eprintln!("ERR: {err}");
            exit(1)
        }
    }
}
