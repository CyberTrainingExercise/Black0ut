use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use rocket::State;
use rocket::response::content::{RawHtml, RawJson};
use rocket::fairing::AdHoc;

use crate::config::Config;
use model::satellite::{Satellite, SatelliteStatus};

// #[get("/")]
// fn index(hit_count: &State<HitCount>) -> RawHtml<String> {
//     let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;
//     RawHtml(format!("Your visit is recorded!<br /><br />Visits: {}", count))
// }


// Try visiting:
//   http://127.0.0.1:8000/status/1
#[get("/<sat>")]
fn status(config: &State<Arc<Config>>, sat: usize) -> RawJson<String> {
    RawJson(config.get_sat(sat))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Get Status", |rocket| async {
        let config = Config::new();

        if config.is_err() {
            print!("Err: {:#?}", config);
        }

        let config = Arc::new(config.unwrap());

        rocket.mount("/status", routes![status])
            .manage(config)
    })
}