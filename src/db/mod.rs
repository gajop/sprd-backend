use diesel::{prelude::*, pg::PgConnection};

use dotenv::dotenv;
use std::env;

use crate::rapid;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_rapid_entry(
    connection: &PgConnection,
    fullname: &str,
    hash: &str,
    something: &str,
    alias: &str,
    repo_id: i32
) {
    let rapid_entry = models::NewRapidEntry {
        fullname,
        hash,
        something,
        alias,
        repo_id
    };

    diesel::insert_into(schema::rapid_entry::table)
        .values(&rapid_entry)
        .execute(connection)
        .expect("Error inserting new rapid entry");
}

pub fn delete_rapid_entry(
    connection: &PgConnection,
    id_: i32
) {
    use self::schema::rapid_entry::dsl::*;

    diesel::delete(rapid_entry.filter(id.eq(id_)))
        .execute(connection)
        .expect("Failed to delete rapid entry");
}

pub fn delete_repo(
    connection: &PgConnection,
    id_: i32
)
{
    use self::schema::repo::dsl::*;

    diesel::delete(repo.filter(id.eq(id_)))
        .execute(connection)
        .expect("Failed to delete repo entry");
}

pub fn query_rapid_entries(connection: &PgConnection) -> Vec<crate::db::models::RapidEntry> {
    schema::rapid_entry::table
            .load::<super::db::models::RapidEntry>(connection)
            .expect("Error loading tasks")
}

pub fn query_rapid_entries_for_repo(
    connection: &PgConnection,
    repo_id_: i32) ->
    Vec<models::RapidEntry> {
    use self::schema::rapid_entry::dsl::*;

    rapid_entry.filter(repo_id.eq(repo_id_)).load::<models::RapidEntry>(connection)
            .expect("Error loading tasks")
}

pub fn find_rapid_entry(connection: &PgConnection, md5: &str) -> Option<crate::db::models::RapidEntry> {
    use self::schema::rapid_entry::dsl::*;

    match rapid_entry.filter(hash.eq(md5))
            .first(connection) {
        Ok(el) => Some(el),
        Err(_) => None,
    }
}

pub fn find_repo_by_name(connection: &PgConnection, name_: &str) -> Option<crate::db::models::Repo> {
    use self::schema::repo::dsl::*;

    match repo.filter(name.eq(name_))
            .first(connection) {
        Ok(el) => Some(el),
        Err(_) => None,
    }
}

pub fn find_repo_by_id(connection: &PgConnection, id_: i32) -> Option<crate::db::models::Repo> {
    use self::schema::repo::dsl::*;

    match repo.filter(id.eq(id_))
            .first(connection) {
        Ok(el) => Some(el),
        Err(_) => None,
    }
}


pub fn create_repo(
    connection: &PgConnection,
    name: &str,
    url: &str
)
{
    let new_repo = models::NewRepo {
        name,
        url
    };

    diesel::insert_into(schema::repo::table)
        .values(&new_repo)
        .execute(connection)
        .expect("Error inserting new repo");
}

pub fn query_find_rapid_entry(connection: &PgConnection, name: &str) -> Option<crate::db::models::RapidEntry> {
    use self::schema::rapid_entry::dsl::*;

    match rapid_entry.filter(fullname.eq(name)).first(connection) {
        Ok(el) => Some(el),
        Err(_) => None,
    }
}