use serde_json::Value;

#[derive(Debug,Serialize)]
pub struct Version {
    pub id: String,
    #[serde(rename="type")]
    pub typelol: String,
    #[serde(rename="crate-id")]
    pub crate_id: String,
    pub name: String,
    pub checksum: String,
    pub yanked: bool,
}

impl Version {
    pub fn from_value(v: Value) -> Version {
        let obj = v.as_object().unwrap();

        let id = obj.get("vers").unwrap().as_string().unwrap().to_string();
        let crate_id = obj.get("name").unwrap().as_string().unwrap().to_string();
        let name = id.clone();
        let checksum = obj.get("cksum").unwrap().as_string().unwrap().to_string();
        let yanked = obj.get("yanked").unwrap().as_boolean().unwrap();

        Version {
            id: crate_id.clone() + "-" + &id,
            typelol: String::from("version"),
            crate_id: crate_id,
            name: name,
            checksum: checksum,
            yanked: yanked,
        }
    }
}

