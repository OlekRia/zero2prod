use reqwest::Client;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_configuration, DatabaseSettings};

pub struct TesApp {
    pub address: String,
    pub db_pool: PgPool,
    _config: DatabaseSettings,
}

async fn make_connection(config: &DatabaseSettings) -> PgConnection {
    PgConnection::connect(&config.connection_string_without_dbname())
        .await
        .expect("Failed to connect to Postgres")
}

async fn create_database(config: &DatabaseSettings) {
    let mut connection = make_connection(config).await;

    let database_query = format!(r#"CREATE DATABASE "{}""#, config.database_name);
    sqlx::query(&database_query)
        .execute(&mut connection)
        .await
        .expect("Failed to create database.");
}

async fn _drop_database(config: &DatabaseSettings) {
    let mut connection = make_connection(config).await;

    let database_query = format!(r#"DROP DATABASE "{}""#, config.database_name);
    sqlx::query(&database_query)
        .execute(&mut connection)
        .await
        .expect("Failed to drop database.");
}

async fn migrate_database(config: &DatabaseSettings) -> sqlx::Pool<sqlx::Postgres> {
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    create_database(config).await;
    migrate_database(config).await
}

async fn spawn_app() -> TesApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = zero2prod::run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server).await;
    TesApp {
        address,
        db_pool: connection_pool,
        _config: configuration.database,
    }
}

async fn make_client() -> (String, Client, TesApp) {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let address = format!("{}", test_app.address);

    (address, client, test_app)
}

////////////////////////////////////////////////////////////////////////

#[tokio::test]
async fn health_check_works() {
    let (address, client, _) = make_client().await;

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let (address, client, app) = make_client().await;

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscriptions.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

    let _ = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_all(&app.db_pool)
        .await
        .expect("Failed to fetch");

    // drop_database(&app.config).await;
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let (address, client, _) = make_client().await;
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
