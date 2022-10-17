use mongodb::bson::doc;
use mongodb::options::{ClientOptions, FindOptions, ResolverConfig};
use mongodb::{Client, Collection};
use rand::Rng;
use std::env;

pub mod passwords;

const DB_NAME: &str = "users";
const COLL: &str = "Users";

use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use sha256::digest;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password_hash: String,
    pub salt: String,
}

impl User {
    pub fn new(username: String, unhashed_password: String) -> Self {
        let (salt, hashed_password) = hash_with_salt(unhashed_password, get_salt());
        User {
            username,
            password_hash: hashed_password,
            salt,
        }
    }

    pub async fn insert_user_into_db(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "username": self.username.as_str() };
        let find_options = FindOptions::builder().sort(doc! { "username": 1 }).build();
        let mut cursor = get_users_collection()
            .await
            .find(filter, find_options)
            .await
            .unwrap();

        while let Some(user) = cursor.next().await {
            if self.username == user.unwrap().username {
                return Err("username already in database".into());
            }
        }

        get_users_collection().await.insert_one(self, None).await?;
        Ok(())
    }

    pub fn check_against_unhashed(&self, unhashed_password: String) -> bool {
        let new_hash = hash_with_salt(unhashed_password, self.salt.clone()).1;
        new_hash == self.password_hash
    }
}

pub async fn get_client() -> Client {
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await
            .unwrap();
    Client::with_options(options).unwrap()
}

pub async fn get_users_collection() -> Collection<User> {
    get_client().await.database(DB_NAME).collection(COLL)
}

///
/// # get_salt
/// Generates a salt with sha256 with a strlen of 32 using system
/// entropy and other minor methods to generate a random string.
///
/// # Returns
/// * `salt` - A sha256 salt.
///
///
pub fn get_salt() -> String {
    const STR_LEN: usize = 32;
    let mut rng = rand::thread_rng();

    let mut chars: Vec<u8> = vec![0; STR_LEN];
    for el in &mut chars {
        let mut ch = rng.gen_range(65..117);
        if ch > 90 {
            ch += 6;
        }
        *el = ch;
    }

    String::from_utf8(chars).unwrap()
}

///
/// # hash_with_salt
/// Function is mostly run with the get_salt() to hash or
/// a provided salt for unhashing a previously hashed password.
///
/// # Arguments
/// * `unhashed_password` - The password being hashed.
/// * `salt` - Salt to hash password.
/// # Returns
/// * `salt` - Salt supplied for hashing.
/// * `hashed_password` - A sha256 hash with salt of the unhashed_password variable.
pub fn hash_with_salt(unhashed_password: String, salt: String) -> (String, String) {
    let input = format!("{}{}", salt, unhashed_password);
    let hashed_password = digest(input);

    (salt, hashed_password)
}

///
/// # validate_email_password
///
/// # Arguments
/// * 'email' - Login email
/// * 'password' - password for login
/// # Return
/// Either a result containing a unit type or an error that
/// needs to be handled from invalid passwords and such
///
pub async fn validate_email_password(
    email: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let filter = doc! { "username": email.as_str() };
    let find_options = FindOptions::builder().sort(doc! { "username": 1 }).build();
    let mut cursor = get_users_collection()
        .await
        .find(filter, find_options)
        .await
        .unwrap();

    if let Some(user) = cursor.next().await {
        return match user {
            Ok(u) => {
                if u.check_against_unhashed(password.clone()) {
                    Ok(())
                } else {
                    Err("".into())
                }
            }
            Err(_) => Err("email not found".into()),
        };
    }
    Err("email not registered".into())
}

#[cfg(test)]
mod tests {
    use crate::register_user_handler;
    use actix_web::App;
    use crate::database::get_salt;

    #[actix_web::test]
    async fn test_server_user_db() {
        let name = format!("{}@example.test", get_salt());

        let srv = actix_test::start(||
            App::new().service(register_user_handler)
        );

        let req = srv.get(format!("/register/{}/password", name));
        let res = req.send().await.expect("error getting test response");

        assert!(res.status().is_success());
    }
}
