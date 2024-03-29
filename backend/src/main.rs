#[macro_use]
extern crate dotenv_codegen;

use std::env;

use crate::{
    database::establish_connection,
    utils::{config::Config, logging::init_telemetry, migrations::run_migrations},
};
use actix_files::Files;
use actix_governor::Governor;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{web, App, HttpServer};
use tracing_actix_web::TracingLogger;

pub mod database;
pub mod schema;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_telemetry();

    match run_migrations(&mut establish_connection().get().unwrap()) {
        Ok(_) => {
            log::info!("migrations successfully applied")
        }
        Err(e) => {
            log::error!("migrations failed to apply\n\t{}", e)
        }
    };

    let config = Config::new();

    let pool = establish_connection();
    log::info!("database connection established");
    log::info!(
        "starting HTTP server at http://{}:8080",
        config.addr.as_str()
    );

    let build_mode = env::var("BUILD_MODE").unwrap_or_else(|_| "development".into());
    log::info!("Running in {} mode", build_mode);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(TracingLogger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    config.secret_key.clone(),
                )
                .cookie_name("zion-login".to_string())
                .cookie_secure(false)
                .build(),
            )
            .wrap(Governor::new(&config.rate_limiter_config))
            .configure(database::routes::user::configure)
            .service(
                Files::new("/", "../frontend/dist")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
    })
    .bind((config.addr.as_str(), 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use serial_test::serial;

    #[actix_web::test]
    #[serial]
    async fn test_index_page() {
        let app = test_app!();

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success())
    }
}
