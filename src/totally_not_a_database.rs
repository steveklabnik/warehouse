use std::collections::BTreeMap;
use krate::Crate;
use version::Version;
use serde_json;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use iron::typemap::Key;

pub struct TotallyNotADatabase(pub BTreeMap<String, Crate>);

impl Key for TotallyNotADatabase { type Value = TotallyNotADatabase; }

fn bad_entry(entry: &fs::DirEntry) -> bool {
    let bad_paths = ["crates.io-index/config.json", "crates.io-index/.git"];
    for path in bad_paths.iter() {
        if &entry.path().to_str().unwrap() == path {
            return true;
        }
    }

    false
}

impl TotallyNotADatabase {

    pub fn new() -> TotallyNotADatabase {
        let mut db = BTreeMap::new();

        for entry in fs::walk_dir("crates.io-index").unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_file() {
                if bad_entry(&entry) { continue; }

                let f = File::open(entry.path()).unwrap();
                let reader = BufReader::new(f);

                for line in reader.lines() {
                    let line = line.unwrap();
                    let data: Value = serde_json::from_str(&line).unwrap();

                    let name = data.as_object().unwrap().get("name").unwrap().as_string().unwrap().to_string();
                    let version = Version::from_value(data);

                    let krate = db.entry(name.clone()).or_insert(Crate { id: name.clone(), versions: BTreeMap::new() });
                    info!("Adding version {} to crate {}", version.id, name);
                    krate.add_version(version);
                }
            }
        }

        TotallyNotADatabase(db)
    }
}

