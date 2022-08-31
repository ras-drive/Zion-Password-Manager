use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use diesel::dsl::{CountStar, Select};
use super::schema;
use self::models::{NewLogin, Login};

pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_login(conn: &mut PgConnection, id: i32, email: &str, password_hash: &str)
    -> Result<Login, Box<dyn Error>> {
    use crate::schema::logins;

    let new_login = NewLogin { id, email, password_hash };

    let login = diesel::insert_into(logins::table)
        .values(&new_login)
        .get_result(conn)?;

    Ok(login)
}

pub fn print_database(connection: &mut PgConnection) {
    use self::schema::logins::dsl::*;

    let results = logins
        .load::<Login>(connection)
        .expect("Error loading posts");

    println!("Displaying {} logins", results.len());
    for login in results {
        println!("login id: {},", login.id);
        println!("login email: {}", login.email);
        println!("-----------\n");
        println!("{}", login.password_hash);
    }
}

pub fn delete_login(connection: &mut PgConnection, target: i32) -> Result<usize, Box<dyn Error>> {
    use self::schema::logins::dsl::*;

    let num_deleted = diesel::delete(logins.filter(id.eq(target)))
        .execute(connection)?;

    Ok(num_deleted)
}

pub async fn check_database_for_id(
    connection: &mut PgConnection) -> Result<i32, Box<dyn Error>> {
    use self::schema::logins::dsl::*;

    let results = logins
        .load::<Login>(connection)
        .expect("Error loading posts");

    return if results.is_empty() {
        Ok(0)
    } else {
        Ok(results.get(results.len() - 1).expect("error getting last login").id + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::main;
    use super::*;
    use super::super::register_handler_data;


    #[test]
    fn test_db() {
        let connection = &mut establish_connection();

        let test_id = i32::MAX;
        let test_email = String::from("example@domain.com");
        let test_password_hash = String::from("example_hash");
        let target = test_id;

        create_login(connection, test_id,
                                 test_email.as_str(),
                                 test_password_hash.as_str())
            .expect("error while creating login");

        print_database(connection);

        delete_login(connection, target).expect("error while deleting login");
    }
}
