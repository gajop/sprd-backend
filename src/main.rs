#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;

mod db;
use db::{api, models::RapidEntry, models::Repo};

mod gz;
mod rapid;

use rocket::serde::json::Json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RapidEntries(Vec<RapidEntry>);

#[get("/rapid_entries")]
fn rapid_entries_get() -> Json<RapidEntries> {
    let mut response = RapidEntries(Vec::new());

    let conn = db::establish_connection();
    for rapid_entry in db::query_rapid_entries(&conn) {
        response.0.push(rapid_entry);
    }
    Json(response)
}

#[get("/")]
fn index() -> Json<RapidEntries> {
    rapid_entries_get()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SDPQuery {
    rapid: RapidEntry,
    repo: Repo,
}

#[get("/sdp/<name>")]
fn sdp_get(name: &str) -> Json<Option<SDPQuery>> {
    let conn = db::establish_connection();
    let rapid_entry = db::find_rapid_entry(&conn, name);
    match rapid_entry {
        Some(rapid_entry) => {
            let repo = db::api::repo::get_by_id(&conn, rapid_entry.repo_id).unwrap();
            let result = SDPQuery {
                rapid: rapid_entry,
                repo,
            };
            Json(Some(result))
        }
        None => Json(None),
    }
}

#[get("/rapid_entry/<name>")]
fn rapid_entry_get(name: &str) -> Json<Option<SDPQuery>> {
    let conn = db::establish_connection();
    let rapid_entry = db::find_rapid_entry(&conn, name);
    match rapid_entry {
        Some(rapid_entry) => {
            let repo = api::repo::get_by_id(&conn, rapid_entry.repo_id).unwrap();
            let result = SDPQuery {
                rapid: rapid_entry,
                repo,
            };
            Json(Some(result))
        }
        None => Json(None),
    }
}

#[get("/rapid/<name>")]
fn rapid_search_get(name: &str) -> Json<Option<RapidEntry>> {
    let conn = db::establish_connection();
    let rapid = db::query_find_rapid_entry(&conn, name);
    Json(rapid)
}

#[get("/repo/<name>")]
fn get_repo(name: &str) -> Json<Option<Repo>> {
    let conn = db::establish_connection();
    let repo = api::repo::get_by_name(&conn, name);
    match repo {
        Some(repo) => Json(Some(repo)),
        None => Json(None),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReposResponse(Vec<Repo>);

#[get("/repo")]
fn get_repos() -> Json<ReposResponse> {
    let conn = db::establish_connection();
    Json(ReposResponse(api::repo::get_all(&conn)))
}

#[rocket::main]
async fn main() {
    let _connection = db::establish_connection();

    rocket::build()
        .mount(
            "/",
            routes![
                index,
                rapid_entries_get,
                rapid_search_get,
                sdp_get,
                rapid_entry_get,
                get_repo,
                get_repos,
            ],
        )
        .launch()
        .await
        .unwrap();
}
