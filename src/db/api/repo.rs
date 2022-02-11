use diesel::{pg::PgConnection, prelude::*};

pub fn create(connection: &PgConnection, name: &str, url: &str) {
    let new_repo = crate::db::models::NewRepo { name, url };

    diesel::insert_into(crate::db::schema::repo::table)
        .values(&new_repo)
        .execute(connection)
        .expect("Error inserting new repo");
}

pub fn get_all(connection: &PgConnection) -> Vec<crate::db::models::Repo> {
    crate::db::schema::repo::table
        .load::<crate::db::models::Repo>(connection)
        .expect("Error loading repos")
}

pub fn get_by_name(connection: &PgConnection, name_: &str) -> Option<crate::db::models::Repo> {
    use crate::db::schema::repo::dsl::*;

    match repo.filter(name.eq(name_)).first(connection) {
        Ok(el) => Some(el),
        Err(_) => None,
    }
}

pub fn get_by_id(connection: &PgConnection, id_: i32) -> Option<crate::db::models::Repo> {
    use crate::db::schema::repo::dsl::*;

    match repo.filter(id.eq(id_)).first(connection) {
        Ok(el) => Some(el),
        Err(_) => None,
    }
}

pub fn delete(connection: &PgConnection, id_: i32) {
    use crate::db::schema::repo::dsl::*;

    diesel::delete(repo.filter(id.eq(id_)))
        .execute(connection)
        .expect("Failed to delete repo");
}
