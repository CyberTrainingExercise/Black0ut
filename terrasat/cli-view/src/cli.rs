use std::{io::{stdin,stdout,Write}, str::FromStr, collections::HashMap, fmt::format};
use reqwest::Response;
use strum::IntoEnumIterator;
use strum_macros::{EnumString, Display, EnumIter};
use crate::config::{Config};
use model::satellite::{Satellite, SatelliteStatus};

#[derive(Debug, Copy, Clone, PartialEq, EnumString, Display, EnumIter)]
enum Command {
    Help,
    List,
    Info,
    Exit,
    Sleep,
    Wake,
    Plan,
    Exec,
}

#[derive(Debug)]
pub struct CLI {
	config: Config,
}

impl CLI {
    pub fn new(config: Config) -> Self {
        CLI {
            config: config
        }
    }

    fn get_command_details(cmd: Command) -> String {
        match cmd {
            Command::Help => "\t\t\t -- display this output".to_owned(),
            Command::List => "\t\t\t -- list all satellites and ground terminals".to_owned(),
            Command::Info => " [sat]\t\t -- get info for a satellite or ground terminal".to_owned(),
            Command::Plan => " [sat] [x]\t\t -- force sleep a satellite for x hours".to_owned(),
            Command::Sleep => " [sat]\t\t -- force wakeup a sleeping satellite".to_owned(),
            Command::Wake => " [sat] [filename]\t -- set a satellite's mission plan to filename".to_owned(),
            Command::Exit => "\t\t\t -- exit this application".to_owned(),
            Command::Exec => " [sat] [filename]\t -- (DEBUG MODE ONLY) exec a python script on a remote satellite system".to_owned(),
        }
    }

    fn get_command_arguments(cmd: Command) -> usize {
        match cmd {
            Command::Help => 0,
            Command::List => 0,
            Command::Info => 1,
            Command::Plan => 2,
            Command::Sleep => 2,
            Command::Wake => 1,
            Command::Exit => 0,
            Command::Exec => 2,
        }
    }

    fn parse_sat_index(sats_len: usize, str: String) -> Result<usize, String> {
        let index = str.parse::<usize>();
        if index.is_err() || index.as_ref().unwrap() >= &sats_len {
            return Err(format!("Cannot parse '{}' as index. Index must be an integer 0 < x < {}",
                       str, sats_len));
        }
        let index = index.unwrap();
        Ok(index)
    }

    // fn parse_sat(sats_len: usize, str: String) -> Result<&Satellite, String> {
    //     match self.parse_sat_index(str) {
    //         Ok(index) => Ok(&self.config.satellites[index]),
    //         Err(res) => Err(res),
    //     }
    // }

    fn parse_cmd(str: String) -> Result<Command, String> {
        let str: &str = &str.to_lowercase();
        let str = str[0..1].to_uppercase() + &str[1..];
        let res = Command::from_str(&str);
        if res.is_err() {
            return Err(format!("Cannot parse command '{}'. Enter 'help' for list of valid commands", str));
        }
        return Ok(res.unwrap());
    }

    fn print_startup(&self) {
        println!("Welcome to the Terrasat Network!
        
        Please type 'help' for list of commands.
        ")
    }

    pub fn run(&self) {
        self.print_startup();
        loop {
            let mut input=String::new();
            // Read input
            print!("> ");
            let _ = stdout().flush();
            stdin().read_line(&mut input).expect("Err: input invalid!");
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }

            // Break into tokens
            let tokens: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();

            if tokens.is_empty() {
                continue;
            }

            // Match on tokens
            let res = CLI::parse_cmd(tokens[0].to_string());
            let cmd: Command;
            match res {
                Ok(command) => cmd = command,
                Err(message) => {
                    println!("{}", message);
                    continue;
                }
            }
            if CLI::get_command_arguments(cmd) != tokens.len() - 1 {
                println!("Please enter {} arguments for command {}. See 'help' for more details.",
                        CLI::get_command_arguments(cmd),
                        cmd.to_string()
                );
                continue;
            }
            match cmd {
                Command::Help => {
                    println!("Commands:");
                    for cmd in Command::iter() {
                        println!("\t{}{}", cmd.to_string(), CLI::get_command_details(cmd));
                    }
                },
                Command::List => {
                    // for sat in &self.config.satellites {
                    //     sat.print_short();
                    // }
                    println!("UNIMPLMENTED!");
                }
                Command::Info => {
                    // let res = self.parse_sat(tokens[1].to_string());
                    // match res {
                    //     Ok(sat) => sat.print_long("\t"),
                    //     Err(message) => println!("{}", message),
                    // }
                    println!("UNIMPLMENTED!");
                }
                Command::Plan => {
                    println!("UNIMPLEMENTED!");
                }
                Command::Sleep => {
                    println!("UNIMPLEMENTED!");
                }
                Command::Wake => {
                    println!("UNIMPLEMENTED!");
                }
                Command::Exit => {
                    println!("Closing...");
                    return;
                }
                Command::Exec => {
                    let index: usize;
                    match CLI::parse_sat_index(3, tokens[1].to_string()) {
                        Ok(i) => index = i,
                        Err(err) => {
                            println!("{}", err);
                            continue;
                        }
                    }
                    let resp = reqwest::blocking::get(
                        format!("{}/status/{}", self.config.server_host, index));
                    match resp {
                        Ok(result) => {
                            println!("{}", result.text().unwrap());
                        },
                        Err(err) => println!("{}", err),
                    }
                }
            }
        }
    }
}