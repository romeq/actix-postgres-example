mod api;
mod config;
mod database;
mod errors;
mod models;
mod schema;

use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web::Data, App, HttpServer};
use config::Config;
use diesel::{r2d2, PgConnection};
use rand::{thread_rng, Rng};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
fn key() -> Key {
    let mut key = [0u8; 1024];
    thread_rng().fill(&mut key);
    Key::from(&key)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cfg: Config = Config::new("config.toml");
    let addr = (cfg.server.host, cfg.server.port.unwrap_or(8080));

    let manager = r2d2::ConnectionManager::<PgConnection>::new(cfg.database.postgres_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    println!("Starting server at {}:{}", addr.0, addr.1);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(database::db::Database { db: pool.clone() }))
            .wrap(Logger::new("%a -> %r --- HTTP %s, took %Dms").log_target("req"))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), key()))
            .service(
                actix_web::web::scope("/user")
                    .service(api::users::new_account)
                    .service(api::users::show_user)
                    .service(api::users::login),
            )
            .service(actix_web::web::scope("").service(api::statistics::statistics))
    })
    .bind(addr)
    .expect("failed to bind")
    .workers(cfg.server.worker_amount.unwrap_or(2))
    .shutdown_timeout(10)
    .run()
    .await
}
