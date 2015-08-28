use std::collections::BTreeMap;
use version::Version;

#[derive(Debug)]
pub struct Crate {
    pub id: String,
    pub versions: BTreeMap<String, Version>,
}

impl Crate {
    pub fn add_version(&mut self, v: Version) {
        let name = v.id.clone();
        self.versions.insert(name, v);
    }
}

