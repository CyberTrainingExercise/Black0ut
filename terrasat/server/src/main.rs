use rocket::response::content::RawHtml;

#[macro_use] extern crate rocket;

mod server;
mod config;

// Try visiting:
//   http://127.0.0.1:8000/
#[get("/")]
fn stats() -> RawHtml<String> {
    RawHtml("<br><h1>Terrasat API</h1><p>Please use the Terrasat CLI Client to connect and use data.</p>".to_owned())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![stats])
        .attach(server::stage())
}