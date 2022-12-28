#[macro_use]
extern crate dotenv_codegen;

use crate::database::establish_connection;
use actix_files::Files;
use actix_web::{
    http::header::LOCATION, middleware, web, App, HttpResponse, HttpServer, Responder,
};

pub mod database;
pub mod schema;

// #[catch(404)]
pub async fn error_404() -> impl Responder {
    HttpResponse::PermanentRedirect()
        .insert_header((LOCATION, "/errors/error_404.html"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    let pool = establish_connection();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(database::routes::user::configure)
            .service(
                Files::new("/", "../frontend/dist")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_index_page() {
        let app = test::init_service(
            App::new().wrap(middleware::Logger::default()).service(
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
