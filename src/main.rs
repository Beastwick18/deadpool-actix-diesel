use actix_web::{App, HttpServer, middleware::Logger, web};
use diesel::{Connection, PgConnection};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use env_logger::Env;

use crate::routes::AppData;

mod database;
mod routes;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let url = std::env::var("DATABASE_URL")
        .unwrap_or("postgresql://postgres:example@localhost/users".to_string());

    let mut conn_sync = PgConnection::establish(&url).unwrap();
    conn_sync.run_pending_migrations(MIGRATIONS).unwrap();

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    let pool = Pool::builder(config).build().unwrap();

    let data = AppData { diesel: pool };

    HttpServer::new(move || {
        App::new()
            .configure(routes::diesel::init_routes)
            .configure(routes::postgres::init_routes)
            .app_data(web::Data::new(data.clone()))
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
