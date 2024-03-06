use sqlx::PgPool;
use std::{io::Error, net::TcpListener};
use zero2prod::{configuration::get_configuration, run};

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let port = configuration.application_port;
    let conn_string = configuration.database.connection_string();
    let connection_pool = PgPool::connect(&conn_string)
        .await
        .expect("Failed to connect Postgres");

    let address = format!("{}:{}", configuration.application_address, port);
    let listener = TcpListener::bind(address).expect("Cannot bind to listener");

    let _ = run(listener, connection_pool)?.await;

    Ok(())
}
