use diesel::Connection;
use diesel::pg::PgConnection;

pub mod models;

pub fn establish_connection() -> PgConnection {
  let database_url = dotenv!("DATABASE_URL");

  PgConnection::establish(database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_connection() {
    establish_connection();
  }
}