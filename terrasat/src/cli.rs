use std::io::{stdin,stdout,Write};
use crate::config::{Config};

const CMD_HELP: &str = "help";
const CMD_LIST: &str = "list";
const CMD_INFO: &str = "info";
const CMD_EXIT: &str = "exit";
const CMD_SLEEP: &str = "sleep";
const CMD_WAKE: &str = "wake";
const CMD_PLAN: &str = "plan";

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

    fn print_help(&self) {
        println!("List of commands:");
        println!("\t{}\t\t\t -- display this output", CMD_HELP);
        println!("\t{}\t\t\t -- list all satellites and ground terminals", CMD_LIST);
        println!("\t{} [sat]\t\t -- get info for a satellite or ground terminal", CMD_INFO);
        println!("\t{} [sat] [x]\t\t -- force sleep a satellite for x hours", CMD_SLEEP);
        println!("\t{} [sat]\t\t -- force wakeup a sleeping satellite", CMD_SLEEP);
        println!("\t{} [sat] [filename]\t -- set a satellite's mission plan to filename", CMD_PLAN);
        println!("\t{}\t\t\t -- exit this application", CMD_EXIT);
    }

    fn print_startup(&self) {
        println!("Welcome to the Terrasat Network!
        
        Please type 'help' for list of commands.
        ")
    }

    fn print_list(&self) {
        for sat in &self.config.satellites {
            sat.print("\t\t");
        }
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
            println!("Echo: {}", input);

            // Break into tokens
            let tokens: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();

            if tokens.is_empty() {
                println!("Err: input invalid, please try again!");
            }

            // Match on tokens
            match tokens[0].as_str() {
                CMD_HELP => {
                    self.print_help();
                }
                CMD_EXIT => {
                    println!("Closing...");
                    return;
                }
                CMD_LIST => {
                    self.print_list();
                }
                _ => {
                    print!("Err: input {} invalid", tokens[0]);
                }
            }
        }
    }
}