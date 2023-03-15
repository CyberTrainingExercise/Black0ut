use std::sync::Arc;
use rocket::State;
use rocket::response::content::RawJson;
use rocket::fairing::AdHoc;

use crate::config::Config;

// Try visiting:
//   http://127.0.0.1:8000/count
#[get("/")]
fn count(config: &State<Arc<Config>>) -> RawJson<String> {
    let count = &config.satellites.len();
    let res = serde_json::to_string(count);
    match res {
        Ok(str) => {
            RawJson(str)
        }
        Err(err) => RawJson(format!("{}", err)),
    }
}

// Try visiting:
//   http://127.0.0.1:8000/all
#[get("/")]
fn status_all(config: &State<Arc<Config>>) -> RawJson<String> {
    let res = serde_json::to_string(&config.satellites);
    match res {
        Ok(str) => {
            RawJson(str)
        }
        Err(err) => RawJson(format!("{}", err)),
    }
}

// Try visiting:
//   http://127.0.0.1:8000/status/1
#[get("/<sat>")]
fn status(config: &State<Arc<Config>>, sat: usize) -> RawJson<String> {
    let res = &config.get_sat(sat);
    let res = match res {
        Ok(val) => serde_json::to_string(val),
        Err(err) => return RawJson(format!("{}", err)),
    };
    match res {
        Ok(str) => {
            RawJson(str)
        }
        Err(err) => RawJson(format!("{}", err)),
    }
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
            .mount("/count", routes![count])
            .mount("/all", routes![status_all])
    })
}