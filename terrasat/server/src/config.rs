use std::fmt::{Formatter, self, Display};
use std::{fs, vec};
use std::io::Error as IoError;
use rocket::form::validate::Len;
use serde::{Serialize, Deserialize};
use toml;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};


use model::satellite::{Satellite, SatelliteStatus};

#[derive(Debug, Clone)]
pub struct ConfigParseError(String);
impl Display for ConfigParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq for ConfigParseError {
    fn eq(&self, _othr: &ConfigParseError) -> bool {
        true
    }
}
impl Eq for ConfigParseError {}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    satellites: Option<HashMap<String, SatelliteToml>>,
	cli: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SatelliteToml {
	os: String,
	version: String,
    debug_mode: bool,
    password: String,
    shutdown_code: usize,
	has_pulse: bool,
}

impl SatelliteToml {
	pub fn to_sat(&self, index: &str) -> Satellite {
		let shutdown_code = if self.shutdown_code == 0 
			{ None } else { Some(self.shutdown_code ) };
		Satellite {
			name: "Sat".to_owned() + index,
			version: self.version.clone(),
			status: SatelliteStatus::ACTIVE,
			os: self.os.clone(),
			debug_mode: self.debug_mode,
			password: self.password.clone(),
			shutdown_code: shutdown_code,
			has_pulse: self.has_pulse,
		}
	}
}

#[derive(Debug)]
pub struct Config {
    pub satellites: Vec<Satellite>,
	pub pulse: Vec<SystemTime>,
	pub dos_active: bool,
}

impl Config {
    pub fn new() -> Result<Self, ConfigParseError> {
		let config_filepaths: [&str; 4] = [
			"./config.toml",
			"./Config.toml",
			"./src/config.toml",
			"./src/Config.toml",
		];

		let mut content: String = "".to_owned();

		for filepath in config_filepaths {
			let result: Result<String, IoError> = fs::read_to_string(filepath);

			if result.is_ok() {
				content = result.unwrap();
				break;
			}
		}

		let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
			("Failed to create ConfigToml Object out of config file.");
			ConfigToml {
				satellites: None,
				cli: None,
			}
		});

		Ok(Config
		{
			satellites: match config_toml.satellites.as_ref() {
				Some(satellites) => {
					let mut vec: Vec<Satellite> = Vec::with_capacity(satellites.len());
					for _ in 0..satellites.len() {
						vec.push(Satellite::empty());
					}
					for (name, sat) in satellites {
						let index = name[3..].parse::<usize>();
						if index.is_err() {
							return Err(ConfigParseError(String::from("Incorrect satellite input for ".to_owned() + &name
							+ ". They need to named sat0...satn without skipping any #."
							)))
						}
						let index = index.unwrap();
						if index > vec.len() {
							return Err(ConfigParseError(String::from("Satellite index out of bounds for ".to_owned()
							+ &name + " at index " + &index.to_string()
							+ ". They need to named sat0...satn without skipping any #."
							)))
						}
						vec[index] = sat.to_sat(&index.to_string());
					}
					vec
				}
				None => Vec::new(),
			},
			pulse: vec!(SystemTime::now(); config_toml.satellites.as_ref().len()),
			dos_active: false,
		})
	}
	pub fn get_sat(&self, sat: usize) -> Result<&Satellite, ConfigParseError> {
		if sat >= self.satellites.len() {
			return Err(ConfigParseError("Error: 'Index out of bounds'".to_owned()));
		}
		Ok(&self.satellites[sat])
	}
}
