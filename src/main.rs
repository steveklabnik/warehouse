extern crate iron;

#[macro_use]
extern crate router;

use router::Router;

use iron::prelude::*;
use iron::status;

fn main() {
    let router = router!(get  "/" => index);

    Iron::new(router).http("localhost:3000").unwrap();
}

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!")))
}
