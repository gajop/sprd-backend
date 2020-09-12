use std::ffi::OsStr;
use std::io::prelude::*;
use std::fs;

use walkdir::WalkDir;

use std::path;


use flate2::read::GzDecoder;


fn read_binary_file(path: &path::Path) -> std::io::Result<Vec<u8>> {
    let mut versions_file = fs::File::open(path)?;
    let mut contents = Vec::new();
    versions_file.read_to_end(&mut contents)?;

    Ok(contents)
}

fn read_gz_from_data(data: Vec<u8>) -> std::io::Result<String> {
    let mut d = GzDecoder::new(&data[..]);
    let mut s = String::new();
    d.read_to_string(&mut s)?;

    Ok(s)
}

fn read_gz_from_file(path: &path::Path) -> std::io::Result<String> {
    let data = read_binary_file(path)?;
    let unzipped = read_gz_from_data(data)?;

    Ok(unzipped)
}

pub struct RapidEntry {
    fullname: String,
    something: String, // what's the purpose of this field?
    hash: String,
    alias: String
}

fn read_rapid_from_file(path: &path::Path) -> std::io::Result<Vec<RapidEntry>> {
    let parsed_gz = read_gz_from_file(path)?;

    let mut entries = Vec::new();

    for line in parsed_gz.lines() {
        let line_entry: Vec<&str> = line.split(',').collect();
        if line_entry.len() != 4 {
            continue;
        }
        entries.push(RapidEntry{
            fullname: line_entry[0].to_string(),
            hash: line_entry[2].to_string(),
            something: line_entry[3].to_string(),
            alias: line_entry[2].to_string()
        });
    }

    Ok(entries)
}

pub fn parse_rapid(directory_path: &path::Path) -> std::io::Result<()> {
    for entry in WalkDir::new(directory_path) {
        let e = entry?;
        let path = e.path();
        if path.extension() == Some(OsStr::new("gz")) {
            // println!("{:?}", path);
            // let read_result = read_gz_from_file(path);
            // if let Ok(s) = read_result {
            //     println!("{}", s);
            // }
            println!("{:?}", path);
            let rapid_entries = read_rapid_from_file(path);
            if let Ok(rapid_entries) = rapid_entries {
                for entry in rapid_entries {
                    if entry.something != "" {
                        println!("{} {} {} {}", entry.fullname, entry.hash, entry.something, entry.alias);
                    }
                }
            }
        }
    }

    // println!("{}", s);

    Ok(())
}
