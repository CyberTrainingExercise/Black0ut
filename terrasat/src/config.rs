use std::fmt::{Formatter, self, Display};
use std::{fs};
use std::io::Error as IoError;
use serde::{ Serialize, Deserialize };
use toml;
use std::collections::HashMap;
use colored::{self, Colorize};

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

#[derive(Debug)]
pub enum SatelliteStatus {
	ACTIVE,
	INACTIVE,
	SLEEP,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SatelliteToml {
	os: String,
    debug_mode: bool,
    password: String,
    connection_limit: usize,
}

impl SatelliteToml {
	pub fn to_sat(&self, index: &str) -> Satellite {
		Satellite {
			name: "Sat".to_owned() + index,
			status: SatelliteStatus::ACTIVE,
			os: self.os.clone(),
			debug_mode: self.debug_mode,
			password: self.password.clone(),
			connection_limit: self.connection_limit
		}
	}
}

#[derive(Debug)]
pub struct Satellite {
	name: String,
	status: SatelliteStatus,
    os: String,
    debug_mode: bool,
    password: String,
    connection_limit: usize,
}

impl Satellite {
	pub fn empty() -> Self {
		Satellite {
			name: String::from("Unknown"),
			status: SatelliteStatus::INACTIVE,
			os: String::from("Unknown"),
			debug_mode: false,
			password: String::from("Unknown"),
			connection_limit: 0
		}
	}
	pub fn print(&self, pre: &str) {
		let status = match self.status {
			SatelliteStatus::ACTIVE => "ACTIVE".green(),
			SatelliteStatus::INACTIVE => "ACTIVE".red(),
			SatelliteStatus::SLEEP => "ACTIVE".yellow(),
		};
		println!("{} - {}:", self.name, status);
		println!("{}OS - {}", pre, self.os);
		print!("{}Debug mode - ", pre);
		match self.debug_mode {
			true => println!("ENABLED"),
			false => println!("DISABLED"),
		}
		println!("{}Password length - {}", pre, self.password.len());
		println!("{}Connection limit - {}", pre, self.connection_limit);
	}
}


#[derive(Debug)]
pub struct Config {
    pub satellites: Vec<Satellite>,
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
			satellites: match config_toml.satellites {
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
		})
	}
}
