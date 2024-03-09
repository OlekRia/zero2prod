use crate::routes;
use actix_web::{dev::Server, middleware, web, App, HttpRequest, HttpServer, Responder};
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .workers(3)
    .run();

    Ok(server)
}
