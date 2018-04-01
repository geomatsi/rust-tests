extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("make config error: {}", err);
        process::exit(1);
    });
    println!("config '{}'", config);

    if let Err(e) = minigrep::run(config) {
        println!("run config error: {}", e);
        process::exit(1);
    };
}
