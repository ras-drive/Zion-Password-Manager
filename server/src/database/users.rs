use rand::Rng;

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

    pub fn check_against_unhashed(&self, unhashed_password: String) -> bool {
        let new_hash = hash_with_salt(unhashed_password, self.salt.clone()).1;
        new_hash == self.password_hash
    }
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
