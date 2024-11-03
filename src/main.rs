use std::process::exit;

use http_server_rs::run;

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
