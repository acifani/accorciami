use redis::{AsyncCommands, Client, RedisResult};

use crate::error::Error;

pub type DbClient = Client;
pub type DbResult<T> = Result<T, Error>;

pub fn make(redis_host: String) -> RedisResult<DbClient> {
    Client::open(redis_host)
}

pub async fn get_long_url(db: &DbClient, key: &str) -> DbResult<String> {
    let db = db.clone();
    let mut con = db.get_async_connection().await?;
    con.hget(key, "long_url").await.map_err(|e| Error::from(e))
}

pub async fn increment_visit_counter(db: &DbClient, key: &str) -> DbResult<()> {
    let db = db.clone();
    let mut con = db.get_async_connection().await?;
    con.hincr(key, "visits", 1)
        .await
        .map_err(|e| Error::from(e))
}

pub async fn get_next_id(db: &DbClient) -> DbResult<u64> {
    let db = db.clone();
    let mut con = db.get_async_connection().await?;
    con.incr("counter", 1).await.map_err(|e| Error::from(e))
}

pub async fn create_new_url(db: &DbClient, short_url: &str, long_url: &str) -> DbResult<String> {
    let db = db.clone();
    let mut con = db.get_async_connection().await?;
    con.hset_multiple(short_url, &[("long_url", long_url), ("visits", "0")])
        .await
        .map_err(|e| Error::from(e))
}
