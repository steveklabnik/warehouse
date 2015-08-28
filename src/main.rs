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
use std::collections::BTreeMap;

struct TotallyNotADatabase(BTreeMap<String, serde_json::value::Value>);

lazy_static! {
    static ref STORE: TotallyNotADatabase = {
        let f = File::open("crates.io-index/ir/on/iron").unwrap();
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        let data: Value = serde_json::from_str(&line).unwrap();
        let obj = data.as_object().unwrap();
        
        TotallyNotADatabase(obj.to_owned())
    };
}

fn main() {
    let router = router!(get "/" => index,
                         get "/crates" => crates);

    Iron::new(router).http("localhost:3000").unwrap();
}

fn index(_: &mut Request) -> IronResult<Response> {
    let index = "{\"crates\": \"/crates\"}";

    Ok(Response::with((status::Ok, String::from(index))))
}

fn crates(_: &mut Request) -> IronResult<Response> {
    let data = &STORE.0;

    Ok(Response::with((status::Ok, format!("{:?}", data))))
}
