use super::schema::{repo, rapid_entry};

#[derive(Insertable)]
#[table_name="repo"]
pub struct NewRepo<'a> {
    pub name: &'a str,
    pub url: &'a str
}

#[derive(Queryable, Clone, Debug, Deserialize, Serialize)]
pub struct Repo {
    pub id: i32,
    pub name: String,
    pub url: String
}

#[derive(Insertable)]
#[table_name="rapid_entry"]
pub struct NewRapidEntry<'a> {
    pub fullname: &'a str,
    pub hash: &'a str,
    pub something: &'a str,
    pub alias: &'a str,
    pub repo_id: i32,
}

#[derive(Queryable, Clone, Debug, Deserialize, Serialize)]
pub struct RapidEntry {
    pub id: i32,
    pub repo_id: i32,
    pub fullname: String,
    pub hash: String,
    pub something: String,
    pub alias: String,
}
