extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use router::Router;
use iron::prelude::*;
use iron::status;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
{
  "name": "iron",
  "vers": "0.0.2",
  "deps": [
    {
      "name": "error",
      "req": "*",
      "features": [
        ""
      ],
      "optional": false,
      "default_features": true,
      "target": null,
      "kind": "normal"
    },
  ],
  "cksum": "af0250cc6225932a7a3ce711481c0ec15cc1fcc474e28bc997ba54dcdb087da0",
  "features": {
    
  },
  "yanked": false
}
*/

struct TotallyNotADatabase(Version);

#[derive(Debug)]
struct Version {
    id: String,
    version: String,
    checksum: String,
    yanked: bool,
}

impl Version {
    fn from_value(v: Value) -> Version {
        let obj = v.as_object().unwrap();

        let id = obj.get("name").unwrap().as_string().unwrap().to_string();
        let version = obj.get("vers").unwrap().as_string().unwrap().to_string();
        let checksum = obj.get("cksum").unwrap().as_string().unwrap().to_string();
        let yanked = obj.get("yanked").unwrap().as_boolean().unwrap();

        Version {
            id: id,
            version: version,
            checksum: checksum,
            yanked: yanked,
        }
    }
}

lazy_static! {
    static ref STORE: TotallyNotADatabase = {
        let f = File::open("crates.io-index/ir/on/iron").unwrap();
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        let data: Value = serde_json::from_str(&line).unwrap();
        
        TotallyNotADatabase(Version::from_value(data))
    };
}

fn main() {
    let router = router!(get "/" => index,
                         get "/crates" => crates);

    Iron::new(router).http("localhost:3000").unwrap();
}

fn index(_: &mut Request) -> IronResult<Response> {
    let index = String::from("{\"crates\": \"/crates\"}");

    Ok(Response::with((status::Ok, index)))
}

fn crates(_: &mut Request) -> IronResult<Response> {
    let data = &STORE.0;

    Ok(Response::with((status::Ok, format!("{:?}", data))))
}
