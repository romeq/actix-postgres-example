mod api;
mod config;
mod database;
mod schema;

use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;
use diesel::{r2d2, PgConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cfg: Config = Config::new("config.toml");

    let manager = r2d2::ConnectionManager::<PgConnection>::new(cfg.database.postgres_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let addr = (cfg.server.host, cfg.server.port.unwrap_or(8080));
    println!("Starting server at {}:{}", addr.0, addr.1);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::new("%a -> %r --- HTTP %s, took %Dms"))
            .service(api::users::statistics)
    })
    .bind(addr)
    .expect("failed to bind")
    .workers(cfg.server.worker_amount.unwrap_or(1))
    .run()
    .await
}