extern crate iron;

use std::env;
use iron::prelude::*;
use iron::status;

fn main() {
    let port = env::var("PORT").unwrap_or("8080".to_string());

    Iron::new(|_: &mut Request| Ok(Response::with((status::Ok, "Hello world!"))))
        .http(format!("localhost:{}", port));
}
