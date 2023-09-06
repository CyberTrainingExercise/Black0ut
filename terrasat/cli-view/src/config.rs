use std::fmt::{Formatter, self, Display};
use std::{fs};
use std::io::Error as IoError;
use serde::{Serialize, Deserialize};
use toml;
use std::collections::HashMap;

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
	cli: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CLIToml {
	server_host: String,
}

#[derive(Debug)]
pub struct Config {
    pub server_host: String,
}

impl Config {
    pub fn new() -> Result<Self, ConfigParseError> {
		let config_filepaths: [&str; 4] = [
			"./cli-config.toml",
			"./config.toml",
			"./src/cli-config.toml",
			"./src/config.toml",
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
				cli: None,
			}
		});

		Ok(Config
		{
			server_host: match config_toml.cli {
				Some(params) => {
					let mut ret = "".to_owned();
					for (name, val) in params {
						if name == "server_host" {
							ret = val;
						}
					}
					ret
				}
				None => "localhost:8000".to_owned(),
			},
		})
	}
}
