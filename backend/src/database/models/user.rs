use crate::schema::users;
use diesel::prelude::*;
use regex::Regex;

use std::error::Error as StdError;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Clone, Debug, Insertable)]
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
    pub fn new(user_email: String, password_string: String) -> Result<Self, UserError> {
        // HTML standard email verification regex
        let re =
            Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
                .unwrap();

        match re.is_match(&user_email) {
            true => Ok(Self {
                id: 0,
                email: user_email,
                password_hash: password_string,
            }),
            false => Err(UserError::InvalidEmail),
        }
    }
}

impl Default for User {
    fn default() -> Self {
        User::new("test@test.com".into(), "test".into()).unwrap()
    }
}

#[derive(Debug)]
pub enum UserError {
    InvalidEmail,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UserError::InvalidEmail => f.write_str("InvalidEmail"),
        }
    }
}

impl StdError for UserError {
    fn description(&self) -> &str {
        match *self {
            UserError::InvalidEmail => "invalid email supplied",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_user() {
        assert!(User::new("test@test.com".into(), "test".into()).is_ok())
    }

    #[test]
    fn test_user_with_invalid_email() {
        assert!(User::new("this_is_not_a_valid_email".into(), "test".into()).is_err())
    }
}
