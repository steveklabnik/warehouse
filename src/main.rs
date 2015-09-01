#![feature(fs_walk)]

extern crate iron;
#[macro_use]
extern crate router;
extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate persistent;

use iron::prelude::*;
use iron::status;
use iron::headers::AccessControlAllowOrigin;
use router::Router;
use persistent::Read;

mod totally_not_a_database;
mod krate;
mod version;

use totally_not_a_database::TotallyNotADatabase;

fn main() {
    env_logger::init().unwrap();
    info!("Starting server");

    let router = router!(get "/" => index,
                         get "/crates" => crates,
                         get "/crates/:id" => crates);

    info!("Initializing data...");
    let data = TotallyNotADatabase::new();
    info!("Data loaded");

    let mut chain = Chain::new(router);

    chain.link(Read::<TotallyNotADatabase>::both(data));

    chain.link_before(|req: &mut Request| {
        // Basic logging of requests.
        info!("REQUEST: {}", req.url.path.join("/"));
        Ok(())
    });

    chain.link_after(|_: &mut Request, mut res: Response| {
        // lol
        res.headers.set(AccessControlAllowOrigin::Any);

        Ok(res)
    });

    info!("Server starting on http://localhost:3000");
    Iron::new(chain).http("localhost:3000").unwrap();
}

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, r#"{"crates": "crates"}"#)))
}

fn crates(req: &mut Request) -> IronResult<Response> {
    let ref data = req.get::<Read<TotallyNotADatabase>>().unwrap().0;
    let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap_or("");

    let mut json = String::from("{\"data\":");

    let mut versions = Vec::new();

    let crates;

    if id.is_empty() {
        crates = data.iter().map(|(_, krate)| {
            let krate_versions = krate.versions.iter().map(|(_, v)| {
                versions.push(v.clone());

                let id = format!("{}-{}", krate.id, v.id);

                let deps = v.dependencies.iter().map(|(_, d)| {
                    format!("{{\"type\": \"dependency\", \"id\": \"{}\"}}", d.id)
                }).collect::<Vec<_>>().join(",");

                format!("{{\"type\": \"version\",\"id\": \"{}\",\"relationships\": {{\"dependencies\": {{\"data\": [{}]}}}}}}", id, deps)
                }).collect::<Vec<_>>().join(",");

                format!("{{\"id\": \"{}\", \"type\":\"crate\",\"relationships\": {{\"versions\": {{\"data\": [{}]}}}}}}", krate.id, krate_versions)
        }).collect::<Vec<String>>().join(",");
    } else {
        let krate = data.get(id.clone()).unwrap();

        let krate_versions = krate.versions.iter().map(|(_, v)| {
            versions.push(v.clone());

            let id = format!("{}-{}", krate.id, v.id);

            let deps = v.dependencies.iter().map(|(_, d)| {
                format!("{{\"type\": \"dependency\", \"id\": \"{}\"}}", d.id)
            }).collect::<Vec<_>>().join(",");

            format!("{{\"type\": \"version\",\"id\": \"{}\",\"relationships\": {{\"dependencies\": {{\"data\": [{}]}}}}}}", id, deps)
        }).collect::<Vec<_>>().join(",");

        crates = format!("{{\"id\": \"{}\", \"type\":\"crate\",\"relationships\": {{\"versions\": {{\"data\": [{}]}}}}}}", krate.id, krate_versions);
    }

    if id.is_empty() {
        json.push_str(&format!("[{}]", crates));
    } else {
        json.push_str(&crates);
    }

    json.push_str(",\"included\":[");

    let included_versions = versions.iter().map(|v| {
        let id = format!("{}-{}", v.crate_id, v.id);

        format!("{{\"type\": \"version\",\"id\": \"{}\", \"attributes\": {{\"crate-id\": \"{}\",\"name\": \"{}\"}}}}", id, v.crate_id, v.id)
    }).collect::<Vec<_>>().join(",");

    json.push_str(&included_versions);

    json.push_str(",");

    let mut included_dependencies = vec![];
    for &version in versions.iter() {
        for (_, d) in version.dependencies.iter() {
            included_dependencies.push(format!("{{\"type\": \"dependency\",\"id\": \"{}\",\"attributes\": {{ \"version-id\": \"{}\", \"name\": \"{}\",\"requirement\":\"{}\",\"optional\":{}}}}}", d.id, format!("{}-{}", version.crate_id, version.id), d.name, d.requirement, d.optional))
        }
    }

    json.push_str(&(included_dependencies.join(",")));

    json.push_str("]}");

    Ok(Response::with((status::Ok, json)))
}
