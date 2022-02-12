use diesel::{pg::PgConnection, prelude::*};

pub fn create(
    connection: &PgConnection,
    fullname: &str,
    hash: &str,
    something: &str,
    alias: &str,
    repo_id: i32,
) {
    let rapid_entry = crate::db::models::NewRapidEntry {
        fullname,
        hash,
        something,
        alias,
        repo_id,
    };

    diesel::insert_into(crate::db::schema::rapid_entry::table)
        .values(&rapid_entry)
        .execute(connection)
        .expect("Error inserting new rapid entry");
}

pub fn delete(connection: &PgConnection, id_: i32) {
    use crate::db::schema::rapid_entry::dsl::*;

    diesel::delete(rapid_entry.filter(id.eq(id_)))
        .execute(connection)
        .expect("Failed to delete rapid entry");
}

pub fn query(connection: &PgConnection) -> Vec<crate::db::models::RapidEntry> {
    crate::db::schema::rapid_entry::table
        .load::<crate::db::models::RapidEntry>(connection)
        .expect("Error loading rapid entries")
}

pub fn query_for_repo(
    connection: &PgConnection,
    repo_id_: i32,
) -> Vec<crate::db::models::RapidEntry> {
    use crate::db::schema::rapid_entry::dsl::*;

    rapid_entry
        .filter(repo_id.eq(repo_id_))
        .load::<crate::db::models::RapidEntry>(connection)
        .expect("Error loading rapid entries")
}

pub fn find(connection: &PgConnection, md5: &str) -> Option<crate::db::models::RapidEntry> {
    use crate::db::schema::rapid_entry::dsl::*;

    match rapid_entry.filter(hash.eq(md5)).first(connection) {
        Ok(el) => Some(el),
        Err(_) => None,
    }
}

pub fn query_with_name(
    connection: &PgConnection,
    name: &str,
) -> Option<crate::db::models::RapidEntry> {
    use crate::db::schema::rapid_entry::dsl::*;

    match rapid_entry.filter(fullname.eq(name)).first(connection) {
        Ok(el) => Some(el),
        Err(_) => None,
    }
}
