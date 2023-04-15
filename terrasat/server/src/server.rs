use std::{sync::{Arc, Mutex}, time::SystemTime};
use rocket::State;
use rocket::response::content::RawJson;
use rocket::fairing::AdHoc;

use model::satellite::SatelliteStatus;
use crate::config::Config;

fn data_guard(config: &State<Arc<Mutex<Config>>>, sat: usize) -> bool {
    if config.lock().unwrap().satellites.len() > sat {
        return false;
    }
    true
}

// Try visiting:
//   http://127.0.0.1:8000/dummy_data
#[get("/")]
fn dummy_data(config: &State<Arc<Mutex<Config>>>) -> RawJson<String> {
   RawJson("{\n  \"status0\": \"offline\",\n  \"status1\": \"ok\",\n  \"status2\": \"offline\"\n}".to_owned())
}

// Try visiting:
//   http://127.0.0.1:8000/get_pulses
#[get("/")]
fn get_pulses(config: &State<Arc<Mutex<Config>>>) -> RawJson<String> {
    let res = serde_json::to_string(&config.lock().unwrap().pulse);
    match res {
        Ok(str) => {
            RawJson(str)
        }
        Err(err) => RawJson(format!("{}", err)),
    }
}


// Try visiting:
//   http://127.0.0.1:8000/get_pulse/0
#[get("/<sat>")]
fn get_pulse(config: &State<Arc<Mutex<Config>>>, sat: usize) -> RawJson<String> {
    if data_guard(config, sat) {
        return RawJson(format!("Failed: index out of bounds"));
    }
    let res = serde_json::to_string(&config.lock().unwrap().pulse[sat]);
    return RawJson(format!("{:#?}", res));
}

#[put("/<sat>")]
fn pulse(config: &State<Arc<Mutex<Config>>>, sat: usize) -> RawJson<String> {
    if data_guard(config, sat) {
        return RawJson(format!("Failed: index out of bounds"));
    }
    config.lock().unwrap().pulse[sat] = SystemTime::now();
    return RawJson(format!("Ok"));
}

// Try visiting:
//   http://127.0.0.1:8000/sleep/0
#[get("/<sat>")]
fn wake(config: &State<Arc<Mutex<Config>>>, sat: usize) -> RawJson<String> {
    if data_guard(config, sat) {
        return RawJson(format!("Failed: index out of bounds"));
    }
    if config.lock().unwrap().satellites[sat].status == SatelliteStatus::SLEEP {
        config.lock().unwrap().satellites[sat].status = SatelliteStatus::ACTIVE;
        return RawJson(format!("Success"));
    }
    return RawJson(format!("Failed: cannot wake sat{} as it is not sleeping", sat));
}

// Try visiting:
//   http://127.0.0.1:8000/sleep/0
#[get("/<sat>")]
fn sleep(config: &State<Arc<Mutex<Config>>>, sat: usize) -> RawJson<String> {
    if data_guard(config, sat) {
        return RawJson(format!("Failed: index out of bounds"));
    }
    let mut sleeping = 0;
    for sat in &config.lock().unwrap().satellites {
        if sat.status == SatelliteStatus::SLEEP {
            sleeping += 1;
        }
    }
    // TODO: put this in the config
    let max = 3;
    if sleeping >= max {
        return RawJson(format!("Failed: max {} sats sleeping at once", max));
    }
    config.lock().unwrap().satellites[sat].status = SatelliteStatus::SLEEP;
    return RawJson(format!("Success"));
}

// Try visiting:
//   http://127.0.0.1:8000/login/0/openup
#[get("/<sat>/<password>")]
fn login(config: &State<Arc<Mutex<Config>>>, sat: usize, password: String) -> RawJson<String> {
    if data_guard(config, sat) {
        return RawJson(format!("Failed: index out of bounds"));
    }
    if config.lock().unwrap().satellites[sat].password == password {
        return RawJson(format!("True"));
    }
    if config.lock().unwrap().satellites[sat].version == "v1.1.2" {
        // Intentional bug, where the password is returned in plaintext for v1.1.2 satellites
        return RawJson(format!("False, password is: {}", config.lock().unwrap().satellites[sat].password));
    }
    return RawJson(format!("False"));
}

// Try visiting:
//   http://127.0.0.1:8000/count
#[get("/")]
fn count(config: &State<Arc<Mutex<Config>>>) -> RawJson<String> {
    let count = &config.lock().unwrap().satellites.len();
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
fn status_all(config: &State<Arc<Mutex<Config>>>) -> RawJson<String> {
    let res = serde_json::to_string(&config.lock().unwrap().satellites);
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
fn status(config: &State<Arc<Mutex<Config>>>, sat: usize) -> RawJson<String> {
    if data_guard(config, sat) {
        return RawJson(format!("Failed: index out of bounds"));
    }
    let binding = config.lock().unwrap();
    let res = binding.get_sat(sat);
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

        let config = Arc::new(Mutex::new(config.unwrap()));

        rocket.mount("/status", routes![status])
            .manage(config)
            .mount("/count", routes![count])
            .mount("/all", routes![status_all])
            .mount("/login", routes![login])
            .mount("/sleep", routes![sleep])
            .mount("/wake", routes![wake])
            .mount("/get_pulse", routes![get_pulse])
            .mount("/pulse", routes![pulse])
            .mount("/get_pulses", routes![get_pulses])
            .mount("/dummy_data", routes![dummy_data])
    })
}