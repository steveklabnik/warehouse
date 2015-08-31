#![feature(fs_walk)]

extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
extern crate hyper;

#[macro_use]
extern crate log;
extern crate env_logger;

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
    env_logger::init().unwrap();
    info!("Starting server");

    let router = router!(get "/" => index,
                         get "/crates" => crates,
                         get "/crates/:id" => crates);

    info!("Initializing data...");
    // let's not be lazy! (Load the data at startup, rather than on the first request
    &STORE.0;
    info!("Data loaded");

    let mut chain = Chain::new(router);
    chain.link_after(|_: &mut Request, mut res: Response| {
        // lol
        res.headers.set(AccessControlAllowOrigin::Any);

        Ok(res)
    });

    info!("Server starting on http://localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn index(_: &mut Request) -> IronResult<Response> {
    info!("REQUEST: /");
    let index = String::from("{\"crates\": \"/crates\"}");

    Ok(Response::with((status::Ok, index)))
}

fn crates(req: &mut Request) -> IronResult<Response> {
    let id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("");

    if id.is_empty() {
        info!("REQUEST: /crates");
    } else {
        info!("REQUEST: /crates/{}", id);
    }

    let data = &STORE.0;

    let mut json = String::from("{\"data\":");

    let mut versions = Vec::new();

    let crates;

    if id.is_empty() {
        crates = data.iter().map(|(_, krate)| {
            let krate_versions = krate.versions.iter().map(|(_, v)| {
                versions.push(v.clone());

                let id = format!("{}-{}", krate.id, v.id);

                format!("{{\"type\": \"version\",\"id\": \"{}\"}}", id)
            }).collect::<Vec<_>>().join(",");

            format!("{{\"id\": \"{}\", \"type\":\"crate\",\"relationships\": {{\"versions\": {{\"data\": [{}]}}}}}}", krate.id, krate_versions)
        }).collect::<Vec<String>>().join(",");
    } else {
        let krate = data.get(id).unwrap();

        let krate_versions = krate.versions.iter().map(|(_, v)| {
            versions.push(v.clone());

            let id = format!("{}-{}", krate.id, v.id);

            format!("{{\"type\": \"version\",\"id\": \"{}\"}}", id)
        }).collect::<Vec<_>>().join(",");

        crates = format!("{{\"id\": \"{}\", \"type\":\"crate\",\"relationships\": {{\"versions\": {{\"data\": [{}]}}}}}}", krate.id, krate_versions);
    }

    if id.is_empty() {
        json.push_str(&format!("[{}]", crates));
    } else {
        json.push_str(&crates);
    }

    json.push_str(",\"included\":[");

    let included = versions.iter().map(|v| {
        let id = format!("{}-{}", v.crate_id, v.id);

        format!("{{\"type\": \"version\",\"id\": \"{}\", \"crate-id\": \"{}\", \"attributes\": {{\"name\": \"{}\"}}}}", id, v.crate_id, v.id)
    }).collect::<Vec<_>>().join(",");

    json.push_str(&included);
    json.push_str("]}");

    Ok(Response::with((status::Ok, json)))
}