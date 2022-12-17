#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate dotenv_codegen;

use std::path::{Path, PathBuf};
use tokio::io;

use rocket::{fs::NamedFile, get, launch, response::Redirect, routes, local::blocking::Client};

pub mod database;
pub mod schema;

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
    rocket::build()
        .attach(database::stage())
        .mount("/", routes![build_dir, index, login, register])
}

pub fn test_client() -> Client {
    Client::tracked(rocket::build()
        .attach(database::stage())
        .mount("/", routes![build_dir, index, login, register])).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::uri;

    #[test]
    fn test_index_page() {
        let client = test_client();
        let req = client.get(uri!("/index.html"));
        let response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_login_page() {
        let client = test_client();
        let req = client.get(uri!("/login/login.html"));
        let response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_register_page() {
        let client = test_client();
        let req = client.get(uri!("/register/register.html"));
        let response = req.dispatch();

        assert_eq!(response.status(), Status::Ok);
    }
}
