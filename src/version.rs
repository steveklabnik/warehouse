use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Version {
    pub id: String,
    pub crate_id: String,
    pub name: String,
    pub checksum: String,
    pub yanked: bool,
    pub dependencies: BTreeMap<String, Dependency>,
}

#[derive(Debug)]
pub struct Dependency {
    pub id: String,
    pub name: String,
    pub requirement: String,
    pub optional: bool,
}

impl Version {
    pub fn from_value(v: Value) -> Version {
        let obj = v.as_object().unwrap();

        let id = obj.get("vers").unwrap().as_string().unwrap().to_string();
        let crate_id = obj.get("name").unwrap().as_string().unwrap().to_string();
        let name = id.clone();
        let checksum = obj.get("cksum").unwrap().as_string().unwrap().to_string();
        let yanked = obj.get("yanked").unwrap().as_boolean().unwrap();

        let raw_deps = obj.get("deps").unwrap().as_array().unwrap();

        let mut dependencies = BTreeMap::new();

        for dep in raw_deps.iter() {
            let dep = dep.as_object().unwrap();

            let name = dep.get("name").unwrap().as_string().unwrap().to_string();
            let id = format!("{}-{}-{}", crate_id, id, name);
            let requirement = dep.get("req").unwrap().as_string().unwrap().to_string();
            let optional = dep.get("optional").unwrap().as_boolean().unwrap();

            dependencies.insert(id.clone(), Dependency {
                id: id,
                name: name,
                requirement: requirement,
                optional: optional,
            });
        }

        Version {
            id: id,
            crate_id: crate_id,
            name: name,
            checksum: checksum,
            yanked: yanked,
            dependencies: dependencies,
        }
    }
}
