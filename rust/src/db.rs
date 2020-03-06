use redis::{AsyncCommands, Client, RedisResult};

pub type DbClient = Client;

pub fn make(redis_host: String) -> RedisResult<DbClient> {
    Client::open(redis_host)
}

pub async fn get_long_url(db: &DbClient, key: &str) -> RedisResult<String> {
    let db = db.clone();
    let mut con = db.get_async_connection().await?;
    con.hget(key, "long_url").await
}

pub async fn increment_visit_counter(db: &DbClient, key: &str) -> RedisResult<()> {
    let db = db.clone();
    let mut con = db.get_async_connection().await?;
    con.hincr(key, "visits", 1).await
}

pub async fn get_next_id(db: &DbClient) -> RedisResult<u64> {
    let db = db.clone();
    let mut con = db.get_async_connection().await?;
    con.incr("counter", 1).await
}

pub async fn create_new_url(db: &DbClient, short_url: &str, long_url: &str) -> RedisResult<String> {
    let db = db.clone();
    let mut con = db.get_async_connection().await?;
    con.hset_multiple(short_url, &[("long_url", long_url), ("visits", "0")])
        .await
}
