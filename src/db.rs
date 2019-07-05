use std::env;
use std::path::Path;

pub use diesel::prelude::*;
pub use diesel::SqliteConnection;

use diesel_migrations;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn = SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    diesel_migrations::run_pending_migrations(&conn).expect("couldn't run migration");

    conn
}
