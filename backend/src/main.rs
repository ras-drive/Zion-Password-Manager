use std::path::{Path, PathBuf};
use tokio::io;

use rocket::{fs::NamedFile, get, launch, response::Redirect, routes};

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
    rocket::build().mount("/", routes![build_dir, index, login, register])
}
