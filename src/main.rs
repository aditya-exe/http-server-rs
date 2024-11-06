use http_server_rs::run;
use std::process::exit;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => println!("OK: Server exit"),
        Err(err) => {
            eprintln!("ERR: {err}");
            exit(1)
        }
    }
}
