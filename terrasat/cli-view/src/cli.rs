use std::{io::{stdin,stdout,Write}, str::FromStr, fmt::{Display, Formatter, self}};
use strum::IntoEnumIterator;
use strum_macros::{EnumString, Display, EnumIter};
use colored::{self, Colorize};
use std::{thread, time::Duration};
use std::path::Path;

use crate::config::{Config};
use model::satellite::{Satellite, SatelliteStatus};

#[derive(Debug, Clone)]
pub struct CLIError(String);
impl Display for CLIError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
impl PartialEq for CLIError {
    fn eq(&self, _othr: &CLIError) -> bool {
        true
    }
}
impl Eq for CLIError {}

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
    Login,
    Shutdown,
    Dos,
    Loop,
}

#[derive(Debug)]
pub struct CLI {
	config: Config,
    password: Option<(usize, String)>,
}

impl CLI {
    pub fn new(config: Config) -> Self {
        CLI {
            config: config,
            password: None
        }
    }

    fn get_command_details(cmd: Command) -> String {
        let mut ret = match cmd {
            Command::Help => "\t\t\t -- display this output".to_owned(),
            Command::List => "\t\t\t -- list all satellites and ground terminals".to_owned(),
            Command::Info => " [sat]\t\t -- get info for a satellite or ground terminal".to_owned(),
            Command::Sleep => " [sat]\t\t -- force sleep a satellite for".to_owned(),
            Command::Wake => " [sat]\t\t -- force wakeup a sleeping satellite".to_owned(),
            Command::Plan => " [sat] [filename]\t -- set a satellite's mission plan to filename".to_owned(),
            Command::Exit => "\t\t\t -- exit this application".to_owned(),
            Command::Exec => " [sat] [filename]\t -- exec a python script on a remote satellite system".to_owned(),
            Command::Login => " [sat] [password]\t -- login to a satellite to perform admin commands".to_owned(),
            Command::Shutdown => " -- unknown operation".to_owned(),
            Command::Dos => " -- unknown operation".to_owned(),
            Command::Loop => " -- unknown operation".to_owned(),
        };
        if CLI::is_admin_command(cmd) {
            ret += &"(ADMIN ONLY)".green().to_string();
        }
        if CLI::is_debug_command(cmd) {
            ret += &"(DEBUG ONLY)".yellow().to_string();
        }
        ret
    }

    fn get_command_arguments(cmd: Command) -> usize {
        match cmd {
            Command::Help => 0,
            Command::List => 0,
            Command::Info => 1,
            Command::Plan => 2,
            Command::Sleep => 1,
            Command::Wake => 1,
            Command::Exit => 0,
            Command::Exec => 2,
            Command::Login => 2,
            Command::Shutdown => 2,
            Command::Dos => 1,
            Command::Loop => 0,
        }
    }

    fn is_admin_command(cmd: Command) -> bool {
        match cmd {
            Command::Help => false,
            Command::List => false,
            Command::Info => false,
            Command::Plan => true,
            Command::Sleep => false,
            Command::Wake => false,
            Command::Exit => false,
            Command::Exec => false,
            Command::Login => false,
            Command::Shutdown => false,
            Command::Dos => false,
            Command::Loop => false,
        }
    }

    fn is_debug_command(cmd: Command) -> bool {
        match cmd {
            Command::Help => false,
            Command::List => false,
            Command::Info => false,
            Command::Plan => false,
            Command::Sleep => false,
            Command::Wake => false,
            Command::Exit => false,
            Command::Exec => true,
            Command::Login => false,
            Command::Shutdown => false,
            Command::Dos => false,
            Command::Loop => false,
        }
    }

    fn is_hidden_command(cmd: Command) -> bool {
        match cmd {
            Command::Help => false,
            Command::List => false,
            Command::Info => false,
            Command::Plan => false,
            Command::Sleep => false,
            Command::Wake => false,
            Command::Exit => false,
            Command::Exec => false,
            Command::Login => false,
            Command::Shutdown => true,
            Command::Dos => true,
            Command::Loop => true,
        }
    }

    fn parse_code(str: String) -> Result<usize, CLIError> {
        let code = str.parse::<usize>();
        if code.is_err() {
            return Err(CLIError(format!("Cannot parse '{}' as code. Code must be an integer.", str)));
        }
        Ok(code.unwrap())
    }

    fn parse_sat_index(sats_len: usize, str: String) -> Result<usize, CLIError> {
        let index = str.parse::<usize>();
        if index.is_err() || index.as_ref().unwrap() >= &sats_len {
            return Err(CLIError(format!("Cannot parse '{}' as index. Index must be an integer 0 < x < {}",
                       str, sats_len)));
        }
        let index = index.unwrap();
        Ok(index)
    }

    fn parse_cmd(str: String) -> Result<Command, CLIError> {
        let str: &str = &str.to_lowercase();
        let str = str[0..1].to_uppercase() + &str[1..];
        let res = Command::from_str(&str);
        if res.is_err() {
            return Err(CLIError(format!("Cannot parse command '{}'. Enter 'help' for list of valid commands", str)));
        }
        return Ok(res.unwrap());
    }

    fn parse_sat(text: String) -> Result<Satellite, CLIError> {
        match serde_json::from_str(&text) {
            Ok(sat) => Ok(sat),
            Err(err) => {
                println!("Uh oh, {}", err);
                Err(CLIError(format!("{}", err)))
            }
        }
    }

    fn parse_sats(text: String) -> Result<Vec<Satellite>, CLIError> {
        match serde_json::from_str(&text) {
            Ok(sat) => Ok(sat),
            Err(err) => {
                println!("Uh oh, {}", err);
                Err(CLIError(format!("{}", err)))
            }
        }
    }

