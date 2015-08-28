extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use router::Router;
use iron::prelude::*;
use iron::status;

mod totally_not_a_database;
mod krate;
mod version;

use totally_not_a_database::TotallyNotADatabase;

lazy_static! {
    static ref STORE: TotallyNotADatabase = {
        TotallyNotADatabase::from_path("crates.io-index/ir/on/iron")
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

    let mut json = String::from("{\"data\":[");

    let crates = data.iter().map(|(_, krate)| {
        let versions = krate.versions.iter().map(|(_, v)| {
            format!("{{\"type\": \"version\",\"id\": \"{}\"}}", v.id)
        }).collect::<Vec<_>>().join(",");

        format!("{{\"id\": \"{}\", \"type\":\"crate\",\"relationships\": {{\"versions\": {{\"data\": [{}]}}}}}}", krate.id, versions)
    }).collect::<Vec<String>>().join(",");

    json.push_str(&crates);
    json.push_str("]}");

    Ok(Response::with((status::Ok, json)))
}
