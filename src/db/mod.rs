use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./testdb.sqlite3";
    SqliteConnection::establish(db).unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_rapid_entry(
    connection: &SqliteConnection,
    fullname: &str,
    hash: &str,
    something: &str,
    alias: &str,
) {
    let rapid_entry = models::NewRapidEntry {
        fullname,
        hash,
        something,
        alias,
    };

    diesel::insert_into(schema::rapid_entry::table)
        .values(&rapid_entry)
        .execute(connection)
        .expect("Error inserting new rapid entry");
}
