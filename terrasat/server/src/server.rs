use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::State;
use rocket::response::content::RawHtml;
use rocket::fairing::AdHoc;

struct HitCount(AtomicUsize);

// #[get("/")]
// fn index(hit_count: &State<HitCount>) -> RawHtml<String> {
//     let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;
//     RawHtml(format!("Your visit is recorded!<br /><br />Visits: {}", count))
// }

// Try visiting:
//   http://127.0.0.1:8000/status/5
#[get("/<sat>")]
fn status(hit_count: &State<HitCount>, sat: usize) -> String {
    let count = hit_count.0.fetch_add(1, Ordering::Relaxed) + 1;
    format!("Sat{} Status - {}", sat, count)
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Get status", |rocket| async {
        rocket.mount("/status", routes![status])
            .manage(HitCount(AtomicUsize::new(0)))
    })
}