use std::collections::BTreeMap;
use krate::Crate;
use version::Version;
use serde_json;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct TotallyNotADatabase(pub BTreeMap<String, Crate>);

impl TotallyNotADatabase {
    pub fn from_path(p: &str) -> TotallyNotADatabase {
        let f = File::open(p).unwrap();
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        let data: Value = serde_json::from_str(&line).unwrap();

        let mut iron = Crate { id: String::from("iron"), versions: BTreeMap::new() };

        let version = Version::from_value(data);

        iron.add_version(version);

        let mut db = BTreeMap::new();

        db.insert(String::from("iron"), iron);
        
        TotallyNotADatabase(db)
    }
}

