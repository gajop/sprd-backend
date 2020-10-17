use super::schema::rapid_entry;

#[derive(Insertable)]
#[table_name = "rapid_entry"]
pub struct NewRapidEntry<'a> {
    pub fullname: &'a str,
    pub hash: &'a str,
    pub something: &'a str,
    pub alias: &'a str,
}
