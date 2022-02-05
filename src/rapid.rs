use std::error::Error;
use std::path;

#[derive(Debug)]
pub struct Repo {
    pub name: String,
    pub url: String,
}

#[derive(Debug)]
pub struct SDP {
    pub fullname: String,
    pub something: String, // what's the purpose of this field?
    pub md5: String,
    pub alias: String,
}

pub fn parse_repos_from_file(path: &path::Path) -> Result<Vec<Repo>, Box<dyn Error>> {
    let s = crate::gz::read_gz_from_file(path)?;
    parse_repos_from_str(&s)
}

pub fn parse_repos_from_str(s: &str) -> Result<Vec<Repo>, Box<dyn Error>> {
    let mut entries = Vec::new();

    for line in s.lines() {
        let line_entry: Vec<&str> = line.split(',').collect();
        let name = line_entry[0];
        let url = line_entry[1];

        entries.push(Repo {
            name: name.to_string(),
            url: url.to_string(),
        });
    }

    Ok(entries)
}

pub fn read_rapid_from_file(path: &path::Path) -> Result<Vec<SDP>, Box<dyn Error>> {
    let parsed_gz = crate::gz::read_gz_from_file(path)?;
    read_rapid_from_str(&parsed_gz)
}

pub fn read_rapid_from_str(parsed_gz: &str) -> Result<Vec<SDP>, Box<dyn Error>> {
    let mut entries = Vec::new();

    for line in parsed_gz.lines() {
        let line_entry: Vec<&str> = line.split(',').collect();
        if line_entry.len() != 4 {
            println!("MALFORMED FILE");
            continue;
        }
        entries.push(SDP {
            fullname: line_entry[0].to_string(),
            md5: line_entry[1].to_string(),
            something: line_entry[3].to_string(),
            alias: line_entry[2].to_string(),
        });
    }

    Ok(entries)
}
