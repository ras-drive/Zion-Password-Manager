use crate::{
    database::{models::user::User, pg_pool_handler, routes::ServiceError, PgPool},
    schema::users::dsl::*,
};
use actix_web::{
    get, post, services,
    web::{self, scope, ServiceConfig},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use diesel::{associations::HasTable, insert_into, prelude::*};

use actix_identity::Identity;
use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};
use serde::{Deserialize, Serialize};

#[post("/user")]
pub async fn insert_user(user: web::Json<AuthData>, dbpool: web::Data<PgPool>) -> impl Responder {
    let mut conn = pg_pool_handler(&dbpool).expect("database connection");
    let user = user.0;

    let user_id = *users::table()
        .select(id)
        .order(id.desc())
        .limit(1)
        .load::<i32>(&mut conn)
        .expect("suitable id")
        .first()
        .unwrap_or(&0);

    let password_string = user.password.clone();
    let unhashed_password = password_string.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Pbkdf2
        .hash_password(unhashed_password, &salt)
        .expect("hashed password")
        .to_string();

    let user = User {
        id: user_id + 1,
        email: user.email,
        password_hash: hashed_password,
    };

    match insert_into(users::table()).values(&user).execute(&mut conn) {
        Ok(r) => {
            if r > 1 || r == 0 {
                log::info!("No rows were affected")
            }
        }
        Err(e) => log::info!("database panicked\n{}", e),
    };

    HttpResponse::Ok()
        .message_body("user successfully registered")
        .expect("http response")
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(scope("/api").service(services![
        insert_user,
        login,
        logout,
        get_user_by_session
    ]));
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

impl From<User> for AuthData {
    fn from(user: User) -> Self {
        Self {
            email: user.email,
            password: user.password_hash,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]

pub struct LoggedUser {
    email: String,
}

impl From<User> for LoggedUser {
    fn from(user: User) -> Self {
        Self {
            email: user.email,
        }
    }
}

#[get("/login/user")]
pub async fn get_user_by_session(identity: Identity) -> impl Responder {
    println!("id: {:?}", identity.id().unwrap());

    HttpResponse::Ok()
}

#[post("/login/user")]
pub async fn login(
    req: HttpRequest,
    auth_data: web::Json<AuthData>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || query(auth_data.into_inner(), pool)).await??;

    let user_string = serde_json::to_string(&user).unwrap();
    Identity::login(&req.extensions(), user_string).unwrap();

    Ok(HttpResponse::NoContent().finish())
}

#[get("/login/logout")]
pub async fn logout(identity: Identity) -> HttpResponse {
    identity.logout();
    // HttpResponse::NoContent().finish()
    HttpResponse::Found().append_header(("Location", "/")).finish()
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    let parsed_hash = PasswordHash::new(hash).unwrap();

    match Pbkdf2.verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(pbkdf2::password_hash::Error::Password) => Ok(false),
        Err(_) => Err(ServiceError::InternalServerError),
    }
}

fn query(auth_data: AuthData, pool: web::Data<PgPool>) -> Result<LoggedUser, ServiceError> {
    let mut conn = pool.get().unwrap();

    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(&mut conn)
        .expect("users");

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.password_hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

#[cfg(test)]
mod tests {
    use crate::database::establish_connection;

    use super::*;
    use actix_identity::IdentityMiddleware;
    use actix_web::{test, App};
    use serial_test::serial;

    #[actix_web::test]
    #[serial]
    async fn test_user_insert() {
        let pool = establish_connection();
        let json: AuthData = User::default().into();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(IdentityMiddleware::default())
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/user")
            .set_json(json)
            .to_request();
        let resp = test::call_service(&app, req).await;

        println!("{}", resp.status());
        assert!(resp.status().is_success())
    }

    #[actix_web::test]
    #[serial]
    async fn test_user_login() {
        let pool = establish_connection();
        let json: AuthData = User::default().into();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(IdentityMiddleware::default())
                .configure(configure),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/api/user")
            .set_json(json.clone())
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let req = test::TestRequest::post()
            .uri("/api/login/user")
            .set_json(json)
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success())
    }
}
