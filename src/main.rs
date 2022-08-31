extern crate bcrypt;

use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Path;
use handlebars::Handlebars;
use serde_json::json;
use bcrypt::{DEFAULT_COST, hash, verify};
use crate::database::{check_database_for_id, create_login, establish_connection};

pub mod database;
pub mod schema;

#[get("/")]
async fn index_page_handler(hb: web::Data<Handlebars<'_>>) -> impl Responder {
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

#[get("/register/{email}/{password_hash}")]
async fn register_handler_data<'a>(path: Path<(String, String)>) -> &'a str {
    let _data = json!({});
    // let body = hb.render("register", &data).unwrap();
    let (email, unhashed_password) = path.into_inner();
    let hashed_password = hash(unhashed_password.as_str(), DEFAULT_COST)
        .expect("error while hashing password");

    match verify(unhashed_password.as_str(), hashed_password.as_str()) {
        Ok(_) => {
            // println!("success");
            let mut id = -1;
            let connection = &mut establish_connection();
                if let Ok(data) = check_database_for_id(connection).await {
                id = data;
        }

        if id.gt(&-1){
            create_login(connection, id, email.as_str(), hashed_password.as_str()).expect("error while creating login");
        } else {
            println!("no suitable id found")
        }
    }
        Err(e) => {
            println!("error while validating bcrypt hash for password!\n{}", e);
        }
    }

    "OK"
    // HttpResponse::Ok().body(body)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // For handlebars template rendering
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./static/html")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);


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
            .service(index_page_handler)
            .service(login_page_handler)
            .service(register_page_handler)
            .service(register_handler_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