    pub fn print_startup(&self) {
        println!("Welcome to the Terrasat Operator Command and Control Application (TOCCA).
        
        Please type 'help' for list of commands.
        ")
    }

    fn get_sat_len(&self) -> Result<usize, CLIError> {
        let text = self.send_request("/count".to_owned())?;
        let index = text.parse::<usize>();
        match index {
            Ok(len) => Ok(len),
            Err(err) => Err(CLIError(format!("{}", err))),
        }
    }

    fn send_request(&self, route: String) -> Result<String, CLIError> {
        let resp = reqwest::blocking::get(
            format!("{}/{}", self.config.server_host, route));
        match resp {
            Ok(result) => {
                let res = result.text();
                match res {
                    Ok(text) => return Ok(text),
                    Err(err) => return Err(CLIError(format!("{}", err))), 
                }
            },
            Err(err) => {
                return Err(CLIError(format!("{}", err)));
            }
        }
    }

    // Return bool, true = stop running. False = continue running.
    pub fn run(&mut self) -> Result<bool, CLIError> {
        let mut input=String::new();
        // Read input
        if self.password.is_some() {
            let str = format!("{} > ", self.password.as_ref().unwrap().0).green();
            print!("{}", str);
        } else {
            print!("> ");
        }
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
            return Ok(false);
        }

        // Match on tokens
        let res = CLI::parse_cmd(tokens[0].to_string());
        let cmd: Command;
        match res {
            Ok(command) => cmd = command,
            Err(message) => {
                println!("{}", message);
                return Ok(false);
            }
        }
        if CLI::get_command_arguments(cmd) != tokens.len() - 1 {
            println!("Please enter {} arguments for command {}. See 'help' for more details.",
                    CLI::get_command_arguments(cmd),
                    cmd.to_string()
            );
            return Ok(false);
        }
        match cmd {
            Command::Help => {
                println!("Commands:");
                for cmd in Command::iter() {
                    if !CLI::is_admin_command(cmd) || self.password.is_some() {
                        if CLI::is_hidden_command(cmd) {
                            continue;
                        }
                        println!("\t{}{}", cmd.to_string(), CLI::get_command_details(cmd));
                    }
                }
            },
            Command::List => {
                let text = self.send_request(format!("all"))?;
                let sats = CLI::parse_sats(text)?;
                for sat in sats {
                    sat.print_short();
                }
            }
            Command::Info => {
                let len = self.get_sat_len()?;
                let index = CLI::parse_sat_index(len, tokens[1].to_string())?;
                let text = self.send_request(format!("status/{}", index))?;
                let sat = CLI::parse_sat(text)?;
                sat.print_long("\t");
            }
            Command::Plan => {
                if Path::new(&tokens[2]).exists() {
                    let len = self.get_sat_len()?;
                    let index = CLI::parse_sat_index(len, tokens[1].to_string())?;
                    let text = self.send_request(format!("status/{}", index))?;
                    let sat = CLI::parse_sat(text)?;
                    if sat.status == SatelliteStatus::ACTIVE {
                        println!("Sending plan file...");
                        thread::sleep(Duration::from_millis(1000));
                        println!("Plan file sent");
                    } else {
                        println!("Cannot plan a satellite with status: {:#?}", sat.status);
                    }
                } else {
                    println!("Error: file {} does not exist!", &tokens[2])
                }
            }
            Command::Sleep => {
                let len = self.get_sat_len()?;
                let index = CLI::parse_sat_index(len, tokens[1].to_string())?;
                let text = self.send_request(format!("sleep/{}", index))?;
                println!("{}", text);
            }
            Command::Wake => {
                let len = self.get_sat_len()?;
                let index = CLI::parse_sat_index(len, tokens[1].to_string())?;
                let text = self.send_request(format!("wake/{}", index))?;
                println!("{}", text);
            }
            Command::Exit => {
                println!("Closing...");
                return Ok(true);
            }
            Command::Exec => {
                let index = CLI::parse_sat_index(3, tokens[1].to_string())?;
                let text = self.send_request(format!("status/{}", index))?;
                println!("{}", text);
            }
            Command::Login => {
                let len = self.get_sat_len()?;
                let index = CLI::parse_sat_index(len, tokens[1].to_string())?;
                let text = self.send_request(format!("login/{}/{}", index, tokens[2].to_string()))?;
                if text.contains("True") {
                    if self.password.is_some() && self.password.as_ref().unwrap().0 != index {
                        println!("Logged out of Sat{} as admin.", self.password.as_ref().unwrap().0);
                    }
                    self.password = Some((index, tokens[2].to_string()));
                    println!("Logged in to Sat{} as admin.", index);
                } else {
                    println!("Password is incorrect.");
                }
            }
            Command::Shutdown => {
                let len = self.get_sat_len()?;
                let index = CLI::parse_sat_index(len, tokens[1].to_string())?;
                let code = CLI::parse_code(tokens[2].to_string())?;
                let text = self.send_request(format!("shutdown/{}/{}", index, code))?;
                println!("{}", text);
            }
            Command::Dos => {
                let code = CLI::parse_code(tokens[1].to_string())?;
                let text = self.send_request(format!("dos/{}", code))?;
                println!("{}", text);
            }
            Command::Loop => {
                loop {
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    let text = self.send_request(format!("all"))?;
                    let sats = CLI::parse_sats(text)?;
                    for sat in sats {
                        sat.print_short();
                    }
                    thread::sleep(Duration::from_millis(1000));
                }
            }
        }
        return Ok(false);
    }
}