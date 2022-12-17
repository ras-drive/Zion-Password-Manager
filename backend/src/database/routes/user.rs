use crate::{database::{models::user::User, PgPool, pg_pool_handler}, schema::users::dsl::*};
use rocket::{serde::json::Json, post, response::Responder, State};
use diesel::{prelude::*, associations::HasTable, insert_into};

use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Pbkdf2
};


#[post("/user", format = "application/json", data = "<user>")]
pub fn insert_user(user: Json<User>, dbpool: &State<PgPool>) -> UserResponse {
    let mut conn = pg_pool_handler(dbpool).expect("database connection");
    let mut user = user.0;
    
    let user_id = users::table().select(id).order(id.desc()).limit(1)
        .load::<i32>(&mut conn).expect("suitable id").first().unwrap() + 1;
    
    user.id = user_id;

    let password_string = user.password_hash.clone();
    let unhashed_password = password_string.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = Pbkdf2.hash_password(unhashed_password, &salt).expect("hashed password").to_string();
    user.password_hash = hashed_password;

    match insert_into(users::table()).values(&user).execute(&mut conn) {
        Ok(_) => UserResponse::Created(Json::from(String::from(r#"{{"status": "202", "message": User inserted}}"#))),
        Err(e) => UserResponse::BadRequest(Json::from(format!(r#"{{"status": "400", "message": {e}}}"#))),
    }
}

#[derive(Responder)]
pub enum UserResponse {
    #[response(status = 201)]
    Created(Json<String>),
    #[response(status = 400)]
    BadRequest(Json<String>),
}

#[cfg(test)]
mod tests {
    use rocket::uri;
    use rocket_http::Status;

    use super::*;
    use crate::test_client;

    #[test]
    fn test_user_insert() {
        let json = User::default();

        let client = test_client();
        let request = client.post(uri!("/api/user")).json(&json);
        let response = request.dispatch();

        assert_eq!(response.status(), Status::Created);
    }
}
