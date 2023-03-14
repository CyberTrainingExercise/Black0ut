use std::io::{stdin,stdout,Write};
use crate::config::{Config};

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
        println!("List of commands:
        help                        -- display this output
        list                        -- list all satellites and ground terminals
        info [x]                    -- get info for a satellite or ground terminal
        shutdown [sat] [x]          -- force shutdown a satellite for x hours
        plan [sat] [filename]       -- set a satellite's mission plan to filename
        exit                        -- exit this application
        ")
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
            println!("Echo: {}", input);

            // Break into tokens
            let tokens: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();

            if tokens.is_empty() {
                println!("Err: input invalid, please try again!");
            }

            // Match on tokens
            match tokens[0].as_str() {
                "help" => {
                    self.print_help();
                }
                "exit" => {
                    println!("Closing...");
                    return;
                }
                _ => {
                    print!("Err: input {} invalid", tokens[0]);
                }
            }
        }
    }
}