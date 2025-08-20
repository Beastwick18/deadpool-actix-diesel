use std::env;

use actix_web::{App, HttpServer, middleware, web};
use diesel::{Connection, PgConnection};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use env_logger::Env;
use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
use utoipa_actix_web::{AppExt, scope, service_config::ServiceConfig};
use utoipa_swagger_ui::SwaggerUi;

use crate::routes::AppData;

mod database;
mod routes;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = dotenvy::dotenv();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let url = env::var("DATABASE_URL").expect("Missing DATABASE_URL environment variable");
    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    let mut conn_sync = PgConnection::establish(&url).unwrap();
    conn_sync.run_pending_migrations(MIGRATIONS).unwrap();

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
    let pool = Pool::builder(config).build().unwrap();

    let data = AppData { diesel: pool };

    #[derive(OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }

    HttpServer::new(move || {
        let api_scope = |scp: &mut ServiceConfig| {
            let scope = scope::scope("/api/v1")
                .configure(routes::diesel::init_routes)
                .configure(routes::postgres::init_routes);

            scp.service(scope);
        };

        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .configure(api_scope)
            .app_data(web::Data::new(data.clone()))
            .openapi_service(|api| {
                SwaggerUi::new("/swagger/{_:.*}").url("/api-docs/openapi.json", api)
            })
            .into_app()
            .wrap(middleware::Logger::default())
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
