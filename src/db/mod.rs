use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use rocket_anyhow::Result;

pub type AsyncPool =
    deadpool::managed::Pool<AsyncDieselConnectionManager<diesel_async::AsyncPgConnection>>;

pub fn build_connection_pool() -> Result<AsyncPool> {
    let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(
        std::env::var("DATABASE_URL")?,
    );
    let pool = Pool::builder(config).build()?;
    Ok(pool)
}
