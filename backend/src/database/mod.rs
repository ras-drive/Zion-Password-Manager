use rocket::fairing::AdHoc;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };

use crate::database::routes::user::insert_user;

pub mod models;
pub mod routes;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    #[cfg(test)]
    let database_url = dotenv!("TEST_DATABASE_URL");

    #[cfg(not(test))]
    let database_url = dotenv!("DATABASE_URL");

    init_pool(database_url).expect("database pool")
}

fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}


pub fn pg_pool_handler(pool: &PgPool) -> Result<PgPooledConnection, PoolError> {
    let pool = pool.get().expect("connection pool");
    Ok(pool)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Database Stage", |rocket| async {
        rocket
            .manage(establish_connection())
            .mount("/api", rocket::routes![insert_user])
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_client;

    #[test]
    fn test_connection() {
        let client = test_client();
        let pool = client.rocket().state::<PgPool>();

        assert!(pool.is_some())
    }
}
