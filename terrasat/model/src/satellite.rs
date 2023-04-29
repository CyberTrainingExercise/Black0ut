use colored::{self, Colorize};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SatelliteStatus {
	ACTIVE,
	INACTIVE,
	SLEEP,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Satellite {
	pub name: String,
	pub status: SatelliteStatus,
	pub version: String,
    pub os: String,
    pub debug_mode: bool,
    pub password: String,
    pub shutdown_code: Option<usize>,
	pub has_pulse: bool,
}


impl Satellite {
	pub fn empty() -> Self {
		Satellite {
			name: String::from("Unknown"),
			version: String::from("Unknown"),
			status: SatelliteStatus::INACTIVE,
			os: String::from("Unknown"),
			debug_mode: false,
			password: String::from("Unknown"),
			shutdown_code: None,
			has_pulse: false,
		}
	}
	pub fn print_short(&self) {
		let status = match self.status {
			SatelliteStatus::ACTIVE => "ACTIVE".green(),
			SatelliteStatus::INACTIVE => "INACTIVE".red(),
			SatelliteStatus::SLEEP => "SLEEP".blue(),
		};
		println!("{} - {}", self.name, status);
	}
	pub fn print_long(&self, pre: &str) {
		self.print_short();
		println!("{}OS - {}", pre, self.os);
		println!("{}Version - {}", pre, self.version);
		print!("{}Debug mode - ", pre);
		match self.debug_mode {
			true => println!("ENABLED"),
			false => println!("DISABLED"),
		}
		println!("{}Password length - {}", pre, self.password.len());
	}
}