use crate::config::Config;

mod config;

fn main() {
    println!("Hello, world!");

    let config = Config::new();

    println!("{:#?}", config);
}
