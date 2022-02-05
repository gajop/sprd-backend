// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate diesel;

mod db;
use db::models::RapidEntry;
use db::models::Repo;

mod gz;
mod rapid;

use rocket::serde::json::Json;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonApiResponse {
    data: Vec<RapidEntry>,
}

#[get("/rapid_entries")]
fn rapid_entries_get() -> Json<JsonApiResponse> {
    let mut response = JsonApiResponse { data: vec![] };

    let conn = db::establish_connection();
    for rapid_entry in db::query_rapid_entries(&conn) {
        response.data.push(rapid_entry);
    }
    Json(response)
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
            let repo = db::find_repo_by_id(&conn, rapid_entry.repo_id).unwrap();
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
            let repo = db::find_repo_by_id(&conn, rapid_entry.repo_id).unwrap();
            let result = SDPQuery {
                rapid: rapid_entry,
                repo,
            };
            Json(Some(result))
        }
        None => Json(None),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct APIRapidEntry {
    rapid_entry: Option<RapidEntry>,
}

#[get("/rapid/<name>")]
fn rapid_search_get(name: &str) -> Json<APIRapidEntry> {
    let conn = db::establish_connection();
    let rapid = db::query_find_rapid_entry(&conn, name);
    Json(APIRapidEntry { rapid_entry: rapid })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            rapid_entries_get,
            rapid_search_get,
            sdp_get,
            rapid_entry_get
        ],
    )
}
