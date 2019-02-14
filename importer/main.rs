use rusqlite::types::ToSql;
use rusqlite::{Connection, NO_PARAMS};
use time::Timespec;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

fn main() {
    let conn = Connection::open("db.sqlite3").unwrap();

    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
        NO_PARAMS,
    )
    .unwrap();
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        time_created: time::get_time(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, time_created, data)
                  VALUES (?1, ?2, ?3)",
        &[&me.name as &ToSql, &me.time_created, &me.data],
    )
    .unwrap();

    let mut stmt = conn
        .prepare("SELECT id, name, time_created, data FROM person")
        .unwrap();
    let person_iter = stmt
        .query_map(NO_PARAMS, |row| Person {
            id: row.get(0),
            name: row.get(1),
            time_created: row.get(2),
            data: row.get(3),
        })
        .unwrap();

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
}
