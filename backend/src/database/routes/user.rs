use crate::{
    database::{models::user::User, pg_pool_handler, PgPool},
    schema::users::dsl::*,
};
use actix_web::{
    post,
    web::{self, scope, ServiceConfig},
    HttpResponse, Responder,
};
use diesel::{associations::HasTable, insert_into, prelude::*};

use pbkdf2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Pbkdf2,
};

#[post("/user")]
pub async fn insert_user(user: web::Json<User>, dbpool: web::Data<PgPool>) -> impl Responder {
    let mut conn = pg_pool_handler(&dbpool).expect("database connection");
    let mut user = user.0;

    let user_id = *users::table()
        .select(id)
        .order(id.desc())
        .limit(1)
        .load::<i32>(&mut conn)
        .expect("suitable id")
        .first()
        .unwrap_or(&0);

    user.id = user_id + 1;

    let password_string = user.password_hash.clone();
    let unhashed_password = password_string.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Pbkdf2
        .hash_password(unhashed_password, &salt)
        .expect("hashed password")
        .to_string();
    user.password_hash = hashed_password;

    match insert_into(users::table()).values(&user).execute(&mut conn) {
        Ok(r) => {
            if r > 1 || r == 0 {
                log::info!("No rows were affected")
            }
        }
        Err(e) => log::info!("database panicked\n{}", e),
    };

    HttpResponse::Ok().json(user)
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(scope("/api").service(insert_user));
}

#[cfg(test)]
mod tests {
    use crate::database::establish_connection;

    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_user_insert() {
        let pool = establish_connection();
        let json = User::default();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
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
}
