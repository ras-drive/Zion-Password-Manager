#[macro_use]
extern crate dotenv_codegen;

use crate::database::establish_connection;
use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{web, App, HttpServer};
use cookie::Key;
use tracing_actix_web::TracingLogger;

pub mod database;
pub mod schema;

/*
#[catch(404)]
pub async fn error_404() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .insert_header((LOCATION, "/errors/error_404.html"))
        .finish()
}
*/

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

fn init_telemetry() {
    use std::{fs::File, sync::Arc};
    use tracing_subscriber::{filter, prelude::*};

    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // A layer that logs events to a file.
    let file = File::create("debug.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    // A layer that collects metrics using specific events.
    let metrics_layer = /* ... */ filter::LevelFilter::INFO;

    tracing_subscriber::registry()
        .with(
            stdout_log
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(debug_log)
                // Add a filter to *both* layers that rejects spans and
                // events whose targets start with `metrics`.
                .with_filter(filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("metrics")
                })),
        )
        .with(
            // Add a filter to the metrics label that *only* enables
            // events whose targets start with `metrics`.
            metrics_layer.with_filter(filter::filter_fn(|metadata| {
                metadata.target().starts_with("metrics")
            })),
        )
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use serial_test::serial;

    #[actix_web::test]
    #[serial]
    async fn test_index_page() {
        init_telemetry();

        let app = test::init_service(
            App::new().wrap(TracingLogger::default()).service(
                Files::new("/", "../frontend/dist")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            ),
        )
        .await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success())
    }
}
