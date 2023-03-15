#[macro_use] extern crate rocket;

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
    rocket::build()
        .mount("/", routes![stats])
        .attach(server::stage())
}