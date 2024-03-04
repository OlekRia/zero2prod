use std::{io::Error, net::TcpListener};
use zero2prod::run;

const ADDRESS: &str = "127.0.0.1";
const PORT: u16 = 8000;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let address = format!("{}:{}", ADDRESS, PORT);
    let listener = TcpListener::bind(address).expect("Cannot bind to 8000 listener");
    let _ = run(listener)?.await;
    Ok(())
}
