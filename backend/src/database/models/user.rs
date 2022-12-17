use crate::schema::users;
use diesel::prelude::*;

use rocket::serde::{Serialize, Deserialize};

#[derive( Serialize, Deserialize, Queryable, Clone, Debug, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
}

impl User {
    ///
    /// initializes a new user ready to be inserted into the database.
    /// after inserting, pbkdf2 encryption is used to hash the password.
    ///
    /// ### example
    /// ```
    /// let user = User::new(1, "johndoe@example.com".to_string(), "hunter42".to_string());
    /// ```
    ///
    pub fn new(user_email: String, password_string: String) -> Self {
        Self {
            id: 0,
            email: user_email,
            password_hash: password_string,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("test@test.com".into(), "test".into())
    }
}
