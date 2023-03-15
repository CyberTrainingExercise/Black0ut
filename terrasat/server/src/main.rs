#[macro_use] extern crate rocket;

use crate::config::{Config};
use model::satellite::{Satellite, SatelliteStatus};

#[cfg(test)] mod tests;

mod server;
mod config;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
fn stats() -> &'static str {
    "System Status - Ok"
}

#[launch]
fn rocket() -> _ {
    let config = Config::new();
    rocket::build()
        .mount("/", routes![stats])
        .attach(server::stage())
}