#![feature(decl_macro, proc_macro_hygiene)]

use std::path::{Path, PathBuf};
use database::establish_connection;
use tokio::io;

#[macro_use] extern crate dotenv_codegen;

use rocket::{fs::NamedFile, get, launch, response::Redirect, routes};

pub mod database;

#[get("/<file..>")]
async fn build_dir(file: PathBuf) -> io::Result<NamedFile> {
    // NamedFile::open(Path::new("../static/").join(file)).await
    NamedFile::open(Path::new("../frontend/dist").join(file)).await
}

#[get("/")]
fn index() -> Redirect {
    Redirect::temporary("/index.html")
}

#[get("/login")]
fn login() -> Redirect {
    Redirect::temporary("/login/login.html")
}

#[get("/register")]
fn register() -> Redirect {
    Redirect::temporary("/register/register.html")
}

#[launch]
fn rocket() -> _ {
    establish_connection();
    rocket::build()
        // .manage()
        .mount("/", routes![build_dir, index, login, register])
}
