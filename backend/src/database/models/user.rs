use crate::schema::users;
use diesel::{delete, insert_into, prelude::*};

use super::{super::super::schema::users::dsl::*, super::establish_connection};
use std::io;

use anyhow::Result;

use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};


#[derive(Queryable, Clone, Debug, Insertable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password_hash: String,
}

impl User {
    ///
    /// initializes a new user ready to be inserted into the database. 
    /// before inserting into the database the password is stored plain, 
    /// after inserting pbkdf2 encryption is used to hash the password.
    /// 
    /// ### example
    /// ```
    /// let user = User::new(1, "johndoe@example.com".to_string(), "hunter42".to_string());
    /// ```
    /// 
    pub fn new(
        user_id: i32,
        user_email: String,
        password_string: String,
    ) -> Self {
        Self {
            id: user_id,
            email: user_email,
            password_hash: password_string,
        }
    }

    ///
    /// attempts to write self to user database
    ///
    /// user id is rewritten on insert to line up with database
    /// password_hash is finally rewritten to an encrypted pbkdf2 that can be verified against
    /// 
    pub async fn insert(&mut self) -> Result<()> {
        let mut conn = establish_connection();

        let password = self.password_hash.clone();

        let user_id = users
            .load::<User>(&mut conn)
            .expect("error loading database")
            .len()
            + 1;

        self.id = user_id as i32;

        // all tests are written with an id of a 4 byte max int
        if cfg!(test) {
            self.id = i32::MAX;
        }

        let password_salt = SaltString::generate(&mut OsRng);

        let hashed_password = Pbkdf2.hash_password(self.password_hash.as_bytes(), &password_salt)
            .expect("error while hashing password").to_string();

        self.password_hash = hashed_password;

        insert_into(users).values(self.clone()).execute(&mut conn)?;

        self.verify(password).expect("error while verifying password");

        Ok(())
    }

    ///
    /// tests password_hash against an unhashed password string using pbkdf2 crate
    /// 
    pub fn verify(&self, unhashed_password: String) -> pbkdf2::password_hash::Result<()> {
        let parsed_hash = PasswordHash::new(&self.password_hash).unwrap();

        Pbkdf2.verify_password(unhashed_password.as_bytes(), &parsed_hash)
    }

    ///
    /// gets a result with a User struct from the user table
    /// 
    pub fn get_user_by_id(user_id: i32) -> Result<User> {
        let mut conn = establish_connection();

    match users.filter(users::id.eq(user_id)).load::<User>(&mut conn) {
        Ok(v) => Ok(v.first().unwrap().clone()),
        Err(e) => Err(e.into())
    }
    }

    /// This function mostly exists for test cases and when a user should be deleted a different process should be used which shifts the database.
    ///
    /// ### Safety
    /// This function panics unless run in a test enviroment
    ///
    pub async unsafe fn delete(&mut self, user_id: i32) -> io::Result<User> {
        if !cfg!(test) {
            panic!("must be run in a test case")
        }

        let mut conn = establish_connection();

        let ret = delete(users)
            .filter(id.eq(user_id))
            .load::<User>(&mut conn)
            .expect("error loading database")
            .get(0)
            .expect("error returning user")
            .clone();

        Ok(ret)
    }
}

impl Default for User {
    fn default() -> Self {
        User::new(
            i32::MAX,
            "test@test.com".into(),
            "test".into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;

    // creates a default test user, inserts them into the DB, verifies the unhashed password
    #[tokio::test]
    #[serial]
    async fn test_insert_and_delete_user() {
        let mut user = User::default();
        user.insert().await.expect("error inserting user");

        user.verify(User::default().password_hash).expect("error while verifying password");

        unsafe {
            user.delete(user.id).await.expect("error deleting user");
        }
    }

    // tests the get_user_by_id method
    #[tokio::test]
    #[serial]
    async fn test_get_user() {
        let mut user = User::default();
        user.insert().await.expect("error inserting user");

        User::get_user_by_id(i32::MAX).expect("error retrieving user");

        unsafe {
            user.delete(user.id).await.expect("error deleting user");
        }
    }
}
