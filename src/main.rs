use std::{io::Error, net::TcpListener};
use zero2prod::run;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Cannot bind to 8000 listener");
    let _ = run(listener)?.await;
    Ok(())
}
