use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;

pub mod database;
pub mod schema;

#[get("/")]
async fn index_handler(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});
    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/login")]
async fn login_handler(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});
    let body = hb.render("login", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/register")]
async fn register_handler(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    let data = json!({});
    let body = hb.render("register", &data).unwrap();
    HttpResponse::Ok().body(body)
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
            .service(index_handler)
            .service(login_handler)
            .service(register_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
