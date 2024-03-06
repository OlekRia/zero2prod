use std::{io::Error, net::TcpListener};
use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let port = configuration.application_port;

    let address = format!("{}:{}", configuration.application_address, port);
    let listener = TcpListener::bind(address).expect("Cannot bind to listener");
    let _ = run(listener)?.await;
    Ok(())
}
