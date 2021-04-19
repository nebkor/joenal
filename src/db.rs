use std::env;

use sqlx::{prelude::*, SqlitePool};

pub async fn make_pool() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = SqlitePool::connect(&database_url).await.unwrap();

    conn
}
