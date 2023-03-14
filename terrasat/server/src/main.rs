#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
fn stats() -> &'static str {
    "System Status - Ok"
}

// Try visiting:
//   http://127.0.0.1:8000/status/5
#[get("/<sat>")]
fn status(sat: usize) -> String {
    format!("Sat{} Status - Ok", sat)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![stats])
        .mount("/status", routes![status])
}