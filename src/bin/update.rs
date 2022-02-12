use std::{collections::HashMap, error::Error};

use clap::{App, Arg};
use db::api;

use std::path;

use sprd_backend::db;
use sprd_backend::rapid;
use sprd_backend::rapid_store;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("updater")
        .version("0.1.0")
        .author("Gajo Petrovic <gajopetrovic@gmail.com>")
        .about("Rapid client")
        .arg(
            Arg::new("root-folder")
                .long("root-folder")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let rapid_store = rapid_store::RapidStore {
        root_folder: &path::PathBuf::from(matches.value_of("root-folder").unwrap()),
    };

    let repos = rapid::parse_repos_from_file(&rapid_store.get_registry_path())?;
    let conn = db::establish_connection();

    let mut added = 0;
    let mut deleted = 0;
    for repo in repos {
        let sdps = match rapid::read_rapid_from_file(&rapid_store.get_repo_path(&repo)) {
            Ok(sdps) => sdps,
            Err(err) => {
                println!("Failed to parse repository {}: {:?}", repo.name, err);
                continue;
            }
        };

        let repo_db = match api::repo::get_by_name(&conn, &repo.name) {
            Some(repo_db) => repo_db,
            None => {
                api::repo::create(&conn, &repo.name, &repo.url);
                api::repo::get_by_name(&conn, &repo.name).unwrap()
            }
        };

        let existing = api::rapid_entry::query_for_repo(&conn, repo_db.id);
        let mut new_map = HashMap::new();
        for sdp in &sdps {
            new_map.insert(&sdp.md5, true);
        }
        let mut old_map = HashMap::new();
        for old in &existing {
            old_map.insert(&old.hash, true);
        }

        for sdp in &sdps {
            if !old_map.contains_key(&sdp.md5) {
                api::rapid_entry::create(
                    &conn,
                    &sdp.fullname,
                    &sdp.md5,
                    &sdp.something,
                    &sdp.alias,
                    repo_db.id,
                );
                added += 1;
            }
        }
        for old in &existing {
            if !new_map.contains_key(&old.hash) {
                api::rapid_entry::delete(&conn, old.id);
                deleted += 1;
            }
        }
    }

    if added == 0 && deleted == 0 {
        println!("No changes");
    } else {
        println!("Added: {}. Deleted: {}", added, deleted);
    }

    Ok(())
}
