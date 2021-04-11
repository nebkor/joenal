use std::env;

use sqlx::{prelude::*, SqliteConnection};

pub async fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = SqliteConnection::connect(&database_url).await.unwrap();

    conn
}
