#[macro_use]
extern crate dotenv_codegen;

extern crate core;

use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod database;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/login")]
async fn login_page_handler(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/register")]
async fn register_page_handler(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});
    let body = hb.render("register", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[derive(Deserialize, Serialize)]
struct RegisterUser {
    email: String,
    password: String,
}

#[get("/register/{email}/{password}")]
pub async fn register_user_handler(info: web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/login/{email}/{password}")]
async fn login_user_handler(info: web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "../static/html")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("server starting...");
    println!("listening on port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(
                fs::Files::new("/public", "../")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(index)
            .service(login_page_handler)
            .service(register_page_handler)
            .service(register_user_handler)
            .service(login_user_handler)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
