use std::error::Error;
use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_files as fs;

mod database;

use crate::database::{test_db, User, validate_email_password};
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
async fn register_user_handler(info: web::Path<(String, String)>) -> impl Responder {
    let (email, password) = info.into_inner();
    let user = User::new(email, password);
    match user.insert_user_into_db().await {
        Ok(_) => {
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("error: {}", e);
            HttpResponse::BadRequest().finish()
        }
    }
}

#[get("/login/{email}/{password}")]
async fn login_user_handler(info: web::Path<(String, String)>) -> impl Responder {
    let (email, password) = info.into_inner();

    return match validate_email_password(email, password).await {
        Ok(_) => {
            HttpResponse::Ok()
        }
        Err(_) => {
            HttpResponse::Forbidden()
        }
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./static/html")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    // test_db().await;

    println!("listening on port 8080");
    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(
                fs::Files::new("/public", ".")
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
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}