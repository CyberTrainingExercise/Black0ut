use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};
use rocket::State;
use rocket::response::content::RawJson;
use rocket::fairing::AdHoc;

use model::satellite::SatelliteStatus;
use crate::config::Config;

fn guard(config: &State<Arc<Mutex<Config>>>) -> bool {
    if config.lock().unwrap().dos_active {
        return true;
    }
    false
}
fn data_guard(config: &State<Arc<Mutex<Config>>>, sat: usize) -> bool {
    if config.lock().unwrap().satellites.len() > sat {
        return false;
    }
    guard(config)
}

// Try visiting:
//   http://127.0.0.1:8000/dummy_data
#[get("/")]
fn dummy_data(config: &State<Arc<Mutex<Config>>>) -> RawJson<String> {
   RawJson("{\n  \"status0\": \"offline\",\n  \"status1\": \"ok\",\n  \"status2\": \"offline\",\n  \"status15\": \"offline\",\n  \"status12\": \"ok\"\n}".to_owned())
}

// Try visiting:
//   http://127.0.0.1:8000/ui
#[get("/")]
fn ui_data(config: &State<Arc<Mutex<Config>>>) -> RawJson<String> {
    let mut res = "{\n".to_owned();
    let mut i = 0;
    let len = &config.lock().unwrap().satellites.len();
    let now = SystemTime::now();
    let now_duration = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    i = 0;
    let mut set_inactive = Vec::new();
    let mut set_active = Vec::new();
    for pulse in &config.lock().unwrap().pulse {
        let pulse_duration = pulse
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        if (now_duration.as_secs() - pulse_duration.as_secs()) > 5 {
            set_inactive.push(i);
        } else {
            set_active.push(i);
        }
        i += 1;
    }
    for sat in set_inactive {
        if !config.lock().unwrap().satellites[sat].has_pulse {
            continue;
        }
        config.lock().unwrap().satellites[sat].status = SatelliteStatus::INACTIVE;
        println!("sat {} has pulse down", sat);
    }
    for sat in set_active {
        if !config.lock().unwrap().satellites[sat].has_pulse {
            continue;
        }
        config.lock().unwrap().satellites[sat].status = SatelliteStatus::ACTIVE;
        println!("sat {} has pulse up", sat);
    }
    i = 0;
    for sat in &config.lock().unwrap().satellites {
        match sat.status {
            SatelliteStatus::ACTIVE => res += &format!("  \"status{}\": \"ok\"", i),
            SatelliteStatus::SLEEP => res += &format!("  \"status{}\": \"sleep\"", i),
            SatelliteStatus::INACTIVE => res += &format!("  \"status{}\": \"offline\"", i),
        }
        i += 1;
        // no comma for last line
        if i != *len {
            res += ",\n";
        }
    }
    res += "\n}";
    RawJson(res)
}

#[get("/<key>")]
fn set_dos(config: &State<Arc<Mutex<Config>>>, key: usize) -> RawJson<String> {
    if key != 1521 {
        return RawJson(format!("Incorrect Key"));
    }
    let current = config.lock().unwrap().dos_active;
    config.lock().unwrap().dos_active = !current;
    let result = if current
    { "DOS inactive, all other commands will work" }
    else { "DOS active, no other commands will work" };
    return RawJson(format!("{result}"));
}   

// Try visiting:
//   http://127.0.0.1:8000/get_pulses
#[get("/")]
fn get_pulses(config: &State<Arc<Mutex<Config>>>) -> RawJson<String> {
    if guard(config) {
        return RawJson(format!("Failed: guard active"));
    }
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
    if guard(config) {
        return RawJson(format!("Failed: guard active"));
    }
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
//   http://127.0.0.1:8000/disable/0/open
#[get("/<sat>/<code>")]
fn disable(config: &State<Arc<Mutex<Config>>>, sat: usize, code: String) -> RawJson<String> {
    if data_guard(config, sat) {
        return RawJson(format!("Failed: index out of bounds"));
    }
    let password = config.lock().unwrap().satellites[sat].password.clone();
    if password == code {
        config.lock().unwrap().satellites[sat].status = SatelliteStatus::INACTIVE;
        return RawJson(format!("Ok: sat {} set to inactive", sat));
    }
    RawJson(format!("Failed: incorrect password"))
}

// Try visiting:
//   http://127.0.0.1:8000/shutdown/2/1882
#[get("/<sat>/<code>")]
fn shutdown(config: &State<Arc<Mutex<Config>>>, sat: usize, code: usize) -> RawJson<String> {
    if data_guard(config, sat) {
        return RawJson(format!("Failed: index out of bounds"));
    }
    let config_code = config.lock().unwrap().satellites[sat].shutdown_code;
    match config_code {
        Some(val) => {
            if val == code {
                config.lock().unwrap().satellites[sat].status = SatelliteStatus::INACTIVE;
                return RawJson(format!("Success"));
            } else {
                // purposely send them a different code for sats with shutdown
                return RawJson(format!("Failed: Invalid Code"));
            }
        }
        None => {
            return RawJson(format!("Failed: Unknown Operation"));
        }
    }
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
    if config.lock().unwrap().satellites[sat].status != SatelliteStatus::ACTIVE {
        return RawJson(format!("Failed: cannot sleep {} because it is not active", sat));
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
    if guard(config) {
        return RawJson(format!("Failed: guard active"));
    }
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
    if guard(config) {
        return RawJson(format!("Failed: guard active"));
    }
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
            .mount("/shutdown", routes![shutdown])
            .mount("/dos", routes![set_dos])
            .mount("/ui", routes![ui_data])
            .mount("/disable", routes![disable])
    })
}