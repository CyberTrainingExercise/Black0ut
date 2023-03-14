use crate::config::{Config};
use crate::cli::{CLI};

mod config;
mod cli;

fn main() {
    let config = Config::new();

    if config.is_err() {
        print!("Err: {:#?}", config);
        return;
    }

    let config = config.unwrap();

    let cli = CLI::new(config);

    let res = cli.run();
    match res {
        Ok(_) => println!("Done"),
        Err(err) => println!("Err {}", err.as_ref()),
    }
}
