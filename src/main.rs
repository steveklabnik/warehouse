extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;

use router::Router;
use iron::prelude::*;
use iron::status;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let router = router!(get  "/" => index);

    Iron::new(router).http("localhost:3000").unwrap();
}

fn index(_: &mut Request) -> IronResult<Response> {
    let f = File::open("crates.io-index/ir/on/iron").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let data: Value = serde_json::from_str(&line).unwrap();
    let obj = data.as_object().unwrap();

    Ok(Response::with((status::Ok, format!("{:?}", obj))))
}
