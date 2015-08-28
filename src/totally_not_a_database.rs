use std::collections::BTreeMap;
use krate::Crate;
use version::Version;
use serde_json;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct TotallyNotADatabase(pub BTreeMap<String, Crate>);

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
                let mut reader = BufReader::new(f);

                let mut line = String::new();
                reader.read_line(&mut line).unwrap();

                let data: Value = serde_json::from_str(&line).unwrap();

                let name = data.as_object().unwrap().get("name").unwrap().as_string().unwrap().to_string();
                let version = Version::from_value(data);

                // TODO: look up the existing crate instead of just making a new one
                let mut krate = Crate { id: name.clone(), versions: BTreeMap::new() };

                krate.add_version(version);

                db.insert(name, krate);
            }
        }
        
        TotallyNotADatabase(db)
    }
}

