#![feature(fs_walk)]

extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate hyper;

use router::Router;
use iron::prelude::*;
use iron::status;
use hyper::header::AccessControlAllowOrigin;

mod totally_not_a_database;
mod krate;
mod version;

use totally_not_a_database::TotallyNotADatabase;

lazy_static! {
    static ref STORE: TotallyNotADatabase = { TotallyNotADatabase::new() };
}

fn main() {
    let router = router!(get "/" => index,
                         get "/crates" => crates);

    let mut chain = Chain::new(router);
    chain.link_after(|_: &mut Request, mut res: Response| {
        // lol
        res.headers.set(AccessControlAllowOrigin::Any);

        Ok(res)
    });

    Iron::new(chain).http("localhost:3000").unwrap();
}

fn index(_: &mut Request) -> IronResult<Response> {
    let index = String::from("{\"crates\": \"/crates\"}");

    Ok(Response::with((status::Ok, index)))
}

fn crates(_: &mut Request) -> IronResult<Response> {
    let data = &STORE.0;

    let mut json = String::from("{\"data\":[");

    let mut versions = Vec::new();

    let crates = data.iter().map(|(_, krate)| {
        for (_, version) in &krate.versions {
            versions.push(version.clone());
        }

        let krate_versions = krate.versions.iter().map(|(_, v)| {
            let mut id = String::from("");
            id.push_str(&krate.id);
            id.push_str(&'-'.to_string());
            id.push_str(&v.id);

            format!("{{\"type\": \"version\",\"id\": \"{}\"}}", id)
        }).collect::<Vec<_>>().join(",");

        format!("{{\"id\": \"{}\", \"type\":\"crate\",\"relationships\": {{\"versions\": {{\"data\": [{}]}}}}}}", krate.id, krate_versions)
    }).collect::<Vec<String>>().join(",");

    json.push_str(&crates);
    json.push_str("],\"included\":[");

    let included = versions.iter().map(|v| {
        let mut id = String::from("");
        id.push_str(&v.crate_id);
        id.push_str(&'-'.to_string());
        id.push_str(&v.id);

        format!("{{\"type\": \"version\",\"id\": \"{}\", \"crate-id\": \"{}\", \"attributes\": {{\"name\": \"{}\"}}}}", id, v.crate_id, v.id)
    }).collect::<Vec<_>>().join(",");

    json.push_str(&included);
    json.push_str("]}");

    Ok(Response::with((status::Ok, json)))
}
