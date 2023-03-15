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

    cli.print_startup();
    loop {
        let res = cli.run();
        match res {
            Ok(should_stop) => {
                if should_stop {
                    break;
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
