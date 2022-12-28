use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

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
    let pool = pool.get()?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection() {
        assert!(establish_connection().get().is_ok())
    }
}
