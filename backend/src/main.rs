#[macro_use]
extern crate dotenv_codegen;

use crate::{database::establish_connection, utils::logging::init_telemetry};
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{web, App, HttpServer};
use cookie::Key;
use tracing_actix_web::TracingLogger;

pub mod database;
pub mod schema;
pub mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_telemetry();

    let secret_key = Key::generate();

    let pool = establish_connection();
    log::info!("database connection established");
    log::info!("starting HTTP server at http://0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(TracingLogger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("zion-login".to_string())
                    .cookie_secure(false)
                    .build(),
            )
            .configure(database::routes::user::configure)
            .service(
                Files::new("/", "../frontend/dist")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
    })
    .bind(("0.0.0.0", 8080))?
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
        /*
        let app = test::init_service(
            App::new().wrap(TracingLogger::default()).service(
                Files::new("/", "../frontend/dist")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            ),
        )
        .await;
        */

        let app = test_app!();

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success())
    }
}
