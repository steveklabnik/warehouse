use serde_json::Value;

#[derive(Debug)]
pub struct Version {
    pub id: String,
    pub crate_id: String,
    pub checksum: String,
    pub yanked: bool,
}

impl Version {
    pub fn from_value(v: Value) -> Version {
        let obj = v.as_object().unwrap();

        let id = obj.get("vers").unwrap().as_string().unwrap().to_string();
        let crate_id = obj.get("name").unwrap().as_string().unwrap().to_string();
        let checksum = obj.get("cksum").unwrap().as_string().unwrap().to_string();
        let yanked = obj.get("yanked").unwrap().as_boolean().unwrap();

        Version {
            id: id,
            crate_id: crate_id,
            checksum: checksum,
            yanked: yanked,
        }
    }
}

